use crate::services::browser::run_terminal_command;

pub fn open_sync_chrome_page() {
    let target_url = "chrome://settings/syncSetup/advanced";
    
    let html_content = format!(
        "data:text/html,<html><script>window.location.href='{}';</script></html>",
        target_url
    );
    #[cfg(target_os = "windows")] {
        run_terminal_command("start", &["chrome", &html_content]).unwrap();
    }
    #[cfg(target_os = "macos")] {
        run_terminal_command("open", &["-a", "Google Chrome", &html_content]).unwrap();
    }
    #[cfg(target_os = "linux")] {
        run_terminal_command("google-chrome", &["--new-window", &html_content]).unwrap();
    }
}