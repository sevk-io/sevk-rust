#[macro_use]
mod regex_cache;
mod checked_replace;
mod content;
mod document;
mod error;
mod hidden_preview;
mod layout;
mod limits;
mod mso;
mod number;
mod parser;
mod processor;
mod self_closing;
mod styles;
mod syntax;
mod template;
mod themes;

use document::{generate_font_links, normalize_markup};
use processor::process_markup;

pub use document::{parse_email_html, EmailHeadSettings, FontConfig, ParsedEmailContent};
pub use error::RenderError;
/// Render sevk markup to email-compatible HTML
pub fn render(html_content: &str) -> Result<String, RenderError> {
    render_document(html_content, None)
}

fn render_document(
    html_content: &str,
    head_settings: Option<EmailHeadSettings>,
) -> Result<String, RenderError> {
    let parsed = parse_email_html(html_content);
    let settings = head_settings.unwrap_or(parsed.head_settings);
    let content_to_process = parsed.body;

    let normalized = normalize_markup(&content_to_process);
    let (processed, gap_styles) = process_markup(&normalized)?;

    let title_tag = settings
        .title
        .as_ref()
        .map(|t| format!("<title>{t}</title>"))
        .unwrap_or_default();

    let font_links = generate_font_links(&settings.fonts);

    let custom_styles = settings
        .styles
        .as_ref()
        .map(|s| format!("<style type=\"text/css\">{s}</style>"))
        .unwrap_or_default();

    let gap_style_tag = if gap_styles.is_empty() {
        String::new()
    } else {
        format!("<style type=\"text/css\">{gap_styles}</style>")
    };

    let preview_text = settings.preview_text.as_ref()
        .map(|p| format!(r#"<div style="display:none;font-size:1px;color:#ffffff;line-height:1px;max-height:0px;max-width:0px;opacity:0;overflow:hidden;">{p}</div>"#))
        .unwrap_or_default();

    let lang = settings.lang.as_deref().unwrap_or("en");
    let dir = settings.dir.as_deref().unwrap_or("ltr");

    Ok(format!(
        r#"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">
<html lang="{lang}" dir="{dir}" xmlns="http://www.w3.org/1999/xhtml" xmlns:v="urn:schemas-microsoft-com:vml" xmlns:o="urn:schemas-microsoft-com:office:office">
<head>
<meta content="text/html; charset=UTF-8" http-equiv="Content-Type"/>
<meta name="viewport" content="width=device-width, initial-scale=1.0"/>
<meta name="x-apple-disable-message-reformatting"/>
<meta content="IE=edge" http-equiv="X-UA-Compatible"/>
<meta name="format-detection" content="telephone=no,address=no,email=no,date=no,url=no"/>
<!--[if mso]>
<noscript>
<xml>
<o:OfficeDocumentSettings>
<o:AllowPNG/>
<o:PixelsPerInch>96</o:PixelsPerInch>
</o:OfficeDocumentSettings>
</xml>
</noscript>
<![endif]-->
<style type="text/css">
#outlook a {{ padding: 0; }}
body {{ margin: 0; padding: 0; -webkit-text-size-adjust: 100%; -ms-text-size-adjust: 100%; }}
table, td {{ border-collapse: collapse; mso-table-lspace: 0pt; mso-table-rspace: 0pt; }}
.sevk-row-table {{ border-collapse: separate !important; }}
img {{ border: 0; height: auto; line-height: 100%; outline: none; text-decoration: none; -ms-interpolation-mode: bicubic; }}
@media only screen and (max-width: 479px) {{
  .sevk-row-table {{ width: 100% !important; }}
  .sevk-column {{ display: block !important; width: 100% !important; max-width: 100% !important; box-sizing: border-box !important; }}
}}
</style>
{gap_style_tag}
{title_tag}
{font_links}
{custom_styles}
</head>
<body style="margin:0;padding:0;word-spacing:normal;-webkit-text-size-adjust:100%;-ms-text-size-adjust:100%;font-family:ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;">
<div aria-roledescription="email" role="article">
{preview_text}
{processed}
</div>
</body>
</html>"#
    ))
}
