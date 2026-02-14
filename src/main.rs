use std::time::Duration;
use tokio::time::sleep;
use datex::core::runtime::{Runtime};

#[datex::main("../config.dx")]
async fn main(runtime: Runtime) {
    flexi_logger::init();
    println!("Hello, DATEX - Version: {}", runtime.version);

    loop {
        // print com hub connection information every 5 seconds
        runtime.com_hub().print_metadata();
        sleep(Duration::from_secs(5)).await;
    }
}