use tracing::{error, info, trace, warn};

fn main() {
    tracing::subscriber::set_global_default(tracing_subscriber::FmtSubscriber::new())
        .expect("setting default subscriber failed");

    let number = 5;
    info!("The number is {}", number);

    let result = compute(number);
    info!("The result is {}", result);
}

fn compute(n: i32) -> i32 {
    trace!("Computing the value...");
    if n > 10 {
        warn!("The number is greater than 10");
    } else if n < 1 {
        error!("The number is less than 1");
    }
    n * 2
}
