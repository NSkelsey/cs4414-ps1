//
// zhttpto.rs
//
// University of Virginia - cs4414 Fall 2013
// Weilin Xu and David Evans
// Version 0.1


extern mod extra;

use extra::uv;
use extra::{net_ip, net_tcp};
use std::io::{Path, read_whole_file_str};
use std::{str, uint, vec};

static BACKLOG: uint = 5;
static PORT:    uint = 4414;
static IPV4_LOOPBACK: &'static str = "127.0.0.1";
static mut visitor_count: int = 0;


fn read_file(s_path: &str) -> ~str {
  let path = ~Path(s_path);
  match read_whole_file_str(path){
     Ok(res) => res,
     Err(_) => ~"Bad path"
  }
}



fn new_connection_callback(new_conn :net_tcp::TcpNewConnection, _killch: std::comm::SharedChan<Option<extra::net_tcp::TcpErrData>>)
{
    do spawn {
        let accept_result = extra::net_tcp::accept(new_conn);
        match accept_result {
            Err(err) => {
               println(fmt!("Connection error: %?", err));
            },  
            Ok(sock) => {
                let peer_addr: ~str = net_ip::format_addr(&sock.get_peer_addr());
                println(fmt!("Received connection from: %s", peer_addr));
                
                let read_result = net_tcp::read(&sock, 0u);
                match read_result {
                    Err(err) => {
                        println(fmt!("Receive error: %?", err));
                    },
                    Ok(bytes) => unsafe {
			visitor_count += 1;
                        let request_str = str::from_bytes(bytes.slice(0, bytes.len() - 1));
                        let iter_o = request_str.split_iter(' ').skip(1).next();
		        let response =  match iter_o {
					Some("/") => {
					   fmt!("Visitor count: %d", visitor_count)
					},
					Some(path) => {
					   let path: &str = "." + path;
					   read_file(path)
					},
					None => { ~"Bad input" }
				};
			let raw_end = response.as_bytes_with_null_consume();
                        println(fmt!("Request received:\n%s", request_str));
                        let mut raw: ~[u8] =
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n".as_bytes().to_owned();
			raw = raw + raw_end;
                        net_tcp::write(&sock, raw);
                    },
                };
            }
        }
    };
}

fn main() {
    net_tcp::listen(net_ip::v4::parse_addr(IPV4_LOOPBACK), PORT, BACKLOG,
                    &uv::global_loop::get(),
                    |_chan| { println(fmt!("Listening on tcp port %u ...", PORT)); },
                    new_connection_callback);
}
