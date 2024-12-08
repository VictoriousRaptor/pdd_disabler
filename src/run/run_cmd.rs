use anyhow::Result;
use log::info;
use std::process::Command;
pub fn kill_app(package_name: &str) {
    let cmdline = format!("am force-stop {}", package_name);
    let _ = Command::new("sh").args(["-c", &cmdline]).output();
}

pub fn precise_kill(package_name: &str) -> Result<()> {
    let cmdline = format!("ps -ef | grep {} | grep -v grep | wc -l", package_name);
    let output = Command::new("sh").args(["-c", &cmdline]).output()?;
    // 将输出转换为字符串并打印
    let output_str: &str = &String::from_utf8_lossy(&output.stdout);
    let output_str = output_str.trim();
    let output_str: u8 = output_str.parse()?;
    if output_str > 0 && output_str < 4 {
        kill_app(package_name);
    }
    Ok(())
}
