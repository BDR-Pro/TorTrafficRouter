use reqwest::Client;
use std::error::Error;
use std::process::Command;
use Tor_Traffic_Router::{is_tor_installed_unix, config_file ,is_tor_installed_windows, install_tor,stop_tor};
use std::env;
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
        let text = format!("HiddenServiceDir {}
                            HiddenServicePort 80 127.0.0.1:{}",env::current_dir().unwrap().to_str().unwrap(), 8080);
        let _ = config_file("etc/tor/torrc", &text);
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
