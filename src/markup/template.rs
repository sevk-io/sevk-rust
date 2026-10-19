use regex::Regex;
use serde_json::{Map, Value};
use std::collections::HashMap;

use super::checked_replace::replace_all_checked;
use super::error::RenderError;
use super::limits::{render_error, TOO_COMPLEX};
use super::number::js_number;

const MAX_IF_PASSES: usize = 2000;
const MAX_EACH_ITEMS: usize = 1000;
const MAX_EACH_OUTPUT: usize = 2_000_000;
const MAX_TEMPLATE_WORK: usize = 20_000_000;
const OUTPUT_EXCEEDED: &str = "Email block exceeds the maximum template output size";

#[derive(Default)]
struct OutputMeter {
    rendered: usize,
}

impl OutputMeter {
    fn charge(&mut self, value: String) -> Result<String, RenderError> {
        if self.rendered + value.len() > MAX_EACH_OUTPUT {
            return Err(render_error(OUTPUT_EXCEEDED));
        }
        self.rendered += value.len();
        Ok(value)
    }

    fn charge_item(&self, item_out: &mut usize, value: String) -> Result<String, RenderError> {
        if self.rendered + *item_out + value.len() > MAX_EACH_OUTPUT {
            return Err(render_error(OUTPUT_EXCEEDED));
        }
        *item_out += value.len();
        Ok(value)
    }

    fn finish_item(&mut self, length: usize) -> Result<(), RenderError> {
        self.rendered += length;
        if self.rendered > MAX_EACH_OUTPUT {
            return Err(render_error(OUTPUT_EXCEEDED));
        }
        Ok(())
    }
}

fn is_truthy(val: &Value) -> bool {
    match val {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().map(|f| f != 0.0).unwrap_or(true),
        Value::String(s) => !s.is_empty(),
        Value::Array(a) => !a.is_empty(),
        Value::Object(_) => true,
    }
}

fn template_text(value: Option<&Value>) -> String {
    match value {
        None | Some(Value::Null) => String::new(),
        Some(Value::Bool(b)) => b.to_string(),
        Some(Value::Number(n)) => js_number(n.as_f64().unwrap_or(0.0)),
        Some(Value::String(s)) => s.clone(),
        Some(Value::Array(items)) => items
            .iter()
            .map(|item| match item {
                Value::Object(_) => "[object Object]".to_string(),
                other => template_text(Some(other)),
            })
            .collect::<Vec<_>>()
            .join(","),
        Some(Value::Object(_)) => "[object Object]".to_string(),
    }
}

