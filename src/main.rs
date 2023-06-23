use std::ffi::OsStr;

use headless_chrome::{Browser, LaunchOptionsBuilder};
use tracing::info;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    ekinet()?;

    Ok(())
}

fn ekinet() -> anyhow::Result<()> {
    let launch_opts = LaunchOptionsBuilder::default()
        .args(vec![
            OsStr::new("--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36"),
        ])
        .build()
        .expect("failed to build LaunchOptions");
    let browser = Browser::new(launch_opts)?;
    let tab = browser.new_tab()?;

    tab.navigate_to("https://www.eki-net.com/personal/top/index")?;
    tab.wait_until_navigated()?;

    let title = tab.get_title()?;
    info!("title: {}", title);

    Ok(())
}
