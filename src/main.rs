
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

// Ethereum protocol magic bytes
const SUBSTRATE_MAGIC: [u8; 4] = [0x22, 0x80, 0x9D, 0xF2];
// Ethereum protocol version
const SUBSTRATE_VERSION: u32 = 5;

//handshake function 
fn perform_substrate_handshake<A: ToSocketAddrs>(addr: A) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(addr)?;

    // Send the Substrate handshake
    let mut handshake = Vec::new();
    handshake.extend_from_slice(&SUBSTRATE_MAGIC);
    handshake.extend_from_slice(&SUBSTRATE_VERSION.to_be_bytes());

    // Send the handshake payload
    stream.write_all(&handshake)?;

    // Read and process the response
    let mut response_magic = [0u8; 4];
    let mut response_version = [0u8; 4];

    stream.read_exact(&mut response_magic)?;
    stream.read_exact(&mut response_version)?;

    // if response_magic != SUBSTRATE_MAGIC {
    //     return Err("Invalid magic bytes in response".into());
    // }

    let remote_version = u32::from_be_bytes(response_version);

    // ... (Process the response, perform additional verification if needed)

    println!("Substrate handshake successful. Remote version: {}", remote_version);

    Ok(())
}


//main function
fn main() {
   //here handshake function will be called
   if let Err(err) = perform_substrate_handshake("127.0.0.1:9944") {
        eprintln!("Error: {}", err);
    }
}
