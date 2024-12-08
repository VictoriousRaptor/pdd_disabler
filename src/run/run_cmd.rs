//From ab/SummerSK/puff/Aestas.Britannia
use std::process::Command;

pub fn kill_app(package_name: &str) {
    let _ = Command::new("nohup")
        .args(["am", "force-stop", package_name, ">/dev/null", "2>&1", "&"])
        .output();
}

pub fn precise_kill(package_name: &str) {
    let output = Command::new("ps")
        .args([
            "-ef",
            "|",
            "grep",
            package_name,
            "|",
            "grep",
            "-v",
            "grep",
            "|",
            "wc",
            "-l",
        ])
        .output()
        .unwrap();
    if output.clone().status.success() {
        // 将输出转换为字符串并打印
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("Command output: {}", output_str);
    }
}
