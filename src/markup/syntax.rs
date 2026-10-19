use super::limits::{RenderBudget, MAX_CODEBLOCK_LENGTH, MAX_TOTAL_CODEBLOCK_LENGTH};
use super::styles::{extract_all_style_attributes, style_priority, style_to_string};
use super::themes::{resolve_code_theme, CodeTheme};
use std::collections::HashMap;
use std::str::FromStr;
use syntect::easy::HighlightLines;
use syntect::highlighting::{
    Color, FontStyle, ScopeSelectors, Style, StyleModifier, Theme, ThemeItem, ThemeSettings,
};
use syntect::util::LinesWithEndings;

const TOKEN_SPACE: &str = "\u{00A0}\u{200D}\u{200B}";

const LANGUAGES: &[&str] = &[
    "plain",
    "extend",
    "plaintext",
    "text",
    "txt",
    "markup",
    "html",
    "mathml",
    "svg",
    "xml",
    "ssml",
    "atom",
    "rss",
    "css",
    "clike",
    "javascript",
    "js",
    "typescript",
    "ts",
    "jsx",
    "tsx",
    "python",
    "py",
    "java",
    "c",
    "cpp",
    "csharp",
    "cs",
    "dotnet",
    "php",
    "ruby",
    "rb",
    "go",
    "rust",
    "sql",
    "bash",
    "sh",
    "shell",
    "json",
    "webmanifest",
    "yaml",
    "yml",
    "markdown",
    "md",
];

const SCOPE_TOKENS: &[(&str, &str)] = &[
    ("comment", "comment"),
    ("keyword", "keyword"),
    ("keyword.operator", "operator"),
    ("storage", "keyword"),
    ("string", "string"),
    ("string.regexp", "regex"),
    ("constant.numeric", "number"),
    ("constant.language", "boolean"),
    ("constant.character.escape", "string"),
    ("constant.other", "constant"),
    ("variable", "variable"),
    ("entity.name.function", "function"),
    ("support.function", "function"),
    ("meta.function-call", "function"),
    ("entity.name.type", "class-name"),
    ("entity.name.class", "class-name"),
    ("support.type", "class-name"),
    ("support.class", "class-name"),
    ("entity.name.tag", "tag"),
    ("entity.other.attribute-name", "attr-name"),
    ("entity.name.namespace", "namespace"),
    ("punctuation", "punctuation"),
    ("meta.selector", "selector"),
    ("support.constant", "constant"),
    ("markup.heading", "important"),
    ("markup.bold", "bold"),
    ("markup.italic", "italic"),
    ("markup.inserted", "inserted"),
    ("markup.deleted", "deleted"),
];

