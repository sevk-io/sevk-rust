use std::collections::HashMap;

use super::styles::{extract_all_style_attributes, parse_attributes, style_to_string};

fn style_with_default_margin(attrs: &HashMap<String, String>) -> String {
    let mut style = extract_all_style_attributes(attrs);
    style
        .entry("margin".to_string())
        .or_insert_with(|| "0".to_string());
    style_to_string(&style)
}

pub(super) fn process_heading(attrs: &HashMap<String, String>, inner: &str) -> String {
    let level = attrs
        .get("level")
        .map(String::as_str)
        .filter(|level| !level.is_empty())
        .unwrap_or("1");
    format!(
        r#"<h{} style="{}">{}</h{}>"#,
        level,
        style_with_default_margin(attrs),
        inner,
        level
    )
}

pub(super) fn process_paragraph(attrs: &HashMap<String, String>, inner: &str) -> String {
    format!(
        r#"<p style="{}">{}</p>"#,
        style_with_default_margin(attrs),
        inner
    )
}

pub(super) fn process_text(attrs: &HashMap<String, String>, inner: &str) -> String {
    let style = extract_all_style_attributes(attrs);
    format!(
        r#"<span style="{}">{}</span>"#,
        style_to_string(&style),
        inner
    )
}

pub(super) fn process_link(attrs: &HashMap<String, String>, inner: &str) -> String {
    let href = attrs
        .get("href")
        .filter(|value| !value.is_empty())
        .map(String::as_str)
        .unwrap_or("#");
    let target = attrs
        .get("target")
        .filter(|value| !value.is_empty())
        .map(String::as_str)
        .unwrap_or("_blank");
    let style = extract_all_style_attributes(attrs);
    format!(
        r#"<a href="{}" target="{}" style="{}">{}</a>"#,
        href,
        target,
        style_to_string(&style),
        inner
    )
}

pub(super) fn process_list(attrs: &HashMap<String, String>, inner: &str) -> String {
    let tag = match attrs.get("type").map(String::as_str) {
        Some("ordered") => "ol",
        _ => "ul",
    };
    let mut style = extract_all_style_attributes(attrs);
    style
        .entry("margin".to_string())
        .or_insert_with(|| "0".to_string());
    if let Some(list_style) = attrs.get("list-style-type") {
        style.insert("list-style-type".to_string(), list_style.clone());
    }

    format!(
        r#"<{} style="{}"{}>{}</{}>"#,
        tag,
        style_to_string(&style),
        class_attribute(attrs),
        inner,
        tag
    )
}

pub(super) fn process_list_item(attrs: &HashMap<String, String>, inner: &str) -> String {
    let style = extract_all_style_attributes(attrs);
    format!(
        r#"<li style="{}"{}>{}</li>"#,
        style_to_string(&style),
        class_attribute(attrs),
        inner
    )
}

pub(super) fn process_images(content: &str) -> String {
    let image_re = regex!(r"(?i)<image([^>]*?)/?>");
    let result = image_re
        .replace_all(content, |captures: &regex::Captures| {
            let attrs = parse_attributes(&captures[1]);
            let src = attrs.get("src").map(String::as_str).unwrap_or("");
            let alt = attrs.get("alt").map(String::as_str).unwrap_or("");
            let mut style = extract_all_style_attributes(&attrs);
            style
                .entry("vertical-align".to_string())
                .or_insert_with(|| "middle".to_string());
            style
                .entry("max-width".to_string())
                .or_insert_with(|| "100%".to_string());

            let width = dimension_attribute("width", attrs.get("width"));
            let height = dimension_attribute("height", attrs.get("height"));
            format!(
                r#"<img src="{}" alt="{}"{}{} style="{}" />"#,
                src,
                alt,
                width,
                height,
                style_to_string(&style)
            )
        })
        .to_string();

    regex!(r"(?i)</image>").replace_all(&result, "").to_string()
}

pub(super) fn process_dividers(content: &str) -> String {
    let divider_re = regex!(r"(?i)<divider([^>]*?)/?>");
    let result = divider_re
        .replace_all(content, |captures: &regex::Captures| {
            let attrs = parse_attributes(&captures[1]);
            let style = extract_all_style_attributes(&attrs);
            format!(
                r#"<hr style="{}"{} />"#,
                style_to_string(&style),
                class_attribute(&attrs)
            )
        })
        .to_string();

    regex!(r"(?i)</divider>")
        .replace_all(&result, "")
        .to_string()
}

fn dimension_attribute(name: &str, value: Option<&String>) -> String {
    value
        .map(|value| value.replacen("px", "", 1))
        .filter(|value| !value.is_empty())
        .map(|value| format!(r#" {name}="{value}""#))
        .unwrap_or_default()
}

fn class_attribute(attrs: &HashMap<String, String>) -> String {
    attrs
        .get("class")
        .filter(|value| !value.is_empty())
        .or_else(|| attrs.get("className").filter(|value| !value.is_empty()))
        .map(|value| format!(r#" class="{value}""#))
        .unwrap_or_default()
}
