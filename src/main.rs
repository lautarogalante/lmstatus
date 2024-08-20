use lmstatus::prelude::*;
use lmstatus::status::status::Status;
use std::sync::mpsc;

fn main() -> Result<(), CliError> {
    let delay = time::Duration::from_secs(1);
    let mut config: Config = Default::default();
    let mut time_modified = time_modified().unwrap();
    config.load()?; 
    let (tx, rx) = mpsc::channel();

    loop {
        let tx_clone = tx.clone();
        let time_thread = thread::spawn(move || {
            let time = compare_modified_date(time_modified);
            tx_clone.send(time).expect("Error to sending time");
        });
        let receiver = rx.recv(); 
        
        match  receiver {
           Ok(time) => {
                config.load()?;
                time_modified = time;
           },
           Err(e) => eprintln!("Error receiving time: {}",e),
        } 
        
        time_thread.join().expect("Error in joining thread");

        let _ = Icons::new(&config);
        let status = Some(Status::new(&config)?).unwrap();
        let status_str = format!("xprop -root -set WM_NAME '{}'", status);

        Command::new("sh") 
        .arg("-c")
        .arg(status_str)
        .output()?;

        thread::sleep(delay);
    } 
}
