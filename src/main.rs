use std::process::Output;

use libc;
use pam_client::conv_cli::Conversation;
use pam_client::{Context, Flag};
fn auth() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("rl start");
        return;
    }
    if unsafe { libc::geteuid() } != 0 {
        eprintln!("ошибка прав root");
        std::process::exit(1);
    }
    unsafe {
        libc::setgid(0);
        libc::setuid(0);
    }
    let output = std::process::Command::new(&args[1])
        .args(&args[2..])
        .output()
        .expect("command is not correct");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}
fn main() {
    let username = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    println!("[rl] Аутентификация для пользователя: {}", username);
    let mut context = Context::new("login", Some(&username), Conversation::new())
        .expect("Не удалось инициализировать PAM");
    match context.authenticate(Flag::NONE) {
        Ok(_) => auth(),
        Err(e) => println!("Ошибка: {:?}", e),
    }
}
