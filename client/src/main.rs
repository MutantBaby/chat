use std::io::{self, ErrorKind, Read, Write};
use std::sync::mpsc::{self, TryRecvError};
use std::net::TcpStream;
use std::time::Duration;
use std::thread;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {}
