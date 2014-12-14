use std::io::{TcpStream, BufferedStream};


fn main() {
    let mut stream = TcpStream::connect("hm.hacked.jp:4242");

    // Time to attempt a connection...

    // here is the handshake
    stream.write_be_u32(0x42b33f00).ok().expect("failed to send magic int"); // we absolutely do not support encryption or compression
    stream.write_be_u32(0x00000001).ok().expect("failed to send legacy protocol bit"); // we are only using the legacy protocol
    stream.write_be_u32(0x80000001).ok().expect("failed to send end-of-list bit"); // end of list

    println!("We are now going to sit on our asses and wait for the buf to come back");

    loop {
        let _ = match stream.read_be_u32() {
            Ok(data) => {
                println!("0x{:x}", data)
            },
            Err(err) => {
                panic!("error reading response: {}", err)
            }
        };
    }
    drop(stream);
    println!("Hopefully the core has shut the fuck up by now");
}

