use crate::run::run_cmd::precise_kill;
use crate::shared::get_top_app::get_topapp_pid_and_name;
use anyhow::Result;
use log::info;
use std::thread;
use tokio::time::Duration;
pub async fn thread_start() -> Result<()> {
    // 启动运行线程任务
    let run_thread = tokio::spawn(async move { app_run("com.xunmeng.pinduoduo") });

    // 等待
    let _ = run_thread.await?;

    Ok(())
}

fn app_run(package_name: &str) -> Result<()> {
    let mut global_package = String::new();
    loop {
        let (_, name) = get_topapp_pid_and_name()?;
        if global_package == name {
            thread::sleep(Duration::from_millis(1000));
            continue;
        }
        global_package = name;
        // 如果此次启动的APP为拼多多，则不kill拼多多
        if global_package == package_name {
            continue;
        }
        // 用户删除拼多多后台时自动清理拼多多残留进程
        let rs = precise_kill(package_name);
        if rs.is_err() {
            info!("清算失败: {}", package_name);
            continue;
        }
        info!("清算成功: {}", package_name);
    }
}
