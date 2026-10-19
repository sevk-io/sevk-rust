use std::collections::HashMap;

pub(super) fn parse_padding(style: &HashMap<String, String>) -> (i32, i32, i32, i32) {
    let mut pt = 0;
    let mut pr = 0;
    let mut pb = 0;
    let mut pl = 0;
    if let Some(padding) = style.get("padding") {
        let parts: Vec<&str> = padding.split_whitespace().collect();
        match parts.len() {
            1 => {
                let val = parse_px(parts[0]);
                pt = val;
                pr = val;
                pb = val;
                pl = val;
            }
            2 => {
                let vertical = parse_px(parts[0]);
                let horizontal = parse_px(parts[1]);
                pt = vertical;
                pr = horizontal;
                pb = vertical;
                pl = horizontal;
            }
            3 => {
                pt = parse_px(parts[0]);
                pr = parse_px(parts[1]);
                pb = parse_px(parts[2]);
                pl = parse_px(parts[1]);
            }
            4 => {
                pt = parse_px(parts[0]);
                pr = parse_px(parts[1]);
                pb = parse_px(parts[2]);
                pl = parse_px(parts[3]);
            }
            _ => {}
        }
    }
    if let Some(v) = style.get("padding-top") {
        pt = parse_px(v);
    }
    if let Some(v) = style.get("padding-right") {
        pr = parse_px(v);
    }
    if let Some(v) = style.get("padding-bottom") {
        pb = parse_px(v);
    }
    if let Some(v) = style.get("padding-left") {
        pl = parse_px(v);
    }
    (pt, pr, pb, pl)
}

pub(super) fn parse_px(s: &str) -> i32 {
    let value = s.trim();
    let end = value
        .char_indices()
        .take_while(|(index, ch)| {
            ch.is_ascii_digit() || (*index == 0 && (*ch == '+' || *ch == '-'))
        })
        .map(|(index, _)| index + 1)
        .last()
        .unwrap_or(0);
    value[..end].parse().unwrap_or(0)
}

pub(super) fn px_to_pt(px: i32) -> i32 {
    (px * 3 + 2).div_euclid(4)
}

pub(super) fn compute_font_width_and_space_count(expected_width: i32) -> (f64, i32) {
    if expected_width <= 0 {
        return (0.0, 0);
    }

    let space_count = ((expected_width as f64 / 10.0).ceil() as i32).clamp(1, 1000);
    (
        expected_width as f64 / space_count as f64 / 2.0,
        space_count,
    )
}

pub(super) fn parse_attributes(attrs_str: &str) -> HashMap<String, String> {
    let mut attrs = HashMap::new();
    let re = regex!(r#"([\w-]+)=(?:"([^"]*)"|'([^']*)')"#);

    for caps in re.captures_iter(attrs_str) {
        let value = caps
            .get(2)
            .or_else(|| caps.get(3))
            .map(|m| m.as_str().to_string())
            .unwrap_or_default();
        attrs.insert(caps[1].to_string(), value);
    }

    attrs
}

pub(super) fn extract_all_style_attributes(
    attrs: &HashMap<String, String>,
) -> HashMap<String, String> {
    let attrs: HashMap<String, String> = attrs
        .iter()
        .filter(|(_, value)| !value.is_empty())
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();
    let attrs = &attrs;
    let mut style = HashMap::new();

    if let Some(v) = attrs.get("text-color").or(attrs.get("color")) {
        style.insert("color".to_string(), v.clone());
    }
    if let Some(v) = attrs.get("background-color") {
        style.insert("background-color".to_string(), v.clone());
    }

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
        style.insert("background-image".to_string(), format!("url('{bg_img}')"));
        style
            .entry("background-size".to_string())
            .or_insert_with(|| "cover".to_string());
        style
            .entry("background-position".to_string())
            .or_insert_with(|| "center".to_string());
        style
            .entry("background-repeat".to_string())
            .or_insert_with(|| "no-repeat".to_string());
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

    if let Some(v) = attrs.get("padding") {
        style.insert("padding".to_string(), v.clone());
    }
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

    if let Some(v) = attrs.get("margin") {
        style.insert("margin".to_string(), v.clone());
    }
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

    if let Some(v) = attrs.get("border") {
        style.insert("border".to_string(), v.clone());
    }
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

    if let Some(v) = attrs.get("border-radius") {
        style.insert("border-radius".to_string(), v.clone());
    }
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

    style
}

pub(super) fn style_priority(key: &str) -> Option<i32> {
    Some(match key {
        "color" => 1,
        "background-color" => 2,
        "background-image" => 3,
        "background-size" => 4,
        "background-position" => 5,
        "background-repeat" => 6,
        "font-size" => 7,
        "font-family" => 8,
        "font-weight" => 9,
        "line-height" => 10,
        "text-align" => 11,
        "text-decoration" => 12,
        "width" => 13,
        "height" => 14,
        "max-width" => 15,
        "max-height" => 16,
        "min-width" => 17,
        "min-height" => 18,
        "padding" => 19,
        "padding-top" => 20,
        "padding-right" => 21,
        "padding-bottom" => 22,
        "padding-left" => 23,
        "margin" => 24,
        "margin-top" => 25,
        "margin-right" => 26,
        "margin-bottom" => 27,
        "margin-left" => 28,
        "border" => 29,
        "border-color" => 30,
        "border-width" => 31,
        "border-style" => 32,
        "border-top" => 33,
        "border-right" => 34,
        "border-bottom" => 35,
        "border-left" => 36,
        "border-radius" => 37,
        "border-top-left-radius" => 38,
        "border-top-right-radius" => 39,
        "border-bottom-left-radius" => 40,
        "border-bottom-right-radius" => 41,
        "border-collapse" => 42,
        "list-style-type" => 43,
        "vertical-align" => 44,
        _ => return None,
    })
}

pub(super) fn style_to_string(style: &HashMap<String, String>) -> String {
    let mut entries: Vec<(&String, &String)> = style
        .iter()
        .filter(|(_, value)| !value.is_empty())
        .collect();
    entries.sort_by(|a, b| {
        let pa = style_priority(a.0).unwrap_or(i32::MAX);
        let pb = style_priority(b.0).unwrap_or(i32::MAX);
        if pa != pb {
            pa.cmp(&pb)
        } else {
            a.0.cmp(b.0)
        }
    });
    entries
        .into_iter()
        .map(|(k, v)| format!("{k}:{v}"))
        .collect::<Vec<_>>()
        .join(";")
}
