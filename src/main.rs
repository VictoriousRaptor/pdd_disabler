mod run;
use log::info;
mod shared;
use crate::run::start::thread_start;
use crate::shared::logger::init_log;
use std::fs;
use std::process;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn init_misc() {
    let _ = init_log();
    let self_pid = process::id();
    let _ = fs::write("/dev/cpuset/background/cgroup.procs", self_pid.to_string());
}

#[tokio::main]
async fn main() {
    init_misc();
    info!("项目启动");
    let _ = thread_start().await;
}
