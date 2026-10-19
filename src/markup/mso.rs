use std::collections::HashMap;

use super::styles::{
    compute_font_width_and_space_count, extract_all_style_attributes, parse_padding, px_to_pt,
    style_to_string,
};

// Outlook ignores regular anchor padding, so hair spaces and mso-text-raise preserve it.
pub(super) fn process_button(attrs: &HashMap<String, String>, inner: &str) -> String {
    let href = attrs
        .get("href")
        .filter(|value| !value.is_empty())
        .map(String::as_str)
        .unwrap_or("#");
    let style = extract_all_style_attributes(attrs);
    let (padding_top, padding_right, padding_bottom, padding_left) = parse_padding(&style);
    let text_raise = px_to_pt(padding_top + padding_bottom);
    let (left_font_width, left_space_count) = compute_font_width_and_space_count(padding_left);
    let (right_font_width, right_space_count) = compute_font_width_and_space_count(padding_right);

    let mut button_style = HashMap::from([
        ("line-height".to_string(), "100%".to_string()),
        ("text-decoration".to_string(), "none".to_string()),
        ("display".to_string(), "inline-block".to_string()),
        ("max-width".to_string(), "100%".to_string()),
        ("mso-padding-alt".to_string(), "0px".to_string()),
    ]);
    button_style.extend(style);
    button_style.insert("padding-top".to_string(), format!("{padding_top}px"));
    button_style.insert("padding-right".to_string(), format!("{padding_right}px"));
    button_style.insert("padding-bottom".to_string(), format!("{padding_bottom}px"));
    button_style.insert("padding-left".to_string(), format!("{padding_left}px"));

    let left_spaces = "&#8202;".repeat(left_space_count as usize);
    let right_spaces = "&#8202;".repeat(right_space_count as usize);

    format!(
        r#"<a href="{}" target="_blank" style="{}"><!--[if mso]><i style="mso-font-width:{}%;mso-text-raise:{}" hidden>{}</i><![endif]--><span style="max-width:100%;display:inline-block;line-height:120%;mso-padding-alt:0px;mso-text-raise:{}">{}</span><!--[if mso]><i style="mso-font-width:{}%" hidden>{}&#8203;</i><![endif]--></a>"#,
        href,
        style_to_string(&button_style),
        (left_font_width * 100.0 + 0.5).floor() as i32,
        text_raise,
        left_spaces,
        px_to_pt(padding_bottom),
        inner,
        (right_font_width * 100.0 + 0.5).floor() as i32,
        right_spaces
    )
}
