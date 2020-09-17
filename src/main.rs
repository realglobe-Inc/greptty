use std::{fs, io};
use std::io::BufReader;
use std::process::Command;
use tokio::fs::File;
use std::io::prelude::*;

use tokio::prelude::*;
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // FIXME: this code includes many unwrap()
    let port_names = fs::read_dir("/dev/").unwrap()
        .map(|res| res.map(|e| e.path()))
        .map(|path| path.unwrap().into_os_string().into_string().unwrap())
        .filter(|path| path.starts_with("/dev/ttyACM"))
        .collect::<Vec<_>>();

    for port_name in port_names {
        set_baud_rate(port_name.clone(), 9600);
        let port = File::open(port_name).await?;
        let mut stream= tokio::io::BufReader::new(port);

        loop {
            let mut buf = String::new();
            stream.read_line(&mut buf).await?;
            print!("{}", buf);
        }
    }

    Ok(())
}

fn set_baud_rate(port_name: String, baud_rate: u32) {
    // TODO: change to better way to set baud rate
    Command::new("stty")
            .arg(format!("-F {} raw {}", port_name, baud_rate))
            .output()
            .expect("error");
}
