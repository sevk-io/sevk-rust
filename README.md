<p align="center">
  <img src="https://sevk.io/logo.png" alt="sevk" width="120" />
</p>

<h1 align="center">sevk Rust SDK</h1>

<p align="center">
  Official Rust SDK for <a href="https://sevk.io">sevk</a> email platform.
</p>

<p align="center">
  <a href="https://docs.sevk.io">Documentation</a> •
  <a href="https://sevk.io">Website</a>
</p>

## Installation

```toml
[dependencies]
sevk = "1.1.0"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Send Email

```rust
use sevk::{SendEmailRequest, Sevk};

#[tokio::main]
async fn main() {
    let client = Sevk::new("your-api-key");

    client.emails().send(SendEmailRequest {
        to: "recipient@example.com".into(),
        from: "hello@yourdomain.com".to_string(),
        subject: "Hello from sevk!".to_string(),
        html: Some("<h1>Welcome!</h1>".to_string()),
        ..Default::default()
    }).await.unwrap();
}
```

## Send Email with Markup

```rust
use sevk::{render, SendEmailRequest, Sevk};

#[tokio::main]
async fn main() {
    let client = Sevk::new("your-api-key");

    let html = render(r##"
      <section padding="40px 20px" background-color="#f8f9fa">
        <container max-width="600px">
          <heading level="1" color="#1a1a1a">Welcome!</heading>
          <paragraph color="#666666">Thanks for signing up.</paragraph>
          <button href="https://example.com" background-color="#5227FF" color="#ffffff" padding="12px 24px">
            Get Started
          </button>
        </container>
      </section>
    "##).unwrap();

    client.emails().send(SendEmailRequest {
        to: "recipient@example.com".into(),
        from: "hello@yourdomain.com".to_string(),
        subject: "Welcome!".to_string(),
        html: Some(html),
        ..Default::default()
    }).await.unwrap();
}
```

## Documentation

For full documentation, visit [docs.sevk.io](https://docs.sevk.io)

## License

MIT
