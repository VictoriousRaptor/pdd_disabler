//From ab/SummerSK/puff/Aestas.Britannia
use std::process::Command;

pub fn kill_app() {
    let _ = Command::new("am force-stop com.xunmeng.pinduoduo").output();
}
