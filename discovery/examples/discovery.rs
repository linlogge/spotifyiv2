use futures::StreamExt;
use spotifyiv2_core::SessionConfig;
use spotifyiv2_discovery::DeviceType;
use sha1::{Digest, Sha1};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let name = "spotifyiv2";
    let device_id = hex::encode(Sha1::digest(name.as_bytes()));

    let mut server =
        spotifyiv2_discovery::Discovery::builder(device_id, SessionConfig::default().client_id)
            .name(name)
            .device_type(DeviceType::Computer)
            .launch()
            .unwrap();

    while let Some(x) = server.next().await {
        println!("Received {:?}", x);
    }
}
