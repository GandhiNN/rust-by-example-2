use tracing::info;
use tracing_subscriber::fmt as TracingFmt;

#[tracing::instrument]
fn compute_sum(a: i32, b: i32) -> i32 {
    info!("starting calculations");
    a + b
}

fn main() {
    // // Setting log level to `debug`
    // tracing_subscriber::fmt()
    //     .with_max_level(Level::DEBUG)
    //     .init();

    TracingFmt::init();

    tracing::info!("Look ma, I'm tracing!");

    let res = compute_sum(5, 10);
    tracing::info!("The sum is: {}", res);
}
