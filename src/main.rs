use lmstatus::prelude::*;
use lmstatus::prelude::utils::*;
use lmstatus::status::status::Status;


fn main() -> Result<(), CliError> {
    let config = load_config()?;
    let _ = Icons::new(&config);
    let delay = time::Duration::from_secs(1);
    loop {
        let status = Status::new(&config);
        let status_str = format!("xprop -root -set WM_NAME '{}'", status.unwrap().to_string());
        Command::new("sh")
        .arg("-c")
        .arg(status_str)
        .output()?;
        thread::sleep(delay);
    } 
}
