use std::{fs, io};
use std::process::Command;

fn main() -> io::Result<()> {
    // FIXME: this code includes many unwrap()
    let port_names = fs::read_dir("/dev/").unwrap()
        .map(|res| res.map(|e| e.path()))
        .map(|path| path.unwrap().into_os_string().into_string().unwrap())
        .filter(|path| path.starts_with("/dev/ttyACM"))
        .collect::<Vec<_>>();

    for port_name in port_names {
        set_baud_rate(port_name, 9600)
    }

    Ok(())
}

fn set_baud_rate(port_name: String, baud_rate: u32) {
    Command::new("stty")
            .arg(format!("-F {} raw {}", port_name, baud_rate))
            .output()
            .expect("error");
}
