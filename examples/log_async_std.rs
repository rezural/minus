// This is the example used when debugging minus. It will crate a file called minus.log
// Tracing if used as the logger
// Dependencies are pulled of tracing, tracing-appender and tracing-subscriber

use async_std::task::sleep;
use futures::join;

use std::fmt::Write;
use std::time::Duration;
use tracing::{subscriber, Level};
use tracing_appender::{non_blocking, rolling::never};
use tracing_subscriber::fmt;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = never("./", "minus.log");
    let (non_block, _guard) = non_blocking(file);
    let subscriber = fmt()
        .with_writer(non_block)
        .with_max_level(Level::INFO)
        .compact()
        .finish();

    subscriber::set_global_default(subscriber).unwrap();

    let output = minus::Pager::new().finish();

    let increment = async {
        for i in 0..=30_u32 {
            let mut output = output.lock().await;
            writeln!(output.lines, "{}", i)?;
            drop(output);
            sleep(Duration::from_millis(100)).await;
        }
        Result::<_, std::fmt::Error>::Ok(())
    };
    let (res1, res2) = join!(minus::async_std_updating(output.clone()), increment);
    res1?;
    res2?;
    Ok(())
}