fn evaluate_condition(expr: &str, config: &Map<String, Value>) -> bool {
    let trimmed = expr.trim();

    if trimmed.contains("||") {
        return trimmed
            .split("||")
            .any(|part| evaluate_condition(part, config));
    }

    if trimmed.contains("&&") {
        return trimmed
            .split("&&")
            .all(|part| evaluate_condition(part, config));
    }

    let eq_re = regex!(r#"^(\w+)\s*==\s*"([^"]*)"$"#);
    if let Some(caps) = eq_re.captures(trimmed) {
        return template_text(config.get(&caps[1])) == caps[2];
    }

    let neq_re = regex!(r#"^(\w+)\s*!=\s*"([^"]*)"$"#);
    if let Some(caps) = neq_re.captures(trimmed) {
        return template_text(config.get(&caps[1])) != caps[2];
    }

    config.get(trimmed).map(is_truthy).unwrap_or(false)
}

fn render_template(template: &str, config: &Map<String, Value>) -> Result<String, RenderError> {
    let mut meter = OutputMeter::default();
    let result = render_conditionals(template, config)?;
    let result = render_loops(&result, config, &mut meter)?;

    let fallback_re = regex!(r"\{%(\w+)\s*\?\?([^%]+)%\}");
    let result = replace_all_checked(fallback_re, &result, |caps| {
        let value = match config.get(&caps[1]) {
            None | Some(Value::Null) => caps[2].trim().to_string(),
            Some(Value::String(s)) if s.is_empty() => caps[2].trim().to_string(),
            Some(value) => template_text(Some(value)),
        };
        meter.charge(value)
    })?;

    let simple_re = regex!(r"\{%(\w+)%\}");
    replace_all_checked(simple_re, &result, |caps| {
        meter.charge(template_text(config.get(&caps[1])))
    })
}

fn render_conditionals(template: &str, config: &Map<String, Value>) -> Result<String, RenderError> {
    let open_re = regex!(r"\{%#if\s+([^%]+)%\}");
    let mut result = template.to_string();
    let mut work = 0;

    for _ in 0..MAX_IF_PASSES {
        work += result.len();
        if work > MAX_TEMPLATE_WORK {
            return Err(render_error(TOO_COMPLEX));
        }

        let openings: Vec<(usize, usize, String)> = open_re
            .captures_iter(&result)
            .map(|caps| {
                let whole = caps.get(0).unwrap();
                (whole.start(), whole.end(), caps[1].to_string())
            })
            .collect();
        if openings.is_empty() {
            break;
        }

        let mut search_from = 0;
        let mut replaced = false;
        while let Some(found) = result[search_from..].find("{%/if%}") {
            let close_index = search_from + found;
            let opening = openings
                .iter()
                .take_while(|(_, end, _)| *end <= close_index)
                .last();
            let Some((open_start, open_end, condition)) = opening else {
                search_from = close_index + "{%/if%}".len();
                continue;
            };

            let body = &result[*open_end..close_index];
            let is_true = evaluate_condition(condition, config);
            let replacement = match body.find("{%else%}") {
                Some(else_index) if is_true => body[..else_index].to_string(),
                Some(else_index) => body[else_index + "{%else%}".len()..].to_string(),
                None if is_true => body.to_string(),
                None => String::new(),
            };
            result = format!(
                "{}{}{}",
                &result[..*open_start],
                replacement,
                &result[close_index + "{%/if%}".len()..]
            );
            replaced = true;
            break;
        }

        if !replaced {
            break;
        }
    }

    Ok(result)
}

fn render_loops(
    template: &str,
    config: &Map<String, Value>,
    meter: &mut OutputMeter,
) -> Result<String, RenderError> {
    if !template.contains("{%/each%}") {
        return Ok(template.to_string());
    }

    let open_re = regex!(r"\{%#each\s+(\w+)(?:\s+as\s+(\w+))?%\}");
    let mut output = String::new();
    let mut from = 0;
    let mut work = 0;

    while from <= template.len() {
        let Some(caps) = open_re.captures(&template[from..]) else {
            output.push_str(&template[from..]);
            break;
        };
        let whole = caps.get(0).unwrap();
        let body_start = from + whole.end();
        let Some(found) = template[body_start..].find("{%/each%}") else {
            output.push_str(&template[from..]);
            break;
        };
        let close_index = body_start + found;

        output.push_str(&template[from..from + whole.start()]);
        if let Some(Value::Array(items)) = config.get(&caps[1]) {
            if !items.is_empty() {
                if items.len() > MAX_EACH_ITEMS {
                    return Err(render_error(format!(
                        "Email block exceeds the maximum of {MAX_EACH_ITEMS} loop items"
                    )));
                }

                let alias = caps.get(2).map(|m| m.as_str()).unwrap_or("this");
                let property_re =
                    Regex::new(&format!(r"\{{%{}\.(\w+)%\}}", regex::escape(alias))).unwrap();
                let body = &template[body_start..close_index];

                for item in items {
                    work += body.len() + 16;
                    if work > MAX_TEMPLATE_WORK {
                        return Err(render_error(TOO_COMPLEX));
                    }
                    output.push_str(&render_loop_item(body, item, config, &property_re, meter)?);
                }
            }
        }

        from = close_index + "{%/each%}".len();
    }

    Ok(output)
}

fn render_loop_item(
    body: &str,
    item: &Value,
    config: &Map<String, Value>,
    property_re: &Regex,
    meter: &mut OutputMeter,
) -> Result<String, RenderError> {
    let mut item_out = 0;

    let rendered = replace_all_checked(property_re, body, |item_caps| {
        let value = match item {
            Value::Object(fields) => template_text(fields.get(&item_caps[1])),
            Value::String(text) if &item_caps[1] == "length" => {
                text.encode_utf16().count().to_string()
            }
            Value::Array(items) if &item_caps[1] == "length" => items.len().to_string(),
            _ => String::new(),
        };
        meter.charge_item(&mut item_out, value)
    })?;
    let simple_re = regex!(r"\{%(\w+)%\}");
    let rendered = replace_all_checked(simple_re, &rendered, |caps| {
        meter.charge_item(&mut item_out, template_text(config.get(&caps[1])))
    })?;

    meter.finish_item(rendered.len())?;
    Ok(rendered)
}

pub(super) fn process_block_tag(
    attrs: &HashMap<String, String>,
    inner: &str,
) -> Result<String, RenderError> {
    let template = if inner.trim().is_empty() {
        attrs.get("template").cloned().unwrap_or_default()
    } else {
        inner.trim().to_string()
    };
    if template.is_empty() {
        return Ok(String::new());
    }
    let raw_config = attrs.get("config").map(String::as_str).unwrap_or("{}");
    let config_str = regex!(r"(?i)&(?:amp|quot|apos|lt|gt|#39|#x27|#34|#x22);").replace_all(
        raw_config,
        |captures: &regex::Captures| {
            match captures[0].to_ascii_lowercase().as_str() {
                "&amp;" => "&",
                "&quot;" | "&#34;" | "&#x22;" => "\"",
                "&apos;" | "&#39;" | "&#x27;" => "'",
                "&lt;" => "<",
                "&gt;" => ">",
                _ => &captures[0],
            }
            .to_owned()
        },
    );
    let config: Map<String, Value> = serde_json::from_str(&config_str)
        .or_else(|_| serde_json::from_str(&config_str.replace('\'', "\"")))
        .unwrap_or_default();
    render_template(&template, &config)
}
