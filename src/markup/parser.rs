use regex::Regex;
use std::collections::HashMap;

use super::error::RenderError;
use super::limits::{render_error, RenderBudget, MAX_RENDER_OUTPUT, MAX_TAG_ITERATIONS};
use super::styles::parse_attributes;

pub(super) fn process_tag<F>(
    content: &str,
    tag_name: &str,
    mut processor: F,
    budget: &mut RenderBudget,
) -> Result<String, RenderError>
where
    F: FnMut(HashMap<String, String>, &str, &mut RenderBudget) -> Result<String, RenderError>,
{
    let mut result = content.to_string();
    let open_re = Regex::new(&format!(r"(?i)<{tag_name}([^>]*)>")).unwrap();
    let close_re = Regex::new(&format!("(?i)</{tag_name}>")).unwrap();
    let nested_open_re = Regex::new(&format!("(?i)<{tag_name}")).unwrap();

    for _ in 0..MAX_TAG_ITERATIONS {
        budget.charge_scan_work(result.len())?;
        let mut replacements = Vec::new();
        let openings: Vec<(usize, usize, String)> = open_re
            .captures_iter(&result)
            .map(|capture| {
                let whole = capture.get(0).unwrap();
                (
                    whole.start(),
                    whole.end(),
                    capture.get(1).map(|m| m.as_str()).unwrap_or("").to_string(),
                )
            })
            .collect();
        if openings.is_empty() {
            break;
        }
        let mut consumed_until = 0;

        for (start, attrs_end, attrs_source) in openings {
            if start < consumed_until {
                continue;
            }

            let remaining = &result[attrs_end..];
            let Some(close_match) = close_re.find(remaining) else {
                budget.charge_scan_work(remaining.len())?;
                continue;
            };
            budget.charge_scan_work(close_match.start())?;

            let inner = &remaining[..close_match.start()];
            if nested_open_re.is_match(inner) {
                continue;
            }

            let attrs = parse_attributes(&attrs_source);
            let end = attrs_end + close_match.end();
            replacements.push((start, end, processor(attrs, inner, budget)?));
            consumed_until = end;
        }

        if replacements.is_empty() {
            break;
        }

        let mut rebuilt = String::new();
        let mut cursor = 0;
        for (start, end, replacement) in replacements {
            rebuilt.push_str(&result[cursor..start]);
            rebuilt.push_str(&replacement);
            cursor = end;
            if rebuilt.len() > MAX_RENDER_OUTPUT {
                return Err(render_error(format!(
                    "Email markup expands beyond the maximum size of {MAX_RENDER_OUTPUT} bytes"
                )));
            }
        }
        rebuilt.push_str(&result[cursor..]);
        if rebuilt == result {
            break;
        }
        result = rebuilt;
    }

    Ok(result)
}
