use std::net::TcpStream;
use std::io::Read;
use ssh2::Session;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Establish a TCP connection to the SSH server
    let tcp = TcpStream::connect("130.60.24.52:22")?;
    
    // Create an SSH session and authenticate with the server
    let mut sess = Session::new()?;
    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    
    // Authenticate with the username and password
    //sess.userauth_password("username", "password")?;
    sess.userauth_password("mimeul", "WjC!7oGbw+KuhaufMond")?;

    // Open a channel and execute a command
    let mut channel = sess.channel_session()?;
    channel.exec("ls")?;
    
    // Read the output
    let mut s = String::new();
    channel.read_to_string(&mut s)?;
    println!("{}", s);
    
    // Close the channel
    channel.close()?;
    channel.wait_close()?;
    Ok(())
}
