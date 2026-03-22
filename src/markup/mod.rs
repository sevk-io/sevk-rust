//! Markup renderer for converting Sevk markup to HTML email

use regex::Regex;
use serde_json;
use std::collections::HashMap;
use std::str::FromStr;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{Theme, ThemeSettings, ThemeItem, StyleModifier, ScopeSelectors, Style, Color, FontStyle as SynFontStyle};
use syntect::easy::HighlightLines;
use syntect::util::LinesWithEndings;

/// Head settings for email generation
#[derive(Debug, Clone, Default)]
pub struct EmailHeadSettings {
    pub title: Option<String>,
    pub preview_text: Option<String>,
    pub styles: Option<String>,
    pub fonts: Vec<FontConfig>,
    pub lang: Option<String>,
    pub dir: Option<String>,
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

/// Render Sevk markup to email-compatible HTML
pub fn render(html_content: &str) -> String {
    generate_email_from_markup(html_content, None)
}

/// Generate email HTML from Sevk markup with optional head settings
pub fn generate_email_from_markup(html_content: &str, head_settings: Option<EmailHeadSettings>) -> String {
    // Always parse to extract clean body content (strips <mail>/<head> wrapper tags)
    let parsed = parse_email_html(html_content);
    let settings = head_settings.unwrap_or(parsed.head_settings);
    let content_to_process = parsed.body;

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

    let lang = settings.lang.as_deref().unwrap_or("en");
    let dir = settings.dir.as_deref().unwrap_or("ltr");

    format!(r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">
<html lang="{}" dir="{}" xmlns="http://www.w3.org/1999/xhtml" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office">
<head>
<meta content="text/html; charset=UTF-8" http-equiv="Content-Type"/>
<meta name="viewport" content="width=device-width, initial-scale=1.0"/>
<meta name="x-apple-disable-message-reformatting"/>
<meta content="IE=edge" http-equiv="X-UA-Compatible"/>
<meta name="format-detection" content="telephone=no,address=no,email=no,date=no,url=no"/>
<!--[if mso]>
<noscript>
<xml>
<o:OfficeDocumentSettings>
<o:AllowPNG/>
<o:PixelsPerInch>96</o:PixelsPerInch>
</o:OfficeDocumentSettings>
</xml>
</noscript>
<![endif]-->
<style type="text/css">
#outlook a {{ padding: 0; }}
body {{ margin: 0; padding: 0; -webkit-text-size-adjust: 100%; -ms-text-size-adjust: 100%; }}
table, td {{ border-collapse: collapse; mso-table-lspace: 0pt; mso-table-rspace: 0pt; }}
.sevk-row-table {{ border-collapse: separate !important; }}
img {{ border: 0; height: auto; line-height: 100%; outline: none; text-decoration: none; -ms-interpolation-mode: bicubic; }}
@media only screen and (max-width: 479px) {{
  .sevk-row-table {{ width: 100% !important; }}
  .sevk-column {{ display: block !important; width: 100% !important; max-width: 100% !important; }}
}}
</style>
{}
{}
{}
</head>
<body style="margin:0;padding:0;word-spacing:normal;-webkit-text-size-adjust:100%;-ms-text-size-adjust:100%;font-family:ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;">
<div aria-roledescription="email" role="article">
{}
{}
</div>
</body>
</html>"#, lang, dir, title_tag, font_links, custom_styles, preview_text, processed)
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

    // Parse lang and dir from <mail> or <email> root tag
    let root_re = Regex::new(r"(?i)<(?:email|mail)([^>]*)>").unwrap();
    if let Some(caps) = root_re.captures(content) {
        let root_attrs = &caps[1];
        let lang_re = Regex::new(r#"(?i)lang=["']([^"']*)["']"#).unwrap();
        let dir_re = Regex::new(r#"(?i)dir=["']([^"']*)["']"#).unwrap();
        if let Some(lang_caps) = lang_re.captures(root_attrs) {
            head_settings.lang = Some(lang_caps[1].to_string());
        }
        if let Some(dir_caps) = dir_re.captures(root_attrs) {
            head_settings.dir = Some(dir_caps[1].to_string());
        }
    }

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

    // Ensure <link> tags are converted to <sevk-link> for processing
    let link_re = Regex::new(r"(?i)<link\s+href=").unwrap();
    result = link_re.replace_all(&result, "<sevk-link href=").to_string();
    let close_link_re = Regex::new(r"(?i)</link>").unwrap();
    result = close_link_re.replace_all(&result, "</sevk-link>").to_string();

    // Process block tags with inner content BEFORE other tags
    result = process_tag(&result, "block", |attrs, inner| {
        process_block_tag(&attrs, inner)
    });

    // Self-closing blocks - <block type="..." config="..." />
    let block_re = Regex::new(r"(?i)<block([^>]*)/?\s*>(?:</block>)?").unwrap();
    result = block_re.replace_all(&result, |caps: &regex::Captures| {
        let attrs_str = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let attrs = parse_attributes(attrs_str);
        process_block_tag(&attrs, "")
    }).to_string();

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
    let row_counter = std::cell::Cell::new(0usize);
    let current_row_gap = std::cell::Cell::new(0i32);
    result = process_tag(&result, "row", |attrs, inner| {
        let gap = attrs.get("gap").map(|s| s.as_str()).unwrap_or("0");
        let mut style = extract_all_style_attributes(&attrs);
        style.remove("gap");
        let style_str = style_to_string(&style);
        let gap_px = gap.replace("px", "");
        let gap_num = gap_px.parse::<i32>().unwrap_or(0);
        let row_id = format!("sevk-row-{}", row_counter.get());
        row_counter.set(row_counter.get() + 1);
        current_row_gap.set(gap_num);

        // Assign equal widths to columns if more than one
        let mut processed_inner = inner.to_string();
        let column_re = Regex::new(r#"class="sevk-column""#).unwrap();
        let column_count = column_re.find_iter(inner).count();
        if column_count > 1 {
            let equal_width = format!("{}%", 100 / column_count);
            let col_replace_re = Regex::new(r#"<td class="sevk-column" style="([^"]*)"#).unwrap();
            processed_inner = col_replace_re.replace_all(&processed_inner, |caps: &regex::Captures| {
                let existing_style = &caps[1];
                if existing_style.contains("width:") {
                    caps[0].to_string()
                } else {
                    format!(r#"<td class="sevk-column" style="width:{};{}"#, equal_width, existing_style)
                }
            }).to_string();
        }

        let gap_style = if gap_num > 0 {
            format!("<style>@media only screen and (max-width:479px){{.{} > tbody > tr > td{{margin-bottom:{}px !important;padding-left:0 !important;padding-right:0 !important;}}.{} > tbody > tr > td:last-child{{margin-bottom:0 !important;}}}}</style>", row_id, gap_px, row_id)
        } else {
            String::new()
        };
        format!(r#"{}<table class="sevk-row-table {}" align="center" width="100%" border="0" cellPadding="0" cellSpacing="0" role="presentation" style="{}">
<tbody style="width:100%">
<tr style="width:100%">{}</tr>
</tbody>
</table>"#, gap_style, row_id, style_str, processed_inner)
    });

    // Process column tags - apply half-gap padding from parent row, vertical-align:top
    result = process_tag(&result, "column", |attrs, inner| {
        let mut style = extract_all_style_attributes(&attrs);
        let gap_num = current_row_gap.get();
        if gap_num > 0 {
            let half_gap = gap_num as f64 / 2.0;
            style.entry("padding-left".to_string()).or_insert_with(|| format!("{}px", half_gap));
            style.entry("padding-right".to_string()).or_insert_with(|| format!("{}px", half_gap));
        }
        style.entry("vertical-align".to_string()).or_insert_with(|| "top".to_string());
        let style_str = style_to_string(&style);
        format!(r#"<td class="sevk-column" style="{}">{}</td>"#, style_str, inner)
    });

    // Process container tags - split styles between table and td
    result = process_tag(&result, "container", |attrs, inner| {
        let style = extract_all_style_attributes(&attrs);
        let mut td_style: HashMap<String, String> = HashMap::new();
        let mut table_style: HashMap<String, String> = HashMap::new();

        let visual_keys = [
            "background-color", "background-image", "background-size", "background-position", "background-repeat",
            "border", "border-top", "border-right", "border-bottom", "border-left",
            "border-color", "border-width", "border-style",
            "border-radius", "border-top-left-radius", "border-top-right-radius",
            "border-bottom-left-radius", "border-bottom-right-radius",
            "padding", "padding-top", "padding-right", "padding-bottom", "padding-left",
        ];

        for (key, value) in &style {
            if visual_keys.contains(&key.as_str()) {
                td_style.insert(key.clone(), value.clone());
            } else {
                table_style.insert(key.clone(), value.clone());
            }
        }

        // Add border-collapse: separate when border-radius is used
        let has_border_radius = td_style.contains_key("border-radius")
            || td_style.contains_key("border-top-left-radius")
            || td_style.contains_key("border-top-right-radius")
            || td_style.contains_key("border-bottom-left-radius")
            || td_style.contains_key("border-bottom-right-radius");
        if has_border_radius {
            table_style.insert("border-collapse".to_string(), "separate".to_string());
        }

        // Make fixed widths responsive: width becomes max-width, width set to 100%
        if let Some(w) = table_style.get("width").cloned() {
            if w != "100%" && w != "auto" {
                table_style.entry("max-width".to_string()).or_insert(w);
                table_style.insert("width".to_string(), "100%".to_string());
            }
        }

        let table_style_str = style_to_string(&table_style);
        let td_style_str = style_to_string(&td_style);
        format!(r#"<table align="center" width="100%" border="0" cellPadding="0" cellSpacing="0" role="presentation" style="{}">
<tbody>
<tr style="width:100%">
<td style="{}">{}</td>
</tr>
</tbody>
</table>"#, table_style_str, td_style_str, inner)
    });

    // Process heading tags
    result = process_tag(&result, "heading", |attrs, inner| {
        let level = attrs.get("level").map(|s| s.as_str()).unwrap_or("1");
        let mut style = extract_all_style_attributes(&attrs);
        style.entry("margin".to_string()).or_insert_with(|| "0".to_string());
        let style_str = style_to_string(&style);
        format!(r#"<h{} style="{}">{}</h{}>"#, level, style_str, inner, level)
    });

    // Process paragraph tags
    result = process_tag(&result, "paragraph", |attrs, inner| {
        let mut style = extract_all_style_attributes(&attrs);
        style.entry("margin".to_string()).or_insert_with(|| "0".to_string());
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
        style.entry("vertical-align".to_string()).or_insert_with(|| "middle".to_string());
        style.entry("max-width".to_string()).or_insert_with(|| "100%".to_string());
        style.entry("outline".to_string()).or_insert_with(|| "none".to_string());
        style.entry("border".to_string()).or_insert_with(|| "none".to_string());
        style.entry("text-decoration".to_string()).or_insert_with(|| "none".to_string());

        let style_str = style_to_string(&style);
        let width_attr = width.map(|w| format!(r#" width="{}""#, w.trim_end_matches("px"))).unwrap_or_default();
        let height_attr = height.map(|h| format!(r#" height="{}""#, h.trim_end_matches("px"))).unwrap_or_default();

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
    // Remove any stray </divider> closing tags
    let divider_close_re = Regex::new(r"(?i)</divider>").unwrap();
    result = divider_close_re.replace_all(&result, "").to_string();

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
        style.entry("margin".to_string()).or_insert_with(|| "0".to_string());
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
        process_codeblock(&attrs, inner)
    });

    // Clean up stray Sevk closing tags that weren't consumed by processTag
    let stray_re = Regex::new(r"(?i)</(?:container|section|row|column|heading|paragraph|text|button|sevk-link)>").unwrap();
    result = stray_re.replace_all(&result, "").to_string();

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

/// Process codeblock tag with syntax highlighting via syntect
fn process_codeblock(attrs: &HashMap<String, String>, inner: &str) -> String {
    let language = attrs.get("language").map(|s| s.as_str()).unwrap_or("");
    let mut style = extract_all_style_attributes(attrs);
    style.entry("width".to_string()).or_insert_with(|| "100%".to_string());
    style.entry("box-sizing".to_string()).or_insert_with(|| "border-box".to_string());
    style.entry("background-color".to_string()).or_insert_with(|| "#282c34".to_string());
    style.entry("color".to_string()).or_insert_with(|| "#abb2bf".to_string());
    style.entry("border-radius".to_string()).or_insert_with(|| "8px".to_string());
    style.entry("padding".to_string()).or_insert_with(|| "16px".to_string());
    style.entry("overflow-x".to_string()).or_insert_with(|| "auto".to_string());
    style.entry("font-family".to_string()).or_insert_with(|| "'Fira Code', 'Courier New', Courier, monospace".to_string());
    style.entry("font-size".to_string()).or_insert_with(|| "14px".to_string());
    style.entry("line-height".to_string()).or_insert_with(|| "1.5".to_string());
    let style_str = style_to_string(&style);

    // Decode HTML entities back to raw text for highlighting
    let raw = inner
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'");

    if !language.is_empty() {
        if let Some(highlighted) = highlight_code(&raw, language) {
            return format!(
                r#"<pre style="{}"><code>{}</code></pre>"#,
                style_str, highlighted
            );
        }
    }

    // Fallback: plain escaped code
    let escaped = inner.replace('<', "&lt;").replace('>', "&gt;");
    format!(r#"<pre style="{}"><code>{}</code></pre>"#, style_str, escaped)
}

/// Build a One Dark-like syntect Theme
fn one_dark_theme() -> Theme {
    let fg = Color { r: 171, g: 178, b: 191, a: 255 }; // #abb2bf
    let bg = Color { r: 40, g: 44, b: 52, a: 255 };    // #282c34

    let rule = |scope: &str, color: (u8, u8, u8), bold: bool, italic: bool| -> ThemeItem {
        let mut font_style = SynFontStyle::empty();
        if bold { font_style |= SynFontStyle::BOLD; }
        if italic { font_style |= SynFontStyle::ITALIC; }
        ThemeItem {
            scope: ScopeSelectors::from_str(scope).unwrap_or_default(),
            style: StyleModifier {
                foreground: Some(Color { r: color.0, g: color.1, b: color.2, a: 255 }),
                background: None,
                font_style: Some(font_style),
            },
        }
    };

    Theme {
        name: Some("One Dark".to_string()),
        author: None,
        settings: ThemeSettings {
            foreground: Some(fg),
            background: Some(bg),
            ..ThemeSettings::default()
        },
        scopes: vec![
            rule("comment", (92, 99, 112), false, true),           // #5c6370
            rule("keyword", (198, 120, 221), false, false),         // #c678dd
            rule("keyword.operator", (86, 182, 194), false, false), // #56b6c2
            rule("storage", (198, 120, 221), false, false),         // #c678dd
            rule("string", (152, 195, 121), false, false),          // #98c379
            rule("constant.numeric", (209, 154, 102), false, false),// #d19a66
            rule("constant.language", (209, 154, 102), false, false),
            rule("variable", (224, 108, 117), false, false),        // #e06c75
            rule("entity.name.function", (97, 175, 239), false, false), // #61afef
            rule("entity.name.type", (229, 192, 123), false, false),    // #e5c07b
            rule("entity.name.class", (229, 192, 123), false, false),
            rule("entity.name.tag", (224, 108, 117), false, false),
            rule("entity.other.attribute-name", (209, 154, 102), false, false),
            rule("support.function", (97, 175, 239), false, false),
            rule("support.type", (86, 182, 194), false, false),
            rule("punctuation", (171, 178, 191), false, false),     // #abb2bf
            rule("meta.function-call", (97, 175, 239), false, false),
        ],
    }
}

/// Highlight source code using syntect with One Dark-like inline styles
fn highlight_code(code: &str, language: &str) -> Option<String> {
    let ss = SyntaxSet::load_defaults_newlines();

    let syntax = ss.find_syntax_by_token(language)
        .or_else(|| ss.find_syntax_by_extension(language))?;

    let theme = one_dark_theme();

    let mut h = HighlightLines::new(syntax, &theme);
    let mut output = String::new();

    for line in LinesWithEndings::from(code) {
        let ranges = h.highlight_line(line, &ss).ok()?;
        for (style, text) in ranges {
            let escaped = text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;");
            let css = inline_style_from_syntect(style);
            if css.is_empty() {
                output.push_str(&escaped);
            } else {
                output.push_str(&format!(r#"<span style="{}">{}</span>"#, css, escaped));
            }
        }
    }

    Some(output)
}

/// Convert a syntect Style into a CSS inline style string
fn inline_style_from_syntect(style: Style) -> String {
    let mut parts = Vec::new();

    // Always emit color
    let fg = style.foreground;
    parts.push(format!("color:#{:02x}{:02x}{:02x}", fg.r, fg.g, fg.b));

    if style.font_style.contains(SynFontStyle::BOLD) {
        parts.push("font-weight:bold".to_string());
    }
    if style.font_style.contains(SynFontStyle::ITALIC) {
        parts.push("font-style:italic".to_string());
    }
    if style.font_style.contains(SynFontStyle::UNDERLINE) {
        parts.push("text-decoration:underline".to_string());
    }

    parts.join(";")
}

/// Check if a serde_json Value is truthy
fn is_truthy(val: &serde_json::Value) -> bool {
    match val {
        serde_json::Value::Null => false,
        serde_json::Value::Bool(b) => *b,
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i != 0
            } else if let Some(f) = n.as_f64() {
                f != 0.0
            } else {
                true
            }
        }
        serde_json::Value::String(s) => !s.is_empty(),
        serde_json::Value::Array(a) => !a.is_empty(),
        serde_json::Value::Object(_) => true,
    }
}

/// Evaluate a condition expression against the config map
fn evaluate_condition(expr: &str, config: &serde_json::Map<String, serde_json::Value>) -> bool {
    let trimmed = expr.trim();

    // OR: split on ||, return true if any part is true
    if trimmed.contains("||") {
        return trimmed.split("||").any(|part| evaluate_condition(part, config));
    }

    // AND: split on &&, return true if all parts are true
    if trimmed.contains("&&") {
        return trimmed.split("&&").all(|part| evaluate_condition(part, config));
    }

    // Equality: key == "value"
    let eq_re = Regex::new(r#"^(\w+)\s*==\s*"([^"]*)"$"#).unwrap();
    if let Some(caps) = eq_re.captures(trimmed) {
        let key = &caps[1];
        let expected = &caps[2];
        let val = config.get(key).map(|v| match v {
            serde_json::Value::String(s) => s.clone(),
            other => other.to_string(),
        }).unwrap_or_default();
        return val == expected;
    }

    // Inequality: key != "value"
    let neq_re = Regex::new(r#"^(\w+)\s*!=\s*"([^"]*)"$"#).unwrap();
    if let Some(caps) = neq_re.captures(trimmed) {
        let key = &caps[1];
        let expected = &caps[2];
        let val = config.get(key).map(|v| match v {
            serde_json::Value::String(s) => s.clone(),
            other => other.to_string(),
        }).unwrap_or_default();
        return val != expected;
    }

    // Simple truthy check
    config.get(trimmed).map(|v| is_truthy(v)).unwrap_or(false)
}

/// Render a template string with the given config map
fn render_template(template: &str, config: &serde_json::Map<String, serde_json::Value>) -> String {
    let mut result = template.to_string();

    // Process #each loops
    let each_re = Regex::new(r"(?s)\{%#each\s+(\w+)(?:\s+as\s+(\w+))?%\}(.*?)\{%/each%\}").unwrap();
    loop {
        let new_result = each_re.replace_all(&result, |caps: &regex::Captures| {
            let key = &caps[1];
            let alias = caps.get(2).map(|m| m.as_str()).unwrap_or("item");
            let body = &caps[3];
            if let Some(serde_json::Value::Array(arr)) = config.get(key) {
                arr.iter().map(|item| {
                    let mut item_config = config.clone();
                    item_config.insert(alias.to_string(), item.clone());
                    // Also flatten object fields into the config
                    if let serde_json::Value::Object(obj) = item {
                        for (k, v) in obj {
                            item_config.insert(k.clone(), v.clone());
                        }
                    }
                    render_template(body, &item_config)
                }).collect::<Vec<_>>().join("")
            } else {
                String::new()
            }
        }).to_string();
        if new_result == result {
            break;
        }
        result = new_result;
    }

    // Process nested #if/#else conditionals (innermost first, loop until stable)
    // Find the innermost {%#if...%}...{%/if%} (no nested #if in body) and resolve it
    loop {
        // Find the last {%#if before a {%/if%} that doesn't have another {%#if between them
        let close_re = Regex::new(r"\{%/if%\}").unwrap();
        let open_re = Regex::new(r"\{%#if\s+([^%]+)%\}").unwrap();
        let mut changed = false;
        if let Some(close_match) = close_re.find(&result) {
            // Find the last {%#if before this {%/if%}
            let before_close = &result[..close_match.start()];
            let mut last_open: Option<(usize, usize, String)> = None;
            for cap in open_re.captures_iter(before_close) {
                let m = cap.get(0).unwrap();
                last_open = Some((m.start(), m.end(), cap[1].to_string()));
            }
            if let Some((open_start, open_end, key)) = last_open {
                let body = &result[open_end..close_match.start()];
                let is_true = evaluate_condition(&key, config);
                let replacement = if let Some(else_pos) = body.find("{%else%}") {
                    if is_true {
                        body[..else_pos].to_string()
                    } else {
                        body[else_pos + 8..].to_string()
                    }
                } else {
                    if is_true {
                        body.to_string()
                    } else {
                        String::new()
                    }
                };
                result = format!("{}{}{}", &result[..open_start], replacement, &result[close_match.end()..]);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    // Process fallback variables: {%variable ?? fallback%}
    let fallback_re = Regex::new(r"\{%(\w+)\s*\?\?\s*([^%]+)%\}").unwrap();
    result = fallback_re.replace_all(&result, |caps: &regex::Captures| {
        let key = &caps[1];
        let fallback = caps[2].trim();
        match config.get(key) {
            Some(serde_json::Value::String(s)) if !s.is_empty() => s.clone(),
            Some(v) if is_truthy(v) => match v {
                serde_json::Value::String(s) => s.clone(),
                other => other.to_string(),
            },
            _ => fallback.to_string(),
        }
    }).to_string();

    // Process simple variables: {%variable%}
    let simple_re = Regex::new(r"\{%(\w+)%\}").unwrap();
    result = simple_re.replace_all(&result, |caps: &regex::Captures| {
        let key = &caps[1];
        match config.get(key) {
            Some(serde_json::Value::String(s)) => s.clone(),
            Some(serde_json::Value::Null) => String::new(),
            Some(v) => v.to_string(),
            None => String::new(),
        }
    }).to_string();

    result
}

/// Process a block tag with template and config attributes
fn process_block_tag(attrs: &HashMap<String, String>, inner: &str) -> String {
    let template = if inner.trim().is_empty() {
        attrs.get("template").cloned().unwrap_or_default()
    } else {
        inner.trim().to_string()
    };
    if template.is_empty() {
        return String::new();
    }
    let config_str = attrs.get("config").cloned().unwrap_or_else(|| "{}".to_string())
        .replace("'", "\"")
        .replace("&quot;", "\"")
        .replace("&amp;", "&");
    let config: serde_json::Map<String, serde_json::Value> = serde_json::from_str(&config_str)
        .unwrap_or_default();
    render_template(&template, &config)
}

fn process_tag<F>(content: &str, tag_name: &str, processor: F) -> String
where
    F: Fn(HashMap<String, String>, &str) -> String,
{
    let mut result = content.to_string();
    let open_pattern = format!(r"(?i)<{}([^>]*)>", tag_name);
    let open_re = Regex::new(&open_pattern).unwrap();
    let close_re = Regex::new(&format!("(?i)</{}>", tag_name)).unwrap();
    let nested_open_re = Regex::new(&format!("(?i)<{}", tag_name)).unwrap();

    let mut iteration = 0;
    let max_iterations = 1000; // Safety limit

    loop {
        iteration += 1;
        if iteration > max_iterations {
            break;
        }

        // Find the innermost tag (one that has no nested same tags)
        let mut best_match: Option<(usize, usize, String, usize, usize)> = None; // (start, attrs_end, attrs_str, close_start_rel, close_len)

        for cap in open_re.captures_iter(&result) {
            let full_match = cap.get(0).unwrap();
            let start = full_match.start();
            let attrs_end = full_match.end();
            let attrs_str = cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();

            let remaining = &result[attrs_end..];

            // Find the first close tag using case-insensitive regex
            if let Some(close_match) = close_re.find(remaining) {
                let close_pos = close_match.start();
                let close_len = close_match.end() - close_match.start();
                let inner = &remaining[..close_pos];

                // Check if there's a nested open tag inside using case-insensitive regex
                let has_nested = nested_open_re.is_match(inner);

                if !has_nested {
                    // This is an innermost tag
                    best_match = Some((start, attrs_end, attrs_str, close_pos, close_len));
                    break;
                }
            }
        }

        if let Some((start, attrs_end, attrs_str, close_pos, close_len)) = best_match {
            let attrs = parse_attributes(&attrs_str);
            let inner = &result[attrs_end..attrs_end + close_pos];
            let replacement = processor(attrs, inner);
            let end = attrs_end + close_pos + close_len;

            result = format!("{}{}{}", &result[..start], replacement, &result[end..]);
        } else {
            break;
        }
    }

    result
}

fn parse_attributes(attrs_str: &str) -> HashMap<String, String> {
    let mut attrs = HashMap::new();
    let re = Regex::new(r#"([\w-]+)=(?:"([^"]*)"|'([^']*)')"#).unwrap();

    for caps in re.captures_iter(attrs_str) {
        let value = caps.get(2)
            .or_else(|| caps.get(3))
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        attrs.insert(caps[1].to_string(), value);
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

    // Background image support
    let background_image = attrs.get("background-image").cloned();
    if let Some(v) = attrs.get("background-size") {
        style.insert("background-size".to_string(), v.clone());
    }
    if let Some(v) = attrs.get("background-position") {
        style.insert("background-position".to_string(), v.clone());
    }
    if let Some(v) = attrs.get("background-repeat") {
        style.insert("background-repeat".to_string(), v.clone());
    }
    if let Some(ref bg_img) = background_image {
        style.insert("background-image".to_string(), format!("url('{}')", bg_img));
        style.entry("background-size".to_string()).or_insert_with(|| "cover".to_string());
        style.entry("background-position".to_string()).or_insert_with(|| "center".to_string());
        style.entry("background-repeat".to_string()).or_insert_with(|| "no-repeat".to_string());
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
    if let Some(v) = attrs.get("max-height") {
        style.insert("max-height".to_string(), v.clone());
    }
    if let Some(v) = attrs.get("min-width") {
        style.insert("min-width".to_string(), v.clone());
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
