use anyhow::Result;
use daemonize::Daemonize;
use std::path::Path;

pub fn daemonize() -> Result<()> {
    let daemon = Daemonize::new()
        .pid_file("/tmp/web-cacher.pid") // 更安全的位置
        .chown_pid_file(true)
        .working_directory(".")
        .umask(0o027)
        .exit_action(|| println!("Daemon started"))
        .privileged_action(|| {
            println!("Executing privileged code");
            Ok(())
        });

    match daemon.start() {
        Ok(_) => {
            tracing::info!("Running in daemon mode");
            Ok(())
        }
        Err(e) => {
            tracing::error!("Daemonization failed: {}", e);
            Err(e.into())
        }
    }
}
