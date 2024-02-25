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
