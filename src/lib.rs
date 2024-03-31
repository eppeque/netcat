use std::{
    error::Error,
    io::{self, BufRead, BufReader, BufWriter, Write},
    net::{Ipv4Addr, TcpStream},
    thread,
};

pub struct Config {
    address: Ipv4Addr,
    port: u32,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let address = match args.next() {
            Some(value) => value,
            None => return Err("Please specify an IP address"),
        };

        let address: Ipv4Addr = match address.parse() {
            Ok(value) => value,
            Err(_) => return Err("The given IP address is invalid"),
        };

        let port = match args.next() {
            Some(value) => value,
            None => return Err("Please specify a port number"),
        };

        let port: u32 = match port.parse() {
            Ok(value) => value,
            Err(_) => return Err("The given port number is invalid"),
        };

        Ok(Self { address, port })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let address = format!("{}:{}", config.address, config.port);
    let stream = TcpStream::connect(address)?;
    let copy = stream.try_clone()?;

    thread::spawn(|| handle_input(stream));

    handle_output(copy)?;

    Ok(())
}

fn handle_input(stream: TcpStream) {
    let mut buffer = BufReader::new(stream);

    loop {
        let mut line = String::new();
        buffer.read_line(&mut line).unwrap();

        let line = line.trim();

        if !line.is_empty() {
            println!("{line}");
        }
    }
}

fn handle_output(stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = BufWriter::new(stream);

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if let Err(_) = buffer.write_all(input.as_bytes()) {
            break;
        }

        if let Err(_) = buffer.flush() {
            break;
        }
    }

    Ok(())
}
