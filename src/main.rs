use reqwest::Client;
use std::error::Error;
use std::process::Command;
use Tor_Traffic_Router::{is_tor_installed_unix, config_file ,is_tor_installed_windows, install_tor,stop_tor,tor_proxy};
use std::env;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Check if Tor is installed; logic varies by OS
    
    let text = format!("HiddenServiceDir {}
    HiddenServicePort 80 127.0.0.1:{}",env::current_dir().unwrap().to_str().unwrap(), 8080);
    
    let _ = config_file("etc/tor/torrc", &text);

    let Client = tor_proxy();

    // Example: Make a request through Tor
    let res = client.get("http://check.torproject.org/api/ip").send().await?;

    println!("Response: {:?}", res.text().await?);

    stop_tor();

    Ok(())
}
