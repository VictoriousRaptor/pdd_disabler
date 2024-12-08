//From ab/SummerSK/puff/Aestas.Britannia
use log::info;
use std::process::Command;
pub fn kill_app(package_name: &str) {
    let _ = Command::new("nohup")
        .args(["am", "force-stop", package_name, ">/dev/null", "2>&1", "&"])
        .output();
}

pub fn precise_kill(package_name: &str) {
    let output = Command::new("sh")
        .args([
            "-c",
            "ps -ef | grep com.xunmeng.pinduoduo | grep -v grep | wc -l",
        ])
        .output()
        .unwrap();
    if output.clone().status.success() {
        // 将输出转换为字符串并打印
        let output_str: &str = &String::from_utf8_lossy(&output.stdout);
        if output_str.trim() == "1" {
            info!("清算拼多多");
            kill_app(package_name);
        } else {
            println!("值为t: {}", output_str);
        }
    } else {
        // 如果命令执行失败，打印错误信息
        let error_str = String::from_utf8_lossy(&output.stderr);
        println!("Error: {}", error_str);
    }
}
