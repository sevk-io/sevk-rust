use regex::Regex;

use super::content::{
    process_dividers, process_heading, process_images, process_link, process_list,
    process_list_item, process_paragraph, process_text,
};
use super::error::RenderError;
use super::layout::{process_column, process_container, process_row, process_section};
use super::limits::{assert_markup_depth, RenderBudget};
use super::mso::process_button;
use super::parser::process_tag;
use super::self_closing::replace_self_closing;
use super::styles::parse_attributes;
use super::syntax::process_codeblock;
use super::template::process_block_tag;

pub(super) fn process_markup(content: &str) -> Result<(String, String), RenderError> {
    assert_markup_depth(content)?;
    let mut budget = RenderBudget::default();

    let mut result = process_tag(
        content,
        "block",
        |attrs, inner, _| process_block_tag(&attrs, inner),
        &mut budget,
    )?;
    result = process_empty_blocks(&result)?;
    assert_markup_depth(&result)?;

    result = alias_link_tags(&result);
    result = process_tag(
        &result,
        "section",
        |attrs, inner, _| Ok(process_section(&attrs, inner)),
        &mut budget,
    )?;
    result = process_tag(
        &result,
        "column",
        |attrs, inner, _| Ok(process_column(&attrs, inner)),
        &mut budget,
    )?;

    let mut row_index = 0;
    let mut gap_rules = Vec::new();
    result = process_tag(
        &result,
        "row",
        |attrs, inner, _| {
            let (html, gap_rule) = process_row(&attrs, inner, row_index);
            row_index += 1;
            if let Some(rule) = gap_rule {
                gap_rules.push(rule);
            }
            Ok(html)
        },
        &mut budget,
    )?;

    result = process_tag(
        &result,
        "container",
        |attrs, inner, _| Ok(process_container(&attrs, inner)),
        &mut budget,
    )?;
    result = process_tag(
        &result,
        "heading",
        |attrs, inner, _| Ok(process_heading(&attrs, inner)),
        &mut budget,
    )?;
    result = process_tag(
        &result,
        "paragraph",
        |attrs, inner, _| Ok(process_paragraph(&attrs, inner)),
        &mut budget,
    )?;
    result = process_tag(
        &result,
        "text",
        |attrs, inner, _| Ok(process_text(&attrs, inner)),
        &mut budget,
    )?;
    result = process_tag(
        &result,
        "button",
        |attrs, inner, _| Ok(process_button(&attrs, inner)),
        &mut budget,
    )?;
    result = process_images(&result);
    result = process_dividers(&result);
    result = process_tag(
        &result,
        "sevk-link",
        |attrs, inner, _| Ok(process_link(&attrs, inner)),
        &mut budget,
    )?;
    result = process_tag(
        &result,
        "list",
        |attrs, inner, _| Ok(process_list(&attrs, inner)),
        &mut budget,
    )?;
    result = process_tag(
        &result,
        "li",
        |attrs, inner, _| Ok(process_list_item(&attrs, inner)),
        &mut budget,
    )?;
    result = process_tag(
        &result,
        "codeblock",
        |attrs, inner, budget| Ok(process_codeblock(&attrs, inner, budget)),
        &mut budget,
    )?;
    result = strip_markup_wrappers(&result);

    let gap_styles = if gap_rules.is_empty() {
        String::new()
    } else {
        format!(
            "@media only screen and (max-width:479px){{{}}}",
            gap_rules.join("")
        )
    };

    Ok((result.trim().to_string(), gap_styles))
}

fn alias_link_tags(content: &str) -> String {
    let result = regex!(r#"(?i)<link(\s+(?:[\w-]+=(?:"[^"]*"|'[^']*')\s+)*)href="#)
        .replace_all(content, "<sevk-link${1}href=")
        .to_string();
    regex!(r"(?i)</link>")
        .replace_all(&result, "</sevk-link>")
        .to_string()
}

fn process_empty_blocks(content: &str) -> Result<String, RenderError> {
    replace_self_closing(content, "block", true, |attributes| {
        process_block_tag(&parse_attributes(attributes), "")
    })
}

fn strip_markup_wrappers(content: &str) -> String {
    let stray_tags =
        regex!(r"(?i)</(?:container|section|row|column|heading|paragraph|text|button|sevk-link)>");
    let mut result = stray_tags.replace_all(content, "").to_string();

    for pattern in [
        r"(?i)<sevk-email[^>]*>",
        r"(?i)</sevk-email>",
        r"(?i)<sevk-body[^>]*>",
        r"(?i)</sevk-body>",
        r"(?i)<email[^>]*>",
        r"(?i)</email>",
        r"(?i)<mail[^>]*>",
        r"(?i)</mail>",
        r"(?i)<body[^>]*>",
        r"(?i)</body>",
    ] {
        result = Regex::new(pattern)
            .unwrap()
            .replace_all(&result, "")
            .to_string();
    }

    result
}
