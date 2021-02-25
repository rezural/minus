use async_std::task::sleep;
use futures::join;

use std::fmt::Write;
use std::time::Duration;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = minus::Pager::new().finish();
    let mut bytes_written: usize = 0;

    fn text() -> String {
        let line = "afasldjgkaslgj asdglk;jasd g;lkja sdg; akjsdg;laksdjg a;slkd gj\n";
        let mut text = String::new();
        // for _ in (0..10000) {
            text.push_str(line);
        // }
        text
    }
    let increment = async {
        loop {
            let mut output = output.lock().await;
            let the_text = text();
            writeln!(output.lines, "{}", the_text)?;
            bytes_written += the_text.len();
            println!("bytes_written: {}", bytes_written);
            drop(output);
            // sleep(Duration::from_millis(100)).await;
        }
        Result::<_, std::fmt::Error>::Ok(())
    };

    let (res1, res2) = join!(minus::async_std_updating(output.clone()), increment);
    res1?;
    res2?;
    Ok(())
}
