# Tor Traffic Router 🛡️

Welcome to **Tor Traffic Router**, a powerful Rust crate designed to make integrating Tor into your Rust applications as seamless as possible. With a strong focus on privacy, this crate ensures your network requests are enhanced with an additional layer of anonymity, leveraging the Tor network. Whether you're developing a new app that requires confidential communication or looking to add a privacy feature to an existing project, Tor Traffic Router has got you covered.

## Features 🌟

- **Automatic Tor Installation**: Effortlessly installs Tor on your machine, whether you're on Windows, macOS, or Linux.
- **Tor Management**: Easy-to-use functions to check if Tor is installed, and to start or stop the Tor service as required.
- **Privacy-Enhanced HTTP Requests**: Utilize the reqwest library configured to route through Tor, ensuring your application's network requests are private and secure.
- **Flexible Configuration**: Easily modify Tor configuration files directly from your Rust code, allowing for customized Tor services such as hidden services.

## Getting Started 🚀

### Prerequisites

Before diving into Tor Traffic Router, ensure you have Rust installed on your machine. This crate is compatible with Rust edition 2021 and requires no additional setup beyond the Rust toolchain.

### Installation

Add Tor Traffic Router to your `Cargo.toml`:

```toml
[dependencies]
Tor_Traffic_Router = "0.3.0"
```

### Basic Usage

Here's a quick example to get you started:

```rust
use Tor_Traffic_Router::{config_file, is_tor_installed_unix, is_tor_installed_windows, install_tor, stop_tor, tor_proxy};
use std::env;
use reqwest::Client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Configure Tor with a new hidden service
    let hidden_service_config = format!(
        "HiddenServiceDir {}\nHiddenServicePort 80 127.0.0.1:{}",
        env::current_dir().unwrap().to_str().unwrap(),
        8080
    );
    
    config_file("etc/tor/torrc", &hidden_service_config);

    // Ensure Tor is installed and start it
    let client = tor_proxy();

    // Make a privacy-focused request
    let res = client.get("http://check.torproject.org/api/ip").send().await?;
    println!("Response: {:?}", res.text().await?);

    stop_tor();

    Ok(())
}
```

## Documentation 📖

For more detailed information about each function and configuration options, please refer to the inline documentation within the code. Our GitHub repository also includes additional examples and usage scenarios to help you integrate Tor Traffic Router into your projects seamlessly.

## Contributing ✨

Contributions are what make the open-source community such a fantastic place to learn, inspire, and create. Any contributions you make are **greatly appreciated**. If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement". Don't forget to give the project a star! Thanks again!

## License 📄

Distributed under the MIT License. See `LICENSE` for more information.

## Acknowledgments 🙏

- Special thanks to the Tor Project for making online privacy accessible to everyone.
- Kudos to the Rust community for providing an ecosystem that makes secure and efficient programming achievable.

Join us in our journey to make the internet a safer, more private space for everyone. Happy coding! 🎉
