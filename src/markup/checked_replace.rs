use regex::{Captures, Regex};

use super::error::RenderError;

pub(super) fn replace_all_checked<F>(
    pattern: &Regex,
    input: &str,
    mut replace: F,
) -> Result<String, RenderError>
where
    F: FnMut(&Captures) -> Result<String, RenderError>,
{
    let mut failure = None;
    let output = pattern
        .replace_all(input, |captures: &Captures| {
            if failure.is_some() {
                return String::new();
            }
            match replace(captures) {
                Ok(value) => value,
                Err(error) => {
                    failure = Some(error);
                    String::new()
                }
            }
        })
        .into_owned();

    match failure {
        Some(error) => Err(error),
        None => Ok(output),
    }
}
