use std::time::Duration;
use tokio::time::sleep;

#[datex::main("../config.dx")]
async fn main(runtime: Runtime) {
    println!("Hello from DATEX! (version: {})", runtime.version());

    // print com hub connection information every 5 seconds
    loop {
        println!("ComHub Metadata: {:#?}", runtime.com_hub().get_metadata());
        sleep(Duration::from_secs(5)).await;
    }
}