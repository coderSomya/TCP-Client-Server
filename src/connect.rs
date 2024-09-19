use std::{fmt::format, io::{stdout, Write}, net::{TcpListener, TcpStream}};
use std::fs::read;

pub fn run(host: String, port: u16)-> Result<(), String>{
    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(addr.clone()).map_err(|err|{
        format!("failed to connnect to {}", err.to_string())
    })?;

    for stream in listener.incoming(){
        match stream{
                Ok(mut s)=> {
                    let mut buff: [u8; 128] = [0;128];
                    let mut read_bytes = 0;
                    while read_bytes == 0 {
                        read_bytes = s.read(&mut buff).map_err(|e|{
                            format!("faied to read bytes {}", e)
                        })?;
    
                    }
                    stdout().write(&buff[0..read_bytes]).map_err(||{
                        "failed to write to stdout"
                    })?;
                    stdout().flush().unwrap();
                    println!("connection accepted");
                }
                Err(e)=> {
                    println!("failed to connect {}", e);
                }
        }
    }
    
    Ok(())
}