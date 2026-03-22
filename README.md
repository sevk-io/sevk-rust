<p align="center">
  <img src="https://sevk.io/logo.png" alt="Sevk" width="120" />
</p>

<h1 align="center">Sevk Rust SDK</h1>

<p align="center">
  Official Rust SDK for <a href="https://sevk.io">Sevk</a> email platform.
</p>

<p align="center">
  <a href="https://docs.sevk.io">Documentation</a> •
  <a href="https://sevk.io">Website</a>
</p>

## Installation

```toml
[dependencies]
sevk = "1.0.0"
```

## Send Email

```rust
use sevk::Sevk;

#[tokio::main]
async fn main() {
    let client = Sevk::new("your-api-key");

    client.emails().send(
        "recipient@example.com",
        "hello@yourdomain.com",
        "Hello from Sevk!",
        "<h1>Welcome!</h1>",
    ).await.unwrap();
}
```

## Send Email with Markup

```rust
use sevk::{Sevk, markup};

#[tokio::main]
async fn main() {
    let client = Sevk::new("your-api-key");

    let html = markup::render(r#"
      <section padding="40px 20px" background-color="#f8f9fa">
        <container max-width="600px">
          <heading level="1" color="#1a1a1a">Welcome!</heading>
          <paragraph color="#666666">Thanks for signing up.</paragraph>
          <button href="https://example.com" background-color="#5227FF" color="#ffffff" padding="12px 24px">
            Get Started
          </button>
        </container>
      </section>
    "#);

    client.emails().send(
        "recipient@example.com",
        "hello@yourdomain.com",
        "Welcome!",
        &html,
    ).await.unwrap();
}
```

## Documentation

For full documentation, visit [docs.sevk.io](https://docs.sevk.io)

## License

MIT
