mod run;
mod shared;
use crate::shared::logger::init_log;
use std::process;
use tokio::fs;
fn init_misc() {
    let _ = init_log();
    let self_pid = process::id();
    let _ = fs::write("/dev/cpuset/background/cgroup.procs", self_pid.to_string());
}

fn main() {
    init_misc();
    thread_start.await();
}
