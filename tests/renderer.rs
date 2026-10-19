use sevk::render;

#[test]
fn renders_markup_into_an_html_email_document() {
    let html =
        render("<section><paragraph>Hello world</paragraph></section>").expect("markup renders");

    assert!(html.starts_with("<!DOCTYPE html PUBLIC"));
    assert!(html.contains("Hello world"));
    assert!(!html.contains("<paragraph>"));
}
