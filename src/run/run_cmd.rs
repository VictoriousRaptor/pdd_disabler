//From ab/SummerSK/puff/Aestas.Britannia
use std::process::Command;

pub fn kill_app(package_name: &str) {
    let _ = Command::new("nohup")
        .args(["am", "force-stop", package_name, ">/dev/null", "2>&1", "&"])
        .output();
}
