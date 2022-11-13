use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(value_enum)]
    #[arg(short, long)]
    pub device: Device
}

#[derive(ValueEnum, Clone, Debug, Eq, PartialEq)]
pub enum Device {
    Laptop,
}
