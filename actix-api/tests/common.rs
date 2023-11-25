use std::process::Command;

pub fn dbmate_rebuild(url: &str) {
    let do_steps = || -> anyhow::Result<()> {
        Command::new("dbmate")
            .arg("drop")
            .env("DATABASE_URL", &url)
            .status()
            .expect("failed to execute process");
        Command::new("dbmate")
            .arg("up")
            .env("DATABASE_URL", &url)
            .status()
            .expect("failed to execute process");
        Command::new("dbmate")
            .arg("wait")
            .env("DATABASE_URL", url)
            .status()
            .expect("failed to execute process");
        Ok(())
    };
    if let Err(err) = do_steps() {
        println!("Failed to perform db operation {}", err.to_string());
        dbmate_rebuild(url);
    }
}
