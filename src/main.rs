use std::error::Error;
use std::env;
use Tor_Traffic_Router::{tor_proxy, stop_tor, config_file};
use tokio; // Ensure Tokio is in your dependencies for async runtime.

const ASCII_ART: &str = r#"
  ______
  

/* //////////////////////////////////////////////////////////// */
/* //                                                        // */
/* //   _______ ____  _____      ____        _               // */
/* //  |__   __/ __ \|  __ \    / __ \      (_)              // */
/* //     | | | |  | | |__) |  | |  | |_ __  _  ___  _ __    // */
/* //     | | | |  | |  _  /   | |  | | '_ \| |/ _ \| '_ \   // */
/* //     | | | |__| | | \ \   | |__| | | | | | (_) | | | |  // */
/* //     |_|  \____/|_|  \_\ (_)____/|_| |_|_|\___/|_| |_|  // */
/* //                                                        // */
/* //////////////////////////////////////////////////////////// */


"#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Configure Tor with a new hidden service
        
    let text = format!(
        "HiddenServiceDir {}\nHiddenServicePort 80 127.0.0.1:{}",
        env::current_dir().unwrap().to_str().unwrap(),
        8080
    );
    //config_file("etc/tor/torrc", &text)?;

    
    println!(" {} Tor Traffic Router",ASCII_ART);

    // Attempt to create a Tor proxy client
    let client = tor_proxy()?;

    // Make a request through Tor
    get_req(client).await?;

    // Stop Tor service after the request
    stop_tor();

    Ok(())
}

async fn get_req(client: reqwest::Client) -> Result<(), Box<dyn Error>> {
    let res = client.get("http://check.torproject.org/api/ip").send().await?;
    println!("Response: {:?}", res.text().await?);

    Ok(())
}
