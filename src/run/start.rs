// use crate::run::run_cmd::kill_app;
use crate::run::run_cmd::precise_kill;
use anyhow::Result;
use std::thread;
use tokio::time::Duration;
pub async fn thread_start() -> Result<()> {
    // 启动运行线程任务
    let run_thread = tokio::spawn(async move { app_run() });

    // 等待
    let _ = run_thread.await?;

    Ok(())
}

fn app_run() -> Result<()> {
    loop {
        precise_kill("com.xunmeng.pinduoduo");
        thread::sleep(Duration::from_millis(1000));
    }
}
