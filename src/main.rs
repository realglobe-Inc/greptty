#[macro_use]
extern crate clap;

use std::process::Command;
use std::io::BufReader;
use std::collections::HashMap;
use std::sync::Arc;

use tokio::fs::File;
use tokio::prelude::*;
use tokio::time::{self, Duration};
use tokio::sync::{Mutex};

use regex::Regex;

use clap::{App, Arg};

#[derive(Clone, Debug)]
struct Entry {
    name: String,
    regexp: Regex,
    baud_rate: u32,
    store:  Arc<Mutex<HashMap<String, Vec<String>>>>,
}

impl Entry {
    async fn scan_port(self: Entry, port_name: String) {
        let p = port_name.clone();
        set_baud_rate(p.clone(), self.baud_rate);
        let port = File::open(port_name).await.unwrap();
        let mut stream = tokio::io::BufReader::new(port);

        loop {
            let mut buf = String::new();
            stream.read_line(&mut buf).await.unwrap();
            let mut s = self.store.lock().await;
            s.get_mut(&p).unwrap().push(buf.clone());
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define CLI application.
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::from_usage("<PREFIX> 'Prefix to scan ports'"));
    let matches = app.get_matches();

    // Collect arguments.
    let prefix = match matches.value_of("PREFIX") {
        Some(p) => p,
        None => panic!("something wrong"),
    };
    // FIXME: This code includes many unwrap().
    //        And structs must tidy up this code.

    // Get pairs of a name and a regexp.
    // And set a port.
    let mut s = HashMap::new();
    let port_names = get_port_names(prefix.to_string())?;
    for port_name in port_names.clone() {
        s.insert(port_name, vec!());
    }

    // Build entries.
    let mut entries: Vec<Entry> = vec!();
    let store = Arc::new(Mutex::new(s));
    for line in std::io::BufRead::lines(BufReader::new(std::io::stdin())){
        let l = line.unwrap();
        let mut args = l.split_whitespace();
        entries.push(Entry {
            name: args.next().unwrap().to_string(),
            regexp: Regex::new(&["(?m)", args.next().unwrap()].concat()).unwrap(),
            baud_rate: 9600,
            store: store.clone(),
        });
    }

    // Collect data from got ports
    for entry in entries.clone(){
        for port_name in &port_names{
            let mut delay = time::delay_for(Duration::from_secs(5));
            tokio::select! {
                _ = &mut delay => {
                }
                _ = entry.clone().scan_port(port_name.to_string()) => {
                    println!("something wrong");
                }
            }
        }
    }

    // Match data with regexp.
    for (k, vs) in store.lock().await.iter() {
        for entry in &entries {
            if vs.into_iter().map(|v| entry.regexp.is_match(&v)).all(|x| x == true) {
                println!("{} 9600 {}", entry.name, k);
                break;
            } else {
            }
        }
    }

    Ok(())
}

fn get_port_names(prefix: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let port_names = std::fs::read_dir("/dev/")?
        .map(|res| res.map(|e| e.path()))
        .map(|path| path.unwrap().into_os_string().into_string().unwrap())
        .filter(|path| path.starts_with(&prefix))
        .collect::<Vec<_>>();
    Ok(port_names)
}

fn set_baud_rate(port_name: String, baud_rate: u32) {
    // TODO: change to better way to set baud rate
    Command::new("stty")
            .args(&["-F", &port_name, "raw", &baud_rate.to_string()])
            .output()
            .expect("error");
}
