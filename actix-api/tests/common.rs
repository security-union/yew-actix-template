use std::process::Command;

pub fn dbmate_up(url: &str) {
    log::info!("dbmate up DATABASE_URL: {}", url);
    let do_steps = || -> bool {
        Command::new("sh")
            .arg("-c")
            .arg("dbmate up")
            .env("DATABASE_URL", url)
            .status()
            .expect("failed to execute process")
            .success()
    };
    if !do_steps() {
        panic!("Failed to perform dbmate up operation");
    }
}

