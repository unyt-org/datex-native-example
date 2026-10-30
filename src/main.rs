#![feature(yield_expr)]
#![feature(gen_blocks)]

use std::time::Duration;
use datex::core::datex_registry::all_datex_impl_registrations;
use datex::core::macros::{datex, Datex};
use tokio::time::sleep;
use datex::core::runtime::Runtime;

#[datex::main("../config.dx")]
async fn main(runtime: Runtime) {
    flexi_logger::init();
    println!("Hello from DATEX! (version: {})", runtime.version());

    println!("IMPLS: {:#?}", all_datex_impl_registrations().collect::<Vec<_>>()); // FIXME: ! should not be empty

    // print com hub connection information every 5 seconds
    loop {
        println!("ComHub Metadata: {}", runtime.com_hub().get_metadata());
        sleep(Duration::from_secs(5)).await;
    }
}