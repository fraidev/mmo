use tokio::{net::{TcpListener, TcpStream}, io::{AsyncReadExt, AsyncWriteExt}};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8080";

    let listener = TcpListener::bind(addr).await?;
    println!("Server listening on {}", addr);

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = process(socket).await {
                println!("An error occurred: {}", e);
            }
        });
    }
}

async fn process(socket: TcpStream) -> Result<(), Box<dyn Error>> {
    let (mut reader, mut writer) = socket.into_split();

    // Read username from the client
    // let mut username = String::new();
    //
    // u8 array size 100
    // let mut u8_array = [0; 100];
    let mut msg = [8; 320];
    
    // reader.read_exact(&mut u8_array).await?;
    let a = reader.read(&mut msg).await?;
    println!("a: {:?}", a);

    println!("msg: {:?}", msg);
    // Headers:
    // 2 bytes for unencrypted message size
    // 4 bytes for checksum
    // 2 bytes for encrypted message size
    let unencrypted_msg_size = u16::from_be_bytes([msg[0], msg[1]]);
    let checksum = u32::from_be_bytes([msg[2], msg[3], msg[4], msg[5]]);
    let encrypted_msg_size = u16::from_be_bytes([msg[6], msg[7]]);

    println!("unencrypted_msg_size: {:?}", unencrypted_msg_size);
    println!("checksum: {:?}", checksum);
    println!("encrypted_msg_size: {:?}", encrypted_msg_size);

    let os = u16::from_be_bytes([msg[8], msg[9]]);
    println!("os: {:?}", os);


    let version = u16::from_be_bytes([msg[10], msg[11]]);
    println!("version: {:?}", version);


	 //  4 bytes: client version (971+)
	 //  12 bytes: dat, spr, pic signatures (4 bytes each)
	 //  1 byte: preview world(971+)

    let client_version = u32::from_be_bytes([msg[12], msg[13], msg[14], msg[15]]);
    let dat_signature = u32::from_be_bytes([msg[16], msg[17], msg[18], msg[19]]);
    let spr_signature = u32::from_be_bytes([msg[20], msg[21], msg[22], msg[23]]);
    let pic_signature = u32::from_be_bytes([msg[24], msg[25], msg[26], msg[27]]);
    let preview_world = msg[28];

    println!("client_version: {:?}", client_version);
    println!("dat_signature: {:?}", dat_signature);
    println!("spr_signature: {:?}", spr_signature);
    println!("pic_signature: {:?}", pic_signature);
    println!("preview_world: {:?}", preview_world);



    // Read password from the client
    // let mut password = String::new();
    // reader.read_to_string(&mut password).await?;

    // // Check if the username and password are valid
    // let is_valid = check_credentials(&username, &password);

    // // Send the authentication result to the client
    // let response = if is_valid {
    //     "Authentication successful"
    // } else {
    //     "Authentication failed"
    // };
    // writer.write_all(response.as_bytes()).await?;
    writer.shutdown().await?;

    Ok(())
}

fn check_credentials(username: &str, password: &str) -> bool {
    // Implement your authentication logic here
    // For simplicity, this example checks if the username is "admin" and the password is "password"
    username == "admin" && password == "password"
}
