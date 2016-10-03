extern crate getopts;
use getopts::Options;
use std::env;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str::FromStr;
use std::thread;

fn main() {
	let mut args = env::args();
	let brief = format!("Usage: {} [options] host port", args.next().unwrap());

	let mut opts = Options::new();
	opts.optflag("l", "listen", "listen for incoming connections");
	opts.optflag("h", "help", "print this help");

	let matches = match opts.parse(args) {
		Ok(v) => v,
		Err(e) => panic!(e.to_string()),
	};

	if matches.opt_present("h") {
		print!("{}", opts.usage(&brief));
		return;
	}

	if matches.opt_present("l") {
		panic!("Listen mode not yet implemented");
	}

	if matches.free.len() != 2 {
		panic!("Not enough arguments!");
	}

	let host = &matches.free[0];
	let port = u16::from_str(&matches.free[1]).unwrap();

	connect(host, port);
}

fn connect(host: &str, port: u16) {
	bidicat(TcpStream::connect((host, port)).unwrap());
}

fn bidicat(stream: TcpStream) {
	let input = stream.try_clone().unwrap();

	thread::spawn(move || {
		let stdout = io::stdout();
		cat(stdout, input);
	});

	let stdin = io::stdin();
	cat(stream, stdin);
}

fn cat<O: Write, I: Read>(mut output: O, mut input: I) {
	let mut buf = [0u8; 512];

	loop {
		let len = input.read(&mut buf).unwrap();
		if len == 0 {
			break;
		}

		output.write(&buf).unwrap();
	}
}
