use std::{
    error,
    io::{self, BufRead, BufReader, BufWriter, Write},
    net::{IpAddr, TcpStream},
    thread,
};

pub struct Config {
    address: IpAddr,
    port: u32,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let address = match args.next() {
            Some(arg) => arg,
            None => return Err("Please specify in IP address"),
        };

        let address = match address.parse() {
            Ok(parsed) => parsed,
            Err(_) => return Err("Invalid IP address"),
        };

        let port = match args.next() {
            Some(arg) => arg,
            None => return Err("Please specify a port number"),
        };

        let port = match port.parse() {
            Ok(parsed) => parsed,
            Err(_) => return Err("Invalid port number"),
        };

        Ok(Self { address, port })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let addr = format!("{}:{}", config.address, config.port);
    let stream = TcpStream::connect(addr)?;
    let copy = stream.try_clone()?;

    thread::spawn(move || handle_read(copy));
    handle_write(stream)?;

    Ok(())
}

fn handle_read(stream: TcpStream) {
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        let line = match line {
            Ok(value) => value.trim().to_string(),
            Err(_) => break,
        };

        println!("{line}");
    }
}

fn handle_write(stream: TcpStream) -> Result<(), io::Error> {
    let mut writer = BufWriter::new(stream);

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Should be able to read the standard input");

        let input = format!("{}\r\n", input.trim());
        writer.write_all(input.as_bytes())?;
        writer.flush()?;
    }
}
