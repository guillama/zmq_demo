use std::process;
use std::io::Result;
use clap::{Arg, Command};

pub const ZMQ_BUFFER_MAX_SIZE: usize = 1500; // bytes
pub const ZMQ_PORT: i32 = 8000;
pub const ZMQ_SOCKET_FLAGS: i32 = 0;

pub mod zmqproto {
    include!(concat!(env!("OUT_DIR"), "/zmqproto.rs"));
}

use zmqproto::ZmqProtocol;
use prost::Message;
use crate::zmqproto::zmq_protocol::MessageType;

fn main() {
    let (responder, matches) = initialise().unwrap_or_else(|e|{
        eprintln!("initialise() failed: {}", e);
        process::exit(1);
    });

    println!("Waiting for data on port {ZMQ_PORT}...");

    // Get 'verbose' value from argument
    let verbose = matches.get_one::<bool>("verbose");

    loop {
        if let Err(_) = run(&responder, verbose) {
            break;
        }
    }
}

// Initialise application arguments, ZMQ context and socket
fn initialise() -> Result<(zmq::Socket, clap::ArgMatches)> {

    // Configure application 'verbose' argument
    let matches = Command::new("rasp0mq")
        .arg(Arg::new("verbose")
            .short('v')
            .long("verbose")
            .required(false)
            .num_args(0)
            .help("Show verbose output"))
            .arg_required_else_help(true)
        .arg(Arg::new("noverbose")
            .long("noverbose")
            .required(false)
            .num_args(0)
            .help("Disable verbose output"))
            .arg_required_else_help(true)
        .get_matches();

    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP)?;

    let bind_args = format!("tcp://*:{ZMQ_PORT}");
    responder.bind(bind_args.as_str())?;

    Ok((responder, matches))
}

// This function process incoming messages from the ZMQ socket. It does :
//  1) Wait for a message
//  2) Build a response from the received message
//  3) Send it back to the sender
fn run(responder: &zmq::Socket, verbose: Option<&bool>) -> Result<()> {

    // Get the received message to extract its id and sequence number for the response
    let recv_msg = match recv_message(&responder, verbose) {
        Err(e) => {
            eprintln!("Warning! recv_message() failed: {}", e);
            return Err(e.into());
        }
        Ok(recv_msg) => recv_msg
    };

    // Build Ack response
    let ack_data = String::from("ACK");
    let ack_msg = ZmqProtocol {
        id: recv_msg.id,
        msg_type: MessageType::Response as i32,
        seq_num: recv_msg.seq_num,
        size: ack_data.len() as u32,
        data: ack_data.into_bytes(),
    };

    // Encode the response to a buffer then send it through the ZMQ socket
    let mut buf = Vec::new();
    if let Err(e) = ack_msg.encode(&mut buf) {
        eprintln!("Warning! encode() failed: {}", e);
        return Err(e.into());
    };

    if let Err(e) = responder.send(&buf, ZMQ_SOCKET_FLAGS) {
        eprintln!("send() failed: {}", e);
        return Err(e.into());
    }

    Ok(())
}

// Receive then decode message from ZMQ socket
fn recv_message(responder: &zmq::Socket, verbose: Option<&bool>) -> Result<ZmqProtocol>{
    let buffer = &mut [0; ZMQ_BUFFER_MAX_SIZE];
    let nb_bytes = responder.recv_into(buffer, ZMQ_SOCKET_FLAGS)?;

    // Ensure that nb_bytes doesn't exceed ZMQ_BUFFER_MAX_SIZE to prevent potential out-of-bounds access
    if nb_bytes > ZMQ_BUFFER_MAX_SIZE { ZMQ_BUFFER_MAX_SIZE } else { nb_bytes };

    let msg = ZmqProtocol::decode(&buffer[..nb_bytes])?;

    if let Some(true) = verbose {
        println!("{:?}", msg); 
    }

    Ok(msg)
}