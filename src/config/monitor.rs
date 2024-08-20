use nix::{errno::Errno, sys};
use std::ffi::OsStr;

use super::config::create_path;

pub fn time_modified() -> Result<i64, Errno> {
    let binding = create_path().unwrap();
    let fd = OsStr::new(&binding);
    let stat = sys::stat::stat(fd);
    Ok(stat.unwrap().st_atime)
}

pub fn compare_modified_date(timestamp: i64) -> i64 {
    let newtimestamp = time_modified().expect("Error when obtaining timestamp");
    if timestamp < newtimestamp {
        return newtimestamp;
    } else {
        return timestamp;
    }
}