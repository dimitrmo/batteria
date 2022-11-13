use batteria_cli::{Args, Device};
use clap::Parser;

use std::io;
use std::thread;
use std::time::Duration;

/*
fn main() -> battery::Result<()> {
    let manager = battery::Manager::new()?;
    let mut battery = match manager.batteries()?.next() {
        Some(Ok(battery)) => battery,
        Some(Err(e)) => {
            eprintln!("Unable to access battery information");
            return Err(e);
        }
        None => {
            eprintln!("Unable to find any batteries");
            return Err(io::Error::from(io::ErrorKind::NotFound).into());
        }
    };

    loop {
        println!("{:?}", battery);
        thread::sleep(Duration::from_secs(1));
        manager.refresh(&mut battery)?;
    }
}*/


pub fn main() {
    let args = Args::parse();
    println!("hello {:?}", args.device);
    if args.device == Device::Laptop {
        let manager = battery::Manager::new().unwrap();

        println!("all batteries {:?}", manager.batteries().unwrap());
        for battery in manager.batteries().unwrap().into_iter() {
            match battery {
                Ok(bb) => {
                    println!("some batter we have here {:?}", bb);
                },
                Err(_) => {
                    eprintln!("Unable to find any batteries");
                }
            };
        }

    }
}
