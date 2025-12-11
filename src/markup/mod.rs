//! Markup renderer for converting Sevk markup to HTML email

use regex::Regex;
use std::collections::HashMap;

/// Head settings for email generation
#[derive(Debug, Clone, Default)]
pub struct EmailHeadSettings {
    pub title: Option<String>,
    pub preview_text: Option<String>,
    pub styles: Option<String>,
    pub fonts: Vec<FontConfig>,
}

/// Font configuration
#[derive(Debug, Clone)]
pub struct FontConfig {
    pub id: String,
    pub name: String,
    pub url: String,
}

/// Parsed email content
#[derive(Debug, Clone)]
pub struct ParsedEmailContent {
    pub body: String,
    pub head_settings: EmailHeadSettings,
}

/// Generate email HTML from Sevk markup
pub fn generate_email_from_markup(html_content: &str, head_settings: Option<EmailHeadSettings>) -> String {
    let (content_to_process, settings) = if let Some(settings) = head_settings {
        (html_content.to_string(), settings)
    } else {
        let parsed = parse_email_html(html_content);
        (parsed.body, parsed.head_settings)
    };

    let normalized = normalize_markup(&content_to_process);
    let processed = process_markup(&normalized);

    // Build head content
    let title_tag = settings.title.as_ref()
        .map(|t| format!("<title>{}</title>", t))
        .unwrap_or_default();

    let font_links = generate_font_links(&settings.fonts);

    let custom_styles = settings.styles.as_ref()
        .map(|s| format!("<style type=\"text/css\">{}</style>", s))
        .unwrap_or_default();

    let preview_text = settings.preview_text.as_ref()
        .map(|p| format!(r#"<div style="display:none;font-size:1px;color:#ffffff;line-height:1px;max-height:0px;max-width:0px;opacity:0;overflow:hidden;">{}</div>"#, p))
        .unwrap_or_default();

    format!(r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">
<html lang="en" dir="ltr">
<head>
<meta content="text/html; charset=UTF-8" http-equiv="Content-Type"/>
<meta name="viewport" content="width=device-width, initial-scale=1.0"/>
{}
{}
{}
</head>
<body style="margin:0;padding:0;font-family:ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;background-color:#ffffff">
{}
{}
</body>
</html>"#, title_tag, font_links, custom_styles, preview_text, processed)
}

fn normalize_markup(content: &str) -> String {
    let mut result = content.to_string();

    // Replace <link> with <sevk-link>
    if result.contains("<link") {
        let re = Regex::new(r"<link\s+href=").unwrap();
        result = re.replace_all(&result, "<sevk-link href=").to_string();
        result = result.replace("</link>", "</sevk-link>");
    }

    if !result.contains("<sevk-email") && !result.contains("<email") && !result.contains("<mail") {
        result = format!("<mail><body>{}</body></mail>", result);
    }

    result
}

fn generate_font_links(fonts: &[FontConfig]) -> String {
    fonts.iter()
        .map(|f| format!(r#"<link href="{}" rel="stylesheet" type="text/css" />"#, f.url))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Parse email HTML and extract head settings
pub fn parse_email_html(content: &str) -> ParsedEmailContent {
    if content.contains("<email>") || content.contains("<email ") || content.contains("<mail>") || content.contains("<mail ") {
        parse_sevk_markup(content)
    } else {
        ParsedEmailContent {
            body: content.to_string(),
            head_settings: EmailHeadSettings::default(),
        }
    }
}

fn parse_sevk_markup(content: &str) -> ParsedEmailContent {
    let mut head_settings = EmailHeadSettings::default();

    // Extract title
    let title_re = Regex::new(r"<title[^>]*>([\s\S]*?)</title>").unwrap();
    if let Some(caps) = title_re.captures(content) {
        head_settings.title = Some(caps[1].trim().to_string());
    }

    // Extract preview
    let preview_re = Regex::new(r"<preview[^>]*>([\s\S]*?)</preview>").unwrap();
    if let Some(caps) = preview_re.captures(content) {
        head_settings.preview_text = Some(caps[1].trim().to_string());
    }

    // Extract styles
    let style_re = Regex::new(r"<style[^>]*>([\s\S]*?)</style>").unwrap();
    if let Some(caps) = style_re.captures(content) {
        head_settings.styles = Some(caps[1].trim().to_string());
    }

    // Extract fonts
    let font_re = Regex::new(r#"<font[^>]*name=["']([^"']*)["'][^>]*url=["']([^"']*)["'][^>]*/?\s*>"#).unwrap();
    for caps in font_re.captures_iter(content) {
        head_settings.fonts.push(FontConfig {
            id: format!("font-{}", head_settings.fonts.len()),
            name: caps[1].to_string(),
            url: caps[2].to_string(),
        });
    }

    // Extract body
    let body_re = Regex::new(r"<body[^>]*>([\s\S]*?)</body>").unwrap();
    let body = if let Some(caps) = body_re.captures(content) {
        caps[1].trim().to_string()
    } else {
        // Remove email, head, and meta tags
        let mut body = content.to_string();
        let patterns = [
            r"<email[^>]*>", r"</email>",
            r"<mail[^>]*>", r"</mail>",
            r"<head[^>]*>[\s\S]*?</head>",
            r"<title[^>]*>[\s\S]*?</title>",
            r"<preview[^>]*>[\s\S]*?</preview>",
            r"<style[^>]*>[\s\S]*?</style>",
            r"<font[^>]*>[\s\S]*?</font>",
            r"<font[^>]*/?>",
        ];
        for pattern in patterns {
            let re = Regex::new(pattern).unwrap();
            body = re.replace_all(&body, "").to_string();
        }
        body.trim().to_string()
    };

    ParsedEmailContent { body, head_settings }
}

/// Process markup and convert to HTML
fn process_markup(content: &str) -> String {
    let mut result = content.to_string();

    // Process section tags
    result = process_tag(&result, "section", |attrs, inner| {
        let style = extract_all_style_attributes(&attrs);
        let style_str = style_to_string(&style);
        format!(r#"<table align="center" width="100%" border="0" cellPadding="0" cellSpacing="0" role="presentation" style="{}">
<tbody>
<tr>
<td>{}</td>
</tr>
</tbody>
</table>"#, style_str, inner)
    });

    // Process row tags
    result = process_tag(&result, "row", |attrs, inner| {
        let style = extract_all_style_attributes(&attrs);
        let style_str = style_to_string(&style);
        format!(r#"<table align="center" width="100%" border="0" cellPadding="0" cellSpacing="0" role="presentation" style="{}">
<tbody style="width:100%">
<tr style="width:100%">{}</tr>
</tbody>
</table>"#, style_str, inner)
    });

    // Process column tags
    result = process_tag(&result, "column", |attrs, inner| {
        let style = extract_all_style_attributes(&attrs);
        let style_str = style_to_string(&style);
        format!(r#"<td style="{}">{}</td>"#, style_str, inner)
    });

    // Process container tags
    result = process_tag(&result, "container", |attrs, inner| {
        let style = extract_all_style_attributes(&attrs);
        let style_str = style_to_string(&style);
        format!(r#"<table align="center" width="100%" border="0" cellPadding="0" cellSpacing="0" role="presentation" style="{}">
<tbody>
<tr style="width:100%">
<td>{}</td>
</tr>
</tbody>
</table>"#, style_str, inner)
    });

    // Process heading tags
    result = process_tag(&result, "heading", |attrs, inner| {
        let level = attrs.get("level").map(|s| s.as_str()).unwrap_or("1");
        let style = extract_all_style_attributes(&attrs);
        let style_str = style_to_string(&style);
        format!(r#"<h{} style="{}">{}</h{}>"#, level, style_str, inner, level)
    });

    // Process paragraph tags
    result = process_tag(&result, "paragraph", |attrs, inner| {
        let style = extract_all_style_attributes(&attrs);
        let style_str = style_to_string(&style);
        format!(r#"<p style="{}">{}</p>"#, style_str, inner)
    });

    // Process text tags
    result = process_tag(&result, "text", |attrs, inner| {
        let style = extract_all_style_attributes(&attrs);
        let style_str = style_to_string(&style);
        format!(r#"<span style="{}">{}</span>"#, style_str, inner)
    });

    // Process button tags with MSO compatibility
    result = process_tag(&result, "button", |attrs, inner| {
        process_button(&attrs, inner)
    });

    // Process image tags
    result = process_tag(&result, "image", |attrs, _| {
        let src = attrs.get("src").map(|s| s.as_str()).unwrap_or("");
        let alt = attrs.get("alt").map(|s| s.as_str()).unwrap_or("");
        let width = attrs.get("width");
        let height = attrs.get("height");

        let mut style = extract_all_style_attributes(&attrs);
        // Add default image styles
        style.entry("outline".to_string()).or_insert_with(|| "none".to_string());
        style.entry("border".to_string()).or_insert_with(|| "none".to_string());
        style.entry("text-decoration".to_string()).or_insert_with(|| "none".to_string());

        let style_str = style_to_string(&style);
        let width_attr = width.map(|w| format!(r#" width="{}""#, w)).unwrap_or_default();
        let height_attr = height.map(|h| format!(r#" height="{}""#, h)).unwrap_or_default();

        format!(r#"<img src="{}" alt="{}"{}{} style="{}" />"#, src, alt, width_attr, height_attr, style_str)
    });

    // Process divider tags
    result = process_tag(&result, "divider", |attrs, _| {
        let style = extract_all_style_attributes(&attrs);
        let style_str = style_to_string(&style);
        let class_attr = attrs.get("class").or(attrs.get("className"))
            .map(|c| format!(r#" class="{}""#, c))
            .unwrap_or_default();
        format!(r#"<hr style="{}"{} />"#, style_str, class_attr)
    });

    // Process link tags
    result = process_tag(&result, "sevk-link", |attrs, inner| {
        let href = attrs.get("href").map(|s| s.as_str()).unwrap_or("#");
        let target = attrs.get("target").map(|s| s.as_str()).unwrap_or("_blank");
        let style = extract_all_style_attributes(&attrs);
        let style_str = style_to_string(&style);
        format!(r#"<a href="{}" target="{}" style="{}">{}</a>"#, href, target, style_str, inner)
    });

    // Process list tags
    result = process_tag(&result, "list", |attrs, inner| {
        let list_type = attrs.get("type").map(|s| s.as_str()).unwrap_or("unordered");
        let tag = if list_type == "ordered" { "ol" } else { "ul" };
        let mut style = extract_all_style_attributes(&attrs);
        if let Some(lst) = attrs.get("list-style-type") {
            style.insert("list-style-type".to_string(), lst.clone());
        }
        let style_str = style_to_string(&style);
        let class_attr = attrs.get("class").or(attrs.get("className"))
            .map(|c| format!(r#" class="{}""#, c))
            .unwrap_or_default();
        format!(r#"<{} style="{}"{}>{}</{}>  "#, tag, style_str, class_attr, inner, tag)
    });

    // Process list item tags
    result = process_tag(&result, "li", |attrs, inner| {
        let style = extract_all_style_attributes(&attrs);
        let style_str = style_to_string(&style);
        let class_attr = attrs.get("class").or(attrs.get("className"))
            .map(|c| format!(r#" class="{}""#, c))
            .unwrap_or_default();
        format!(r#"<li style="{}"{}>{}</li>"#, style_str, class_attr, inner)
    });

    // Process codeblock tags
    result = process_tag(&result, "codeblock", |attrs, inner| {
        let mut style = extract_all_style_attributes(&attrs);
        style.entry("width".to_string()).or_insert_with(|| "100%".to_string());
        style.entry("box-sizing".to_string()).or_insert_with(|| "border-box".to_string());
        let style_str = style_to_string(&style);
        let escaped = inner.replace("<", "&lt;").replace(">", "&gt;");
        format!(r#"<pre style="{}"><code>{}</code></pre>"#, style_str, escaped)
    });

    // Clean up wrapper tags
    let wrapper_patterns = [
        r"<sevk-email[^>]*>", r"</sevk-email>",
        r"<sevk-body[^>]*>", r"</sevk-body>",
        r"<email[^>]*>", r"</email>",
        r"<mail[^>]*>", r"</mail>",
        r"<body[^>]*>", r"</body>",
    ];
    for pattern in wrapper_patterns {
        let re = Regex::new(pattern).unwrap();
        result = re.replace_all(&result, "").to_string();
    }

    result.trim().to_string()
}

/// Process button with MSO compatibility (like Node.js)
fn process_button(attrs: &HashMap<String, String>, inner: &str) -> String {
    let href = attrs.get("href").map(|s| s.as_str()).unwrap_or("#");
    let style = extract_all_style_attributes(attrs);

    // Parse padding
    let (padding_top, padding_right, padding_bottom, padding_left) = parse_padding(&style);

    let y = padding_top + padding_bottom;
    let text_raise = px_to_pt(y);

    let (pl_font_width, pl_space_count) = compute_font_width_and_space_count(padding_left);
    let (pr_font_width, pr_space_count) = compute_font_width_and_space_count(padding_right);

    let mut button_style = HashMap::new();
    button_style.insert("line-height".to_string(), "100%".to_string());
    button_style.insert("text-decoration".to_string(), "none".to_string());
    button_style.insert("display".to_string(), "inline-block".to_string());
    button_style.insert("max-width".to_string(), "100%".to_string());
    button_style.insert("mso-padding-alt".to_string(), "0px".to_string());

    // Merge with extracted styles
    for (k, v) in &style {
        button_style.insert(k.clone(), v.clone());
    }

    // Override padding with parsed values
    button_style.insert("padding-top".to_string(), format!("{}px", padding_top));
    button_style.insert("padding-right".to_string(), format!("{}px", padding_right));
    button_style.insert("padding-bottom".to_string(), format!("{}px", padding_bottom));
    button_style.insert("padding-left".to_string(), format!("{}px", padding_left));

    let style_str = style_to_string(&button_style);

    let left_mso_spaces = "&#8202;".repeat(pl_space_count as usize);
    let right_mso_spaces = "&#8202;".repeat(pr_space_count as usize);

    format!(
        r#"<a href="{}" target="_blank" style="{}"><!--[if mso]><i style="mso-font-width:{}%;mso-text-raise:{}" hidden>{}</i><![endif]--><span style="max-width:100%;display:inline-block;line-height:120%;mso-padding-alt:0px;mso-text-raise:{}">{}</span><!--[if mso]><i style="mso-font-width:{}%" hidden>{}&#8203;</i><![endif]--></a>"#,
        href,
        style_str,
        (pl_font_width * 100.0) as i32,
        text_raise,
        left_mso_spaces,
        px_to_pt(padding_bottom),
        inner,
        (pr_font_width * 100.0) as i32,
        right_mso_spaces
    )
}

/// Parse padding values from style
fn parse_padding(style: &HashMap<String, String>) -> (i32, i32, i32, i32) {
    if let Some(padding) = style.get("padding") {
        let parts: Vec<&str> = padding.split_whitespace().collect();
        match parts.len() {
            1 => {
                let val = parse_px(parts[0]);
                (val, val, val, val)
            }
            2 => {
                let vertical = parse_px(parts[0]);
                let horizontal = parse_px(parts[1]);
                (vertical, horizontal, vertical, horizontal)
            }
            4 => {
                (parse_px(parts[0]), parse_px(parts[1]), parse_px(parts[2]), parse_px(parts[3]))
            }
            _ => (0, 0, 0, 0)
        }
    } else {
        let pt = style.get("padding-top").map(|s| parse_px(s)).unwrap_or(0);
        let pr = style.get("padding-right").map(|s| parse_px(s)).unwrap_or(0);
        let pb = style.get("padding-bottom").map(|s| parse_px(s)).unwrap_or(0);
        let pl = style.get("padding-left").map(|s| parse_px(s)).unwrap_or(0);
        (pt, pr, pb, pl)
    }
}

fn parse_px(s: &str) -> i32 {
    s.trim_end_matches("px").parse().unwrap_or(0)
}

/// Convert px to pt for MSO
fn px_to_pt(px: i32) -> i32 {
    (px * 3) / 4
}

/// Compute font width and space count for MSO padding
fn compute_font_width_and_space_count(expected_width: i32) -> (f64, i32) {
    if expected_width == 0 {
        return (0.0, 0);
    }

    let mut smallest_space_count = 0;
    let max_font_width = 5.0;

    loop {
        let required_font_width = if smallest_space_count > 0 {
            expected_width as f64 / smallest_space_count as f64 / 2.0
        } else {
            f64::INFINITY
        };

        if required_font_width <= max_font_width {
            return (required_font_width, smallest_space_count);
        }
        smallest_space_count += 1;
    }
}

fn process_tag<F>(content: &str, tag_name: &str, processor: F) -> String
where
    F: Fn(HashMap<String, String>, &str) -> String,
{
    let mut result = content.to_string();
    let open_pattern = format!(r"<{}([^>]*)>", tag_name);
    let close_tag = format!("</{}>", tag_name);
    let open_tag_start = format!("<{}", tag_name);
    let open_re = Regex::new(&open_pattern).unwrap();

    let mut iteration = 0;
    let max_iterations = 1000; // Safety limit

    loop {
        iteration += 1;
        if iteration > max_iterations {
            break;
        }

        // Find the innermost tag (one that has no nested same tags)
        let mut best_match: Option<(usize, usize, String, usize)> = None; // (start, attrs_end, attrs_str, close_pos)

        for cap in open_re.captures_iter(&result) {
            let full_match = cap.get(0).unwrap();
            let start = full_match.start();
            let attrs_end = full_match.end();
            let attrs_str = cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();

            let remaining = &result[attrs_end..];

            // Find the first close tag
            if let Some(close_pos) = remaining.find(&close_tag) {
                let inner = &remaining[..close_pos];

                // Check if there's a nested open tag inside
                let has_nested = inner.contains(&open_tag_start);

                if !has_nested {
                    // This is an innermost tag
                    best_match = Some((start, attrs_end, attrs_str, close_pos));
                    break;
                }
            }
        }

        if let Some((start, attrs_end, attrs_str, close_pos)) = best_match {
            let attrs = parse_attributes(&attrs_str);
            let inner = &result[attrs_end..attrs_end + close_pos];
            let replacement = processor(attrs, inner);
            let end = attrs_end + close_pos + close_tag.len();

            result = format!("{}{}{}", &result[..start], replacement, &result[end..]);
        } else {
            break;
        }
    }

    result
}

fn parse_attributes(attrs_str: &str) -> HashMap<String, String> {
    let mut attrs = HashMap::new();
    let re = Regex::new(r#"([\w-]+)=["']([^"']*)["']"#).unwrap();

    for caps in re.captures_iter(attrs_str) {
        attrs.insert(caps[1].to_string(), caps[2].to_string());
    }

    attrs
}

/// Extract all style attributes from element attributes (like Node.js extractStyleAttributes)
fn extract_all_style_attributes(attrs: &HashMap<String, String>) -> HashMap<String, String> {
    let mut style = HashMap::new();

    // Typography attributes
    if let Some(v) = attrs.get("text-color").or(attrs.get("color")) {
        style.insert("color".to_string(), v.clone());
    }
    if let Some(v) = attrs.get("background-color") {
        style.insert("background-color".to_string(), v.clone());
    }
    if let Some(v) = attrs.get("font-size") {
        style.insert("font-size".to_string(), v.clone());
    }
    if let Some(v) = attrs.get("font-family") {
        style.insert("font-family".to_string(), v.clone());
    }
    if let Some(v) = attrs.get("font-weight") {
        style.insert("font-weight".to_string(), v.clone());
    }
    if let Some(v) = attrs.get("line-height") {
        style.insert("line-height".to_string(), v.clone());
    }
    if let Some(v) = attrs.get("text-align") {
        style.insert("text-align".to_string(), v.clone());
    }
    if let Some(v) = attrs.get("text-decoration") {
        style.insert("text-decoration".to_string(), v.clone());
    }

    // Dimensions
    if let Some(v) = attrs.get("width") {
        style.insert("width".to_string(), v.clone());
    }
    if let Some(v) = attrs.get("height") {
        style.insert("height".to_string(), v.clone());
    }
    if let Some(v) = attrs.get("max-width") {
        style.insert("max-width".to_string(), v.clone());
    }
    if let Some(v) = attrs.get("min-height") {
        style.insert("min-height".to_string(), v.clone());
    }

    // Spacing - Padding
    if let Some(v) = attrs.get("padding") {
        style.insert("padding".to_string(), v.clone());
    } else {
        if let Some(v) = attrs.get("padding-top") {
            style.insert("padding-top".to_string(), v.clone());
        }
        if let Some(v) = attrs.get("padding-right") {
            style.insert("padding-right".to_string(), v.clone());
        }
        if let Some(v) = attrs.get("padding-bottom") {
            style.insert("padding-bottom".to_string(), v.clone());
        }
        if let Some(v) = attrs.get("padding-left") {
            style.insert("padding-left".to_string(), v.clone());
        }
    }

    // Spacing - Margin
    if let Some(v) = attrs.get("margin") {
        style.insert("margin".to_string(), v.clone());
    } else {
        if let Some(v) = attrs.get("margin-top") {
            style.insert("margin-top".to_string(), v.clone());
        }
        if let Some(v) = attrs.get("margin-right") {
            style.insert("margin-right".to_string(), v.clone());
        }
        if let Some(v) = attrs.get("margin-bottom") {
            style.insert("margin-bottom".to_string(), v.clone());
        }
        if let Some(v) = attrs.get("margin-left") {
            style.insert("margin-left".to_string(), v.clone());
        }
    }

    // Borders
    if let Some(v) = attrs.get("border") {
        style.insert("border".to_string(), v.clone());
    } else {
        if let Some(v) = attrs.get("border-top") {
            style.insert("border-top".to_string(), v.clone());
        }
        if let Some(v) = attrs.get("border-right") {
            style.insert("border-right".to_string(), v.clone());
        }
        if let Some(v) = attrs.get("border-bottom") {
            style.insert("border-bottom".to_string(), v.clone());
        }
        if let Some(v) = attrs.get("border-left") {
            style.insert("border-left".to_string(), v.clone());
        }
        if let Some(v) = attrs.get("border-color") {
            style.insert("border-color".to_string(), v.clone());
        }
        if let Some(v) = attrs.get("border-width") {
            style.insert("border-width".to_string(), v.clone());
        }
        if let Some(v) = attrs.get("border-style") {
            style.insert("border-style".to_string(), v.clone());
        }
    }

    // Border Radius
    if let Some(v) = attrs.get("border-radius") {
        style.insert("border-radius".to_string(), v.clone());
    } else {
        if let Some(v) = attrs.get("border-top-left-radius") {
            style.insert("border-top-left-radius".to_string(), v.clone());
        }
        if let Some(v) = attrs.get("border-top-right-radius") {
            style.insert("border-top-right-radius".to_string(), v.clone());
        }
        if let Some(v) = attrs.get("border-bottom-left-radius") {
            style.insert("border-bottom-left-radius".to_string(), v.clone());
        }
        if let Some(v) = attrs.get("border-bottom-right-radius") {
            style.insert("border-bottom-right-radius".to_string(), v.clone());
        }
    }

    style
}

/// Convert style HashMap to inline style string
fn style_to_string(style: &HashMap<String, String>) -> String {
    style.iter()
        .map(|(k, v)| format!("{}:{}", k, v))
        .collect::<Vec<_>>()
        .join(";")
}
