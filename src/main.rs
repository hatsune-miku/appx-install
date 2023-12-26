fn run_elevated_pwsh(command: &str) -> bool {
    let command = format!("\"{}\"", command);
    let args = ["powershell", "-command", command.as_str()];
    return runas::Command::new("powershell.exe")
        .args(&args)
        .show(true)
        .status()
        .is_ok();
}

fn elevated_change_policy() -> bool {
    run_elevated_pwsh("Set-ExecutionPolicy RemoteSigned -Scope CurrentUser")
}

fn run_install_ps1() -> bool {
    std::process::Command::new("powershell.exe")
        .args(&["-File", "Install.ps1"])
        .status()
        .is_ok()
}

fn main() {
    if !elevated_change_policy() {
        println!("Error: Failed to elevate and change policy");
        println!("错误：提权并更改 Powershell 执行策略失败");
        std::process::exit(1);
    }

    if !run_install_ps1() {
        println!("Error: Failed to elevate and run Install.ps1");
        println!("错误：提权并运行 Install.ps1 失败");
        std::process::exit(2);
    }
}
