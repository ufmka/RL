
use libc;
use pam_client::conv_cli::Conversation;
use pam_client::{Context, Flag};
fn auth(args: &[String]) {
    if unsafe { libc::geteuid() } != 0 {
        eprintln!("root permissions error");
        std::process::exit(1);
    }
    unsafe {
        libc::setgid(0);
        libc::setuid(0);
    }
    let output = std::process::Command::new(&args[0])
        .args(&args[1..])
        .output()
        .expect("command is not correct");
    print!("{}", String::from_utf8_lossy(&output.stdout));
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("rl whoami");
        return;
    }
    let username = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    let mut context = Context::new("login", Some(&username), Conversation::new())
        .expect("Failed to initialize PAM");
    match context.authenticate(Flag::NONE) {
        Ok(_) => auth(&args[1..]),
        Err(e) => println!("error: {:?}", e),
    }
}
