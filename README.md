# disposable-email-domains

> 🛡️ A Rust library to detect and block disposable/temporary email addresses during user registration.

`disposable-email-domains` is a simple and efficient Rust library that helps developers identify whether an email address belongs to a known disposable (temporary) email provider. This is particularly useful in preventing spam registrations or enforcing real-user policies in your application.

## ✨ Features

- 🚀 Fast email domain lookup
- 🧠 Built-in list of disposable email providers
- 🔄 Supports automatic updates (optional)
- ✅ Simple API and easy integration

## 📦 Installation

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
disposable-email-domains = "0.1"
```

Or get the latest from GitHub:

```toml
[dependencies]
disposable-email-domains = { git = "https://github.com/houzhongjian/disposable-email-domains" }
```

## 🔧 Usage

```rust
use disposable_email_domains::is_disposable;

fn main() {
    let email = "user@mailinator.com";
    if is_disposable(email) {
        println!("Blocked: Disposable email detected.");
    } else {
        println!("Allowed: Email is not disposable.");
    }
}
```

### Example Output

```sh
Blocked: Disposable email detected.
```

## 📚 API

### `is_disposable(email: &str) -> bool`

Checks if the provided email address uses a disposable email domain.

- **email**: A full email address like `example@mailinator.com`
- **returns**: `true` if it's disposable, `false` otherwise.


You can also contribute more domains or override the list if needed.

## 🤝 Contributing

Contributions, issues, and feature requests are welcome!  
Feel free to check the [issues page](https://github.com/houzhongjian/disposable-email-domains/issues) or submit a pull request.

### To update the domain list:

```bash
cargo run --example update_list
```

## ⚖️ License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

---

Made with ❤️ by [Temp Mail](https://tempmail100.com)