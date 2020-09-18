use std::process::Command;

use tokio::fs::File;
use tokio::prelude::*;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // FIXME: this code includes many unwrap()
    let mut port_names = std::fs::read_dir("/dev/").unwrap()
        .map(|res| res.map(|e| e.path()))
        .map(|path| path.unwrap().into_os_string().into_string().unwrap())
        .filter(|path| path.starts_with("/dev/ttyACM"))
        .collect::<Vec<_>>();
    port_names.reverse();

    println!("{:?}", port_names);
    for port_name in port_names {
        let mut delay = time::delay_for(Duration::from_secs(5));
        let mut port = File::open(port_name.clone()).await.unwrap();
        tokio::select! {
            _ = &mut delay => {
                println!("{}", port_name);
            }
            _ = match_serial_port_input(port.try_clone().await.unwrap(), port_name.clone(), 9600) => {
                println!("something wrong");
            }
        }
    }

    Ok(())
}

async fn match_serial_port_input(port: File, port_name: String, baud_rate: u32) {
    set_baud_rate(port_name.clone(), baud_rate);
    let mut stream= tokio::io::BufReader::new(port);

    loop {
        let mut buf = String::new();
        stream.read_line(&mut buf).await.unwrap();
        print!("{}", buf);
    }
}

fn set_baud_rate(port_name: String, baud_rate: u32) {
    // TODO: change to better way to set baud rate
    Command::new("stty")
            .args(&["-F", &port_name, "raw", &baud_rate.to_string()])
            .output()
            .expect("error");
}
