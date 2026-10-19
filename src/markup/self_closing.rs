use regex::Regex;

use super::error::RenderError;

pub(super) fn replace_self_closing<F>(
    source: &str,
    tag: &str,
    self_close_only: bool,
    mut processor: F,
) -> Result<String, RenderError>
where
    F: FnMut(&str) -> Result<String, RenderError>,
{
    let open_re = Regex::new(&format!("(?i)<{tag}")).unwrap();
    let name_length = tag.len() + 1;
    let bytes = source.as_bytes();
    let mut output = String::new();
    let mut last_index = 0;
    let mut position = 0;

    while let Some(found) = open_re.find_at(source, position) {
        let open = found.start();
        let Some(close) = source[open + name_length..]
            .find('>')
            .map(|index| open + name_length + index)
        else {
            break;
        };
        position = close + 1;

        if self_close_only && bytes[close - 1] != b'/' {
            continue;
        }

        let attributes_end = if self_close_only { close - 1 } else { close };
        output.push_str(&source[last_index..open]);
        output.push_str(&processor(&source[open + name_length..attributes_end])?);
        last_index = close + 1;
    }

    output.push_str(&source[last_index..]);
    Ok(output)
}
