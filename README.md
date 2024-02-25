# `tor_traffic_router` Crate 🌐✨

Hey there, fellow digital explorer! 🚀 Ready to dive into the realms of privacy with a sprinkle of Rust magic? 🧙‍♂️💻 Welcome to `tor_traffic_router`, your go-to Rust crate for integrating Tor into your applications with ease and style. Perfect for the Gen Z coder who values privacy, loves open-source, and digs cool tech. Let's keep our internet adventures secure and anonymous, shall we?

## What's Inside the Box? 🎁

This crate is all about empowering your applications with Tor, the free and open-source software for enabling anonymous communication. Whether you're building the next big social media platform, a super-secret project, or just experimenting, `tor_traffic_router` has got your back.

### Features 🌟

- **Automatic Tor Installation**: Whether you're chilling on Windows, maxing out on macOS, or living the Linux life, `tor_traffic_router` auto-magically installs Tor for you. No fuss, no muss.
- **Cross-Platform Love**: Crafted with ❤️ for Linux, macOS, and Windows. We speak your OS language.
- **Tor Check & Management**: Easily check if Tor is installed and wave your wizard wand to start or stop it as needed. Because control is cool.
- **Privacy-Powered Requests**: Make HTTP requests through Tor with minimal code. Anonymous browsing mode: Activated!

## Getting Started 🚀

1. **Add `tor_traffic_router` to Your Cargo.toml**: Just like adding a new friend on social media, but for your project's dependencies.

```toml
[dependencies]
Tor_Traffic_Router = "0.1.0"
```

2. **Dive into the Code**: Check out `main.rs` for a shining example. From checking if Tor is installed, to making those stealthy web requests, we've got the blueprint for your privacy-focused adventures.

## Example Usage 📚

```rust
// Inside your main.rs or wherever your heart desires
mod tor_setup; 

use reqwest::Client;
use std::error::Error;
use std::process::Command;
use tor_setup::{is_tor_installed_unix, is_tor_installed_windows, install_tor,stop_tor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Check if Tor is installed; logic varies by OS
    let tor_installed = if cfg!(target_os = "windows") {
        is_tor_installed_windows()
    } else {
        is_tor_installed_unix()
    };

    if !tor_installed {
        println!("Tor is not installed. Installing...");
        install_tor();
    } else {
        println!("Tor is already installed. Proceeding...");
        // Start Tor
        Command::new("tor").spawn()?;
        
    }

    // Assuming Tor is now installed and configured to listen on the default SOCKS5 port
    let proxy = reqwest::Proxy::all("socks5://127.0.0.1:9050")?;
    let client = Client::builder().proxy(proxy).build()?;

    // Example: Make a request through Tor
    let res = client.get("http://check.torproject.org/api/ip").send().await?;

    println!("Response: {:?}", res.text().await?);

    stop_tor();

    Ok(())
}

```

` Response from the beyond: "{\"IsTor\":true,\"IP\":\"192.42.116.194\"}" `

## Why `tor_traffic_router`? 🤔

In an age where online privacy is as precious as the latest drop from your favorite brand, `tor_traffic_router` is here to keep your digital footprint as elusive as a ghost emoji. 🕵️‍♂️👻 Plus, it's built with the simplicity and efficiency that Gen Z coders value.

## Join the Movement 🌍

Grab `tor_traffic_router` and join the ranks of privacy defenders. Whether you're coding in a coffee shop, at a hackathon, or in your cozy room, let's make the internet a safer space for everyone.

**Remember**: With great power comes great responsibility. Use `tor_traffic_router` wisely and ethically. Happy coding! 🎉

---

P.S. Need help or wanna contribute? Find us on GitHub. Let's make `tor_traffic_router` even more awesome together!
