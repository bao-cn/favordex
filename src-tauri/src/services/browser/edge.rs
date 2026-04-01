use crate::services::browser::run_terminal_command;

pub fn open_sync_edge_page() {
    #[cfg(target_os = "windows")] {
        run_terminal_command("start", &["msedge", "edge://settings/profiles/sync/reset"]).unwrap();
    }
    #[cfg(target_os = "macos")] {
        run_terminal_command("open", &["-a", "Microsoft Edge", "edge://settings/profiles/sync/reset"]).unwrap();
    }
    #[cfg(target_os = "linux")] {
        run_terminal_command("microsoft-edge", &["--new-window", "edge://settings/profiles/sync/reset"]).unwrap();
    }
}
