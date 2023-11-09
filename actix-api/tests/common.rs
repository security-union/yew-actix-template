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

pub fn dbmate_drop(url: &str) {
    log::info!("dbmate drop DATABASE_URL: {}", url);
    let do_steps = || -> bool {
        Command::new("sh")
            .arg("-c")
            .arg("dbmate drop")
            .env("DATABASE_URL", url)
            .status()
            .expect("failed to execute process")
            .success()
    };
    if !do_steps() {
        log::error!("Failed to perform dbmate drop operation");
    }
}

