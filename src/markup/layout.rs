use std::collections::HashMap;

use super::styles::{extract_all_style_attributes, parse_px, style_to_string};

pub(super) fn process_section(attrs: &HashMap<String, String>, inner: &str) -> String {
    let mut style = extract_all_style_attributes(attrs);
    let mut td_style = HashMap::new();
    if let Some(value) = style.remove("padding") {
        td_style.insert("padding".to_string(), value);
    }
    if let Some(value) = style.remove("text-align") {
        td_style.insert("text-align".to_string(), value);
    }

    let radius_keys = [
        "border-radius",
        "border-top-left-radius",
        "border-top-right-radius",
        "border-bottom-left-radius",
        "border-bottom-right-radius",
    ];
    if radius_keys.iter().any(|key| style.contains_key(*key)) {
        style.insert("border-collapse".to_string(), "separate".to_string());
    }

    format!(
        r#"<table align="center" width="100%" border="0" cellPadding="0" cellSpacing="0" role="presentation" style="{}">
<tbody>
<tr>
<td style="{}">{}</td>
</tr>
</tbody>
</table>"#,
        style_to_string(&style),
        style_to_string(&td_style),
        inner
    )
}

pub(super) fn process_column(attrs: &HashMap<String, String>, inner: &str) -> String {
    let mut style = extract_all_style_attributes(attrs);
    style
        .entry("vertical-align".to_string())
        .or_insert_with(|| "top".to_string());

    format!(
        r#"<td class="sevk-column" style="{}">{}</td>"#,
        style_to_string(&style),
        inner
    )
}

pub(super) fn process_row(
    attrs: &HashMap<String, String>,
    inner: &str,
    row_index: usize,
) -> (String, Option<String>) {
    let gap = attrs.get("gap").map(String::as_str).unwrap_or("0");
    let mut style = extract_all_style_attributes(attrs);
    style.remove("gap");

    let gap_px = gap.replacen("px", "", 1);
    let gap_num = parse_px(&gap_px);
    let row_id = format!("sevk-row-{row_index}");
    let mut processed_inner = inner.to_string();

    let column_count = regex!(r#"class="sevk-column""#).find_iter(inner).count();
    if column_count > 1 {
        let equal_width = format!("{}%", 100 / column_count);
        let column_re = regex!(r#"<td class="sevk-column" style="([^"]*)"#);
        processed_inner = column_re
            .replace_all(&processed_inner, |captures: &regex::Captures| {
                let existing_style = &captures[1];
                if existing_style.contains("width:") {
                    captures[0].to_string()
                } else {
                    format!(
                        r#"<td class="sevk-column" style="width:{equal_width};{existing_style}"#
                    )
                }
            })
            .to_string();
    }

    let gap_rule = if gap_num > 0 {
        let spacer = format!(
            r#"</td><td class="sevk-gap" style="width:{gap_px}px;min-width:{gap_px}px" width="{gap_px}"></td><td class="sevk-column""#
        );
        let insertion = regex!(r#"</td>\s*<td class="sevk-column""#);
        processed_inner = insertion
            .replace_all(&processed_inner, spacer.as_str())
            .to_string();

        Some(format!(".{row_id} .sevk-gap{{display:none !important;}}.{row_id} > tbody > tr > td.sevk-column{{display:block !important;width:100% !important;margin-bottom:{gap_px}px !important;}}.{row_id} > tbody > tr > td.sevk-column:last-of-type{{margin-bottom:0 !important;}}"))
    } else {
        None
    };

    let html = format!(
        r#"<table align="center" width="100%" border="0" cellPadding="0" cellSpacing="0" role="presentation" class="sevk-row-table {}" style="{}">
<tbody style="width:100%">
<tr style="width:100%">{}</tr>
</tbody>
</table>"#,
        row_id,
        style_to_string(&style),
        processed_inner
    );

    (html, gap_rule)
}

pub(super) fn process_container(attrs: &HashMap<String, String>, inner: &str) -> String {
    let style = extract_all_style_attributes(attrs);
    let mut td_style = HashMap::new();
    let mut table_style = HashMap::new();
    let visual_keys = [
        "background-color",
        "background-image",
        "background-size",
        "background-position",
        "background-repeat",
        "border",
        "border-top",
        "border-right",
        "border-bottom",
        "border-left",
        "border-color",
        "border-width",
        "border-style",
        "border-radius",
        "border-top-left-radius",
        "border-top-right-radius",
        "border-bottom-left-radius",
        "border-bottom-right-radius",
        "padding",
        "padding-top",
        "padding-right",
        "padding-bottom",
        "padding-left",
    ];

    for (key, value) in style {
        if visual_keys.contains(&key.as_str()) {
            td_style.insert(key, value);
        } else {
            table_style.insert(key, value);
        }
    }

    if [
        "border-radius",
        "border-top-left-radius",
        "border-top-right-radius",
        "border-bottom-left-radius",
        "border-bottom-right-radius",
    ]
    .iter()
    .any(|key| td_style.contains_key(*key))
    {
        table_style.insert("border-collapse".to_string(), "separate".to_string());
    }

    if let Some(width) = table_style.get("width").cloned() {
        if width != "100%" && width != "auto" {
            table_style.entry("max-width".to_string()).or_insert(width);
            table_style.insert("width".to_string(), "100%".to_string());
        }
    }

    // Container centering is deliberately opt-in so editor preview and email output agree.
    format!(
        r#"<table width="100%" border="0" cellPadding="0" cellSpacing="0" role="presentation" style="{}">
<tbody>
<tr style="width:100%">
<td style="{}">{}</td>
</tr>
</tbody>
</table>"#,
        style_to_string(&table_style),
        style_to_string(&td_style),
        inner
    )
}
