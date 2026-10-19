use regex::Regex;

use super::hidden_preview::find_hidden_preview_div;

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

pub(super) fn normalize_markup(content: &str) -> String {
    let mut result = content.to_string();

    if result.contains("<link") {
        let re = regex!(r#"<link(\s+(?:[\w-]+=(?:"[^"]*"|'[^']*')\s+)*)href="#);
        result = re.replace_all(&result, "<sevk-link${1}href=").to_string();
        result = result.replace("</link>", "</sevk-link>");
    }

    if !result.contains("<sevk-email") && !result.contains("<email") && !result.contains("<mail") {
        result = format!("<mail><body>{result}</body></mail>");
    }

    result
}

pub(super) fn generate_font_links(fonts: &[FontConfig]) -> String {
    fonts
        .iter()
        .map(|f| {
            format!(
                r#"<link href="{}" rel="stylesheet" type="text/css" />"#,
                f.url
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Parse email HTML and extract head settings
pub fn parse_email_html(content: &str) -> ParsedEmailContent {
    if regex!(r"(?i)<(?:email|mail)[\s>]").is_match(content) {
        parse_sevk_markup(content)
    } else {
        parse_html_document(content)
    }
}

fn parse_html_document(content: &str) -> ParsedEmailContent {
    let mut head_settings = EmailHeadSettings::default();
    let title_re = regex!(r"(?i)<title[^>]*>([\s\S]*?)</title>");
    if let Some(caps) = title_re.captures(content) {
        head_settings.title = Some(caps[1].trim().to_string());
    }

    let style_re = regex!(r"(?i)<style[^>]*>([\s\S]*?)</style>");
    let styles = style_re
        .captures_iter(content)
        .map(|caps| caps[1].to_string())
        .collect::<Vec<_>>();
    if !styles.is_empty() {
        head_settings.styles = Some(styles.join("\n"));
    }

    let font_re = regex!(r#"(?i)<link\b[^>]*href=["']([^"']*fonts[^"']*)["'][^>]*>"#);
    let family_re = regex!(r"family=([^:&]+)");
    for caps in font_re.captures_iter(content) {
        let index = head_settings.fonts.len();
        head_settings.fonts.push(FontConfig {
            id: format!("font-{index}"),
            name: family_re
                .captures(&caps[1])
                .map(|family| family[1].replace('+', " "))
                .unwrap_or_else(|| format!("Font {}", index + 1)),
            url: caps[1].to_string(),
        });
    }

    let body_re = regex!(r"(?i)<body[^>]*>([\s\S]*?)</body>");
    let mut body = body_re
        .captures(content)
        .map(|caps| caps[1].to_string())
        .unwrap_or_else(|| content.to_string());
    if let Some((inner, full)) = find_hidden_preview_div(content) {
        head_settings.preview_text = Some(inner.trim().to_string());
        body = body.replacen(&full, "", 1);
    }

    ParsedEmailContent {
        body: body.trim().to_string(),
        head_settings,
    }
}

fn parse_sevk_markup(content: &str) -> ParsedEmailContent {
    let mut head_settings = EmailHeadSettings::default();

    let root_caps = regex!(r"(?i)<email([^>]*)>")
        .captures(content)
        .or_else(|| regex!(r"(?i)<mail([^>]*)>").captures(content));
    if let Some(caps) = root_caps {
        let root_attrs = &caps[1];
        let lang_re = regex!(r#"(?i)lang=["']([^"']*)["']"#);
        let dir_re = regex!(r#"(?i)dir=["']([^"']*)["']"#);
        if let Some(lang_caps) = lang_re.captures(root_attrs) {
            head_settings.lang = Some(lang_caps[1].to_string());
        }
        if let Some(dir_caps) = dir_re.captures(root_attrs) {
            head_settings.dir = Some(dir_caps[1].to_string());
        }
    }

    let title_re = regex!(r"(?i)<title[^>]*>([\s\S]*?)</title>");
    if let Some(caps) = title_re.captures(content) {
        head_settings.title = Some(caps[1].trim().to_string());
    }

    let preview_re = regex!(r"(?i)<preview[^>]*>([\s\S]*?)</preview>");
    if let Some(caps) = preview_re.captures(content) {
        head_settings.preview_text = Some(caps[1].trim().to_string());
    }

    let style_re = regex!(r"(?i)<style[^>]*>([\s\S]*?)</style>");
    if let Some(caps) = style_re.captures(content) {
        head_settings.styles = Some(caps[1].trim().to_string());
    }

    let font_re = regex!(r"(?i)<font([^>]*)>");
    let font_name_re = regex!(r#"(?i)name=["']([^"']*)["']"#);
    let font_url_re = regex!(r#"(?i)url=["']([^"']*)["']"#);
    for caps in font_re.captures_iter(content) {
        let (name, url) = match (
            font_name_re.captures(&caps[1]),
            font_url_re.captures(&caps[1]),
        ) {
            (Some(name), Some(url)) => (name, url),
            _ => continue,
        };
        if head_settings.fonts.iter().any(|font| font.url == url[1]) {
            continue;
        }
        head_settings.fonts.push(FontConfig {
            id: format!("font-{}", head_settings.fonts.len()),
            name: name[1].to_string(),
            url: url[1].to_string(),
        });
    }

    let body_re = regex!(r"(?i)<body[^>]*>([\s\S]*?)</body>");
    let body_content = body_re
        .captures(content)
        .map(|caps| caps[1].trim().to_string())
        .filter(|body| !body.is_empty());
    let body = if let Some(body) = body_content {
        body
    } else {
        let mut body = content.to_string();
        let patterns = [
            r"<(?:email|mail)[^>]*>",
            r"</(?:email|mail)>",
            r"<head[^>]*>[\s\S]*?</head>",
            r"<title[^>]*>[\s\S]*?</title>",
            r"<preview[^>]*>[\s\S]*?</preview>",
            r"<style[^>]*>[\s\S]*?</style>",
            r"<font[^>]*>",
            r"</font>",
        ];
        for pattern in patterns {
            let re = Regex::new(&format!("(?i){pattern}")).unwrap();
            body = re.replace_all(&body, "").to_string();
        }
        body.trim().to_string()
    };

    ParsedEmailContent {
        body,
        head_settings,
    }
}
