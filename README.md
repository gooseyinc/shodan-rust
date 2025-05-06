# 🔍 shodan-rust

**An async-native Rust client for the [Shodan.io API](https://developer.shodan.io/).**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-Async-orange)](https://www.rust-lang.org/)
[![Build](https://img.shields.io/github/actions/workflow/status/gooseyinc/shodan-rust/ci.yml?branch=main)](https://github.com/gooseyinc/shodan-rust/actions)

`shodan-rust` is a lightweight and modern Rust crate for accessing Shodan's powerful search capabilities over the internet of things.

---

## ✨ Features

- ✅ Native async support using [`reqwest`](https://crates.io/crates/reqwest) and [`tokio`](https://tokio.rs/)
- 🔐 Easy API key integration
- 🌍 Search IP addresses, services, ports
- 🧠 Clean and extensible design for future Shodan endpoints
- 📁 Includes examples in `/examples` directory

---

## 🚀 Usage

### Add to your `Cargo.toml`

```toml
[dependencies]
shodan-rust = { git = "https://github.com/gooseyinc/shodan-rust" }
tokio = { version = "1", features = ["full"] }
```

### Example

```rust
use shodan_rust::ShodanClient;

#[tokio::main]
async fn main() {
    let client = ShodanClient::new("YOUR_SHODAN_API_KEY");

    match client.host_info("8.8.8.8").await {
        Ok(info) => println!("{:#?}", info),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

> 📎 Check the [`examples/`](examples) folder for more real-world usage.

---

## 🧪 Testing

```bash
export SHODAN_API_KEY=your_key_here
cargo test
```

---

## 📌 Roadmap

- [x] IP Lookup
- [x] Honeyscore (Shodan Addon)
- [ ] Search Query
- [ ] Hostnames & DNS Resolve
- [ ] Error handling improvements

---

## 🤝 Contributing

PRs, ideas and bug reports are welcome.  
Want to extend the library or implement more endpoints? Let’s build it together!

---

## 📄 License

Licensed under the MIT License.  
_Not affiliated with Shodan.io — this is a community-built Rust library._