pub(super) fn process_codeblock(
    attrs: &HashMap<String, String>,
    code: &str,
    budget: &mut RenderBudget,
) -> String {
    let requested = attrs
        .get("language")
        .filter(|value| !value.is_empty())
        .map(|value| value.to_lowercase())
        .unwrap_or_else(|| "javascript".to_string());
    let language = match requested.as_str() {
        "jsx" => "javascript",
        "tsx" => "typescript",
        other => other,
    };
    let theme_name = attrs
        .get("theme")
        .filter(|value| !value.is_empty())
        .map(String::as_str)
        .unwrap_or("oneDark");
    let theme = resolve_code_theme(theme_name);
    let extracted_style = extract_all_style_attributes(attrs);
    let custom_style = ordered_style(&extracted_style);

    if code.is_empty() {
        return "<pre><code></code></pre>".to_string();
    }

    let length = code.encode_utf16().count();
    if !LANGUAGES.contains(&language)
        || length > MAX_CODEBLOCK_LENGTH
        || budget.highlighted + length > MAX_TOTAL_CODEBLOCK_LENGTH
    {
        let style = pre_style(&[theme.base, &[("width", "100%")], &custom_style]);
        let escaped = code.replace('<', "&lt;").replace('>', "&gt;");
        return format!(r#"<pre style="{style}"><code>{escaped}</code></pre>"#);
    }
    budget.highlighted += length;

    let mut inherited = HashMap::new();
    if let Some(font_family) = attrs.get("font-family").filter(|value| !value.is_empty()) {
        inherited.insert("font-family".to_string(), font_family.clone());
    }
    let numbered = attrs.get("line-numbers").map(String::as_str) == Some("true");
    let html = highlight_lines(code, language, theme_name, theme, &inherited)
        .iter()
        .enumerate()
        .map(|(index, line)| {
            let number = if numbered {
                format!(
                    r#"<span style="width:2em;display:inline-block">{}</span>"#,
                    index + 1
                )
            } else {
                String::new()
            };
            format!(r#"<p style="margin:0;min-height:1em">{number}{line}</p>"#)
        })
        .collect::<String>();
    let style = pre_style(&[
        theme.base,
        &[("width", "100%"), ("box-sizing", "border-box")],
        &custom_style,
    ]);
    format!(r#"<pre style="{style}"><code>{html}</code></pre>"#)
}

fn ordered_style(style: &HashMap<String, String>) -> Vec<(&str, &str)> {
    let mut entries: Vec<(&str, &str)> = style
        .iter()
        .map(|(key, value)| (key.as_str(), value.as_str()))
        .collect();
    entries.sort_by_key(|(key, _)| style_priority(key).unwrap_or(i32::MAX));
    entries
}

fn pre_style(layers: &[&[(&str, &str)]]) -> String {
    let mut merged: Vec<(&str, &str)> = Vec::new();
    for layer in layers {
        for (key, value) in layer.iter() {
            match merged.iter_mut().find(|(existing, _)| existing == key) {
                Some(entry) => entry.1 = value,
                None => merged.push((key, value)),
            }
        }
    }
    merged
        .iter()
        .map(|(key, value)| format!("{}:{}", kebab_property(key), value))
        .collect::<Vec<_>>()
        .join(";")
}

fn kebab_property(key: &str) -> String {
    let mut property = String::new();
    for ch in key.chars() {
        if ch.is_ascii_uppercase() {
            property.push('-');
            property.push(ch.to_ascii_lowercase());
        } else {
            property.push(ch);
        }
    }
    if property.starts_with("ms-") {
        property.insert(0, '-');
    }
    property
}

fn highlight_lines(
    code: &str,
    language: &str,
    theme_name: &str,
    theme: &CodeTheme,
    inherited: &HashMap<String, String>,
) -> Vec<String> {
    let inherited_style = style_to_string(inherited);
    let tokens = tokenize(code, language, theme_name, theme, inherited)
        .unwrap_or_else(|| vec![(inherited_style.clone(), code.to_string())]);
    let mut lines = vec![String::new()];

    for (style, text) in tokens {
        for (index, part) in text.split('\n').enumerate() {
            if index > 0 {
                lines.push(String::new());
            }
            if part.is_empty() {
                continue;
            }
            let escaped = part
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;")
                .replace('"', "&quot;")
                .replace('\'', "&#39;")
                .replace(' ', TOKEN_SPACE);
            let line = lines.last_mut().unwrap();
            if style.is_empty() {
                line.push_str(&escaped);
            } else {
                line.push_str(&format!(r#"<span style="{style}">{escaped}</span>"#));
            }
        }
    }

    lines
}

fn tokenize(
    code: &str,
    language: &str,
    theme_name: &str,
    theme: &CodeTheme,
    inherited: &HashMap<String, String>,
) -> Option<Vec<(String, String)>> {
    let syntax_name = match language {
        "markup" => "html",
        "mathml" | "svg" | "ssml" | "atom" | "rss" => "xml",
        "csharp" | "dotnet" => "cs",
        "webmanifest" => "json",
        "shell" => "sh",
        "plain" | "plaintext" | "text" | "txt" | "clike" => return None,
        other => other,
    };
    let syntax_set = two_face::syntax::extra_newlines();
    let syntax = syntax_set
        .find_syntax_by_token(syntax_name)
        .or_else(|| syntax_set.find_syntax_by_extension(syntax_name))?;
    let syntect_theme = syntect_theme(theme);
    let default_foreground = syntect_theme.settings.foreground?;
    let mut highlighter = HighlightLines::new(syntax, &syntect_theme);
    let mut tokens = Vec::new();

    for line in LinesWithEndings::from(code) {
        for (style, text) in highlighter.highlight_line(line, &syntax_set).ok()? {
            tokens.push((
                inline_style(style, default_foreground, theme_name, inherited),
                text.to_string(),
            ));
        }
    }

    Some(tokens)
}

fn syntect_theme(theme: &CodeTheme) -> Theme {
    let base_color = |key: &str| {
        theme
            .base
            .iter()
            .find(|(name, _)| *name == key)
            .and_then(|(_, value)| parse_color(value))
    };
    let scopes = SCOPE_TOKENS
        .iter()
        .filter_map(|(scope, token)| {
            let style = theme.token(token);
            let foreground = style
                .iter()
                .find(|(name, _)| *name == "color")
                .and_then(|(_, value)| parse_color(value));
            let mut font_style = FontStyle::empty();
            if style.contains(&("font-weight", "bold")) {
                font_style |= FontStyle::BOLD;
            }
            if style.contains(&("font-style", "italic")) {
                font_style |= FontStyle::ITALIC;
            }
            if foreground.is_none() && font_style.is_empty() {
                return None;
            }
            Some(ThemeItem {
                scope: ScopeSelectors::from_str(scope).ok()?,
                style: StyleModifier {
                    foreground,
                    background: None,
                    font_style: Some(font_style),
                },
            })
        })
        .collect();

    Theme {
        name: None,
        author: None,
        settings: ThemeSettings {
            foreground: base_color("color"),
            background: base_color("backgroundColor"),
            ..ThemeSettings::default()
        },
        scopes,
    }
}

fn parse_color(value: &str) -> Option<Color> {
    let hex = value.strip_prefix('#')?;
    let expand = |pair: &str| u8::from_str_radix(pair, 16).ok();
    let (r, g, b) = match hex.len() {
        6 => (
            expand(&hex[0..2])?,
            expand(&hex[2..4])?,
            expand(&hex[4..6])?,
        ),
        3 => (
            expand(&hex[0..1].repeat(2))?,
            expand(&hex[1..2].repeat(2))?,
            expand(&hex[2..3].repeat(2))?,
        ),
        _ => return None,
    };
    Some(Color { r, g, b, a: 255 })
}

fn inline_style(
    style: Style,
    default_foreground: Color,
    theme_name: &str,
    inherited: &HashMap<String, String>,
) -> String {
    let mut merged = inherited.clone();
    if style.foreground != default_foreground {
        let hex = format!(
            "#{:02x}{:02x}{:02x}",
            style.foreground.r, style.foreground.g, style.foreground.b
        );
        merged.insert("color".to_string(), theme_color_text(theme_name, &hex));
    }
    if style.font_style.contains(FontStyle::BOLD) {
        merged.insert("font-weight".to_string(), "bold".to_string());
    }
    if style.font_style.contains(FontStyle::ITALIC) {
        merged.insert("font-style".to_string(), "italic".to_string());
    }
    style_to_string(&merged)
}

fn theme_color_text(theme_name: &str, hex: &str) -> String {
    resolve_code_theme(theme_name)
        .tokens
        .iter()
        .flat_map(|(_, style)| style.iter())
        .filter(|(name, _)| *name == "color")
        .map(|(_, value)| *value)
        .find(|value| value.eq_ignore_ascii_case(hex) || parse_color(value) == parse_color(hex))
        .unwrap_or(hex)
        .to_string()
}
