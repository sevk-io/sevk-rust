pub(super) fn find_hidden_preview_div(html: &str) -> Option<(String, String)> {
    let div_open = regex!(r"(?i)<div");
    let div_close = regex!(r"(?i)</div>");
    let hidden_style = regex!(r#"(?i)style=["'][^"']*display:\s*none"#);
    let mut position = 0;

    while let Some(found) = div_open.find_at(html, position) {
        let open = found.start();
        let close = open + 4 + html[open + 4..].find('>')?;
        let opening_tag = &html[open..=close];

        if hidden_style.is_match(opening_tag) {
            let closing = div_close.find_at(html, close + 1)?;
            return Some((
                html[close + 1..closing.start()].to_string(),
                html[open..closing.end()].to_string(),
            ));
        }

        position = close + 1;
    }

    None
}
