use std::{fs, io};

fn main() -> io::Result<()> {
    let mut entries = fs::read_dir("/dev/").unwrap()
        .map(|res| res.map(|e| e.path()))
        .map(|path| path.unwrap().into_os_string().into_string().unwrap())
        .filter(|path| path.starts_with("/dev/ttyACM"))
        .collect::<Vec<_>>();
    entries.sort();

    println!("{:?}", entries);
    Ok(())
}
