use std::process::Command;
use std::io::BufReader;
use std::collections::HashMap;
use std::sync::Arc;

use tokio::fs::File;
use tokio::prelude::*;
use tokio::time::{self, Duration};
use tokio::sync::{Mutex, MutexGuard};

use regex::Regex;

#[derive(Clone, Debug)]
struct Entry {
    name: String,
    regexp: Regex,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // FIXME: This code includes many unwrap().
    //        And structs must tidy up this code.

    // Get pairs of a name and a regexp
    let mut entries: Vec<Entry> = vec!();
    for line in std::io::BufRead::lines(BufReader::new(std::io::stdin())){
        let l = line.unwrap();
        let mut args = l.split_whitespace();
        entries.push(Entry {
            name: args.next().unwrap().to_string(),
            regexp: Regex::new(args.next().unwrap()).unwrap(),
        });

    }

    // Get port names
    let port_names = std::fs::read_dir("/dev/").unwrap()
        .map(|res| res.map(|e| e.path()))
        .map(|path| path.unwrap().into_os_string().into_string().unwrap())
        .filter(|path| path.starts_with("/dev/ttyACM"))
        .collect::<Vec<_>>();

    // Collect data from got ports
    let mut s = HashMap::new();
    for port_name in port_names.clone() {
        s.insert(port_name, vec!());
    }
    let store = Arc::new(Mutex::new(s));
    for port_name in port_names {
        let mut delay = time::delay_for(Duration::from_secs(5));
        let port = File::open(port_name.clone()).await.unwrap();
        tokio::select! {
            _ = &mut delay => {
                println!("{}", port_name);
            }
            _ = collect_data_from_serial_port(port.try_clone().await.unwrap(), port_name.clone(), 9600, store.clone()) => {
                println!("something wrong");
            }
        }
    }

    // Match data with regexp.
    for (k, vs) in store.lock().await.iter() {
        for entry in entries.clone() {
            if vs.into_iter().map(|v| entry.regexp.is_match(&v)).all(|x| x == true) {
            } else {
            }
        }
    }

    Ok(())
}

async fn collect_data_from_serial_port(port: File, port_name: String, baud_rate: u32, store: Arc<Mutex<HashMap<String, Vec<String>>>>) {
    set_baud_rate(port_name.clone(), baud_rate);
    let mut stream= tokio::io::BufReader::new(port);

    loop {
        let mut buf = String::new();
        stream.read_line(&mut buf).await.unwrap();
        let mut s = store.lock().await;
        s.get_mut(&port_name).unwrap().push(buf.clone());
    }
}

fn set_baud_rate(port_name: String, baud_rate: u32) {
    // TODO: change to better way to set baud rate
    Command::new("stty")
            .args(&["-F", &port_name, "raw", &baud_rate.to_string()])
            .output()
            .expect("error");
}
