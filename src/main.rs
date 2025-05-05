#![warn(clippy::nursery, clippy::pedantic)]
#![allow(
    clippy::non_std_lazy_statics,
    clippy::similar_names,
    clippy::missing_safety_doc,
    clippy::missing_panics_doc
)]
mod run;
mod shared;

use log::info;
use run::start::thread_start;
use shared::logger::init_log;
use std::{fs, process};

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
