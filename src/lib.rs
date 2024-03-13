use std::process::Command;
use sysinfo::System;
use std::fs::OpenOptions;
use std::io::Write;


pub fn install_tor() {
    #[cfg(target_os = "linux")]
    let (pkg_manager, update_cmd, install_cmd) = ("sudo apt-get", "update", "install tor -y");

    #[cfg(target_os = "macos")]
    let (pkg_manager, update_cmd, install_cmd) = ("brew", "update", "install tor");

    #[cfg(target_os = "windows")]
    {
        if !is_chocolatey_installed() {
            println!("Chocolatey not found. Installing Chocolatey...");
            install_chocolatey();
        }
        println!("Installing Tor using Chocolatey...");
        let status_install = Command::new("powershell")
            .args(["-Command", "choco install tor -y"])
            .status()
            .expect("Failed to install Tor via Chocolatey");

        if status_install.success() {
            println!("Tor has been successfully installed.");
        } else {
            println!("Failed to install Tor.");
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        let status_update = Command::new(pkg_manager)
            .arg(update_cmd)
            .status()
            .expect("Failed to update package list");

        if status_update.success() {
            let status_install = Command::new("sh")
                .arg("-c")
                .arg(format!("{} {}", pkg_manager, install_cmd))
                .status()
                .expect("Failed to install Tor");

            if status_install.success() {
                println!("Tor has been successfully installed.");
            } else {
                println!("Failed to install Tor.");
            }
        } else {
            println!("Failed to update package list.");
        }
    }
}

pub fn is_tor_installed_unix() -> bool {
    Command::new("sh")
        .arg("-c")
        .arg("tor --version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(target_os = "windows")]
pub fn is_tor_installed_windows() -> bool {
    if !is_chocolatey_installed() {
        return false;
    }
    if !is_admin(){
        panic!("Please run this program as an administrator to check if Tor is installed.");
    }
    Command::new("powershell")
        .args(["-Command", "tor --version"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(target_os = "windows")]
pub fn is_chocolatey_installed() -> bool {
    Command::new("powershell")
        .args(["-Command", "choco -v"])
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(target_os = "windows")]
pub fn install_chocolatey() {
    if !is_admin(){
        panic!("Please run this program as an administrator to install Chocolatey.");
    }
    let choco_install_script = r#"Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))"#;
    let status = Command::new("powershell")
        .args(["-Command", choco_install_script])
        .status()
        .expect("Failed to install Chocolatey");

    if !status.success() {
        panic!("Failed to install Chocolatey. Please install it manually.");
    }
    println!("Chocolatey installed successfully.");
}


pub fn is_admin() -> bool {
    Command::new("powershell")
        .args(["-Command", "(New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)"])
        .output()
        .map(|output| {
            let output_str = String::from_utf8_lossy(&output.stdout);
            output_str.trim() == "True"
        })
        .unwrap_or(false)
}



pub fn stop_tor() {
    let mut system = System::new_all();
    system.refresh_all();

    let processes = system.processes();
    let tor_process_name = if cfg!(target_os = "windows") { "tor.exe" } else { "tor" };

    for (pid, process) in processes {
        if process.name().to_lowercase().contains(tor_process_name) {
            println!("Tor process found (PID: {}), attempting to stop...", pid);
            #[cfg(target_os = "windows")]
            {
                if let Err(e) = std::process::Command::new("taskkill")
                    .args(&["/PID", &pid.to_string(), "/F"])
                    .output()
                {
                    println!("Failed to kill Tor process: {:?}", e);
                } else {
                    println!("Tor process successfully stopped.");
                }
            }
            #[cfg(not(target_os = "windows"))]
            {
                if let Err(e) = std::process::Command::new("kill")
                    .arg("-9")
                    .arg(pid.to_string())
                    .output()
                {
                    println!("Failed to kill Tor process: {:?}", e);
                } else {
                    println!("Tor process successfully stopped.");
                }
            }
        }
    }
}


pub fn config_file(file_path: &str, text: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true) // Give write permission.
        .append(true) // Set the file to append mode.
        .create(true) // Create the file if it does not exist.
        .open(file_path)?; // Open the file, return the error if there's a problem.

    writeln!(file, "{}", text)?; // Write the text to the file with a new line.
    Ok(())
}



pub fn tor_proxy() -> reqwest::Client{
    
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
        // Start Tor and print the output
        Command::new("tor").spawn()?;
        let proxy = reqwest::Proxy::all("socks5://127.0.0.1:9050")?;
        let client = Client::builder().proxy(proxy).build()?;
        
        return client;
    }

}
