use nmrs::NetworkManager;

#[tokio::main]
async fn main() -> nmrs::Result<()> {
    let nm = NetworkManager::new().await?;
    
    // Scan for networks
    let devices = nm.list_devices().await?;
    let networks = nm.list_networks().await?;
    
    for net in networks {
        println!(
            "{} - Signal: {}%, Security: {:?}, Device: {}",
            net.ssid,
            net.strength.unwrap_or(0),
            net.secured,
            net.bssid.unwrap_or(String::new())
        );
    }

    for dev in devices {
        println!(
            "{}",
            dev
        )
    }
    
    Ok(())
}
