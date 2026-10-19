use sevk::{SendEmailRequest, SendEmailResponse, Sevk, SevkOptions};

#[tokio::test]
#[ignore = "requires send credentials; run cargo test --test send -- --ignored"]
async fn sends_a_markup_email_through_the_sevk_api() {
    let api_key = std::env::var("SEVK_API_KEY")
        .expect("SEVK_API_KEY, SEVK_TEST_FROM and SEVK_TEST_TO are required");
    let from = std::env::var("SEVK_TEST_FROM")
        .expect("SEVK_API_KEY, SEVK_TEST_FROM and SEVK_TEST_TO are required");
    let to = std::env::var("SEVK_TEST_TO")
        .expect("SEVK_API_KEY, SEVK_TEST_FROM and SEVK_TEST_TO are required");
    assert!(
        !api_key.is_empty() && !from.is_empty() && !to.is_empty(),
        "SEVK_API_KEY, SEVK_TEST_FROM and SEVK_TEST_TO are required"
    );

    let defaults = SevkOptions::default();
    let options = SevkOptions {
        base_url: std::env::var("SEVK_BASE_URL").unwrap_or(defaults.base_url),
        ..defaults
    };
    let client = Sevk::with_options(&api_key, options);
    let response = client
        .emails()
        .send(SendEmailRequest {
            from,
            to: to.into(),
            subject: "Sevk SDK send test".to_string(),
            markup: Some(
                "<section><paragraph>Hello from the Sevk Rust SDK</paragraph></section>"
                    .to_string(),
            ),
            ..SendEmailRequest::default()
        })
        .await
        .expect("the Sevk API accepts the email");

    match response {
        SendEmailResponse::Single(single) => assert!(!single.id.is_empty()),
        SendEmailResponse::Multiple(_) => panic!("expected a single email id"),
    }
}
