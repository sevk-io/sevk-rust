use std::collections::HashMap;

use super::error::RenderError;

pub(super) const MAX_MARKUP_DEPTH: usize = 64;
pub(super) const MAX_TAG_ITERATIONS: usize = 200;
pub(super) const MAX_RENDER_OUTPUT: usize = 5_000_000;
pub(super) const MAX_TAG_SCAN_WORK: usize = 30_000_000;
pub(super) const MAX_CODEBLOCK_LENGTH: usize = 2500;
pub(super) const MAX_TOTAL_CODEBLOCK_LENGTH: usize = 2500;
pub(super) const TOO_COMPLEX: &str = "Email markup is too complex to render";

const DEPTH_TAGS: [&str; 14] = [
    "block",
    "section",
    "column",
    "row",
    "container",
    "heading",
    "paragraph",
    "text",
    "button",
    "link",
    "sevk-link",
    "list",
    "li",
    "codeblock",
];

#[derive(Default)]
pub(super) struct RenderBudget {
    pub(super) scan_work: usize,
    pub(super) highlighted: usize,
}

impl RenderBudget {
    pub(super) fn charge_scan_work(&mut self, work: usize) -> Result<(), RenderError> {
        self.scan_work += work;
        if self.scan_work > MAX_TAG_SCAN_WORK {
            return Err(render_error(TOO_COMPLEX));
        }
        Ok(())
    }
}

pub(super) fn render_error(message: impl Into<String>) -> RenderError {
    RenderError(message.into())
}

fn is_tag_name_byte(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'-'
}

pub(super) fn assert_markup_depth(content: &str) -> Result<(), RenderError> {
    let bytes = content.as_bytes();
    let mut depths: HashMap<String, usize> = HashMap::new();
    let mut index = 0;

    while index < bytes.len() {
        let Some(open) = content[index..].find('<').map(|found| index + found) else {
            break;
        };
        let Some(close) = content[open + 1..].find('>').map(|found| open + 1 + found) else {
            break;
        };
        index = close + 1;

        let mut position = open + 1;
        let closing = bytes[position] == b'/';
        if closing {
            position += 1;
        }
        let mut name_end = position;
        while name_end < close && is_tag_name_byte(bytes[name_end]) {
            name_end += 1;
        }
        let name = content[position..name_end].to_ascii_lowercase();
        if !DEPTH_TAGS.contains(&name.as_str()) {
            continue;
        }

        let depth = depths.entry(name).or_insert(0);
        if closing {
            *depth = depth.saturating_sub(1);
        } else if bytes[close - 1] != b'/' {
            *depth += 1;
            if *depth > MAX_MARKUP_DEPTH {
                return Err(render_error(format!(
                    "Email markup nesting exceeds the maximum depth of {MAX_MARKUP_DEPTH}"
                )));
            }
        }
    }

    Ok(())
}
