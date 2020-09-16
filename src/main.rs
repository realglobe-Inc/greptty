use std::{fs, io};

fn main() -> io::Result<()> {
    let mut entries = fs::read_dir("/dev/")?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort();

    println!("{:?}", entries);
    Ok(())
}
