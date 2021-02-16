use async_std::task::sleep;
use futures::join;

use std::fmt::Write;
use std::time::Duration;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = minus::Pager::new().finish();

    fn large_page() -> String {
        let mut ret = String::new();
        //2 bugs here: when outputting a string larger than the terminal, the screen appears blank until keypress
        for i in 0..100u32 {
        // and when outputting a string with more than one line (comment above and uncomment below to see this)
        // the last string appended that goes over the terminal height, doesnt get displayed until keypress
        // for i in 0..10u32 {
            ret.push_str("asdfasdf\n");
        }
        ret
    }
    let increment = async {
        for i in 0..=10_u32 {
            let mut output = output.lock().await;
            writeln!(output.lines, "{}", large_page())?;
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
