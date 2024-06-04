extern crate valorantapi;

use valorantapi::ValorantAPI;

#[tokio::main]
async fn main() {
    let valorant = ValorantAPI::new();
    //valorant.status();
    //let _ = valorant.request("val/status/v1/platform-data".to_string()).await;
    
    //let val = valorant.content().await;

    let val = valorant.ranked_by_act("4539cac3-47ae-90e5-3d01-b3812ca3274e".to_string()).await;
    println!("{}", val);
}
