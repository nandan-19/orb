use std::{
    fs::File,
    io::{self, Read, Write},
    net::TcpStream,
    path::Path,
};

pub fn send_file_data(mut stream: TcpStream, file_path: &str) -> io::Result<()> {
    let path = Path::new(file_path);
    let mut file = File::open(path)?;
    let file_name = path.file_name().unwrap().to_string_lossy();
    let file_size = path.metadata()?.len();

    stream.write_all(format!("File Name: {}, File size: {}", file_name, file_size).as_bytes())?;

    let mut confimation_buf = [0u8; 64];
    let len = stream.read(&mut confimation_buf)?;
    let msg = String::from_utf8_lossy(&confimation_buf[..len]);
    if msg.starts_with("y") {
        println!("Sending {} ({} bytes...", file_name, file_size);
        stream.write_all(&file_size.to_be_bytes())?;

        let name_bytes = file_name.as_bytes();
        stream.write_all(&(name_bytes.len() as u32).to_be_bytes())?;

        stream.write_all(name_bytes)?;

        let mut buffer = [0u8; 4096];

        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            stream.write_all(&buffer[..bytes_read])?;
        }

        println!("File sent successfully!");
        Ok(())
    } else {
        Ok(())
    }
}

pub fn receive_file_data(mut stream: TcpStream) -> io::Result<()> {
    let mut confirm_buf = [0u8; 1024];
    let len_confirm_buf = stream.read(&mut confirm_buf)?;
    let confirm_msg = String::from_utf8_lossy(&confirm_buf[..len_confirm_buf]);
    println!("Wanna receive this file: {} (y/n) ", confirm_msg);
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed reading choice");
    if choice.trim() == "y" {
        stream.write_all("y".as_bytes())?;

        let mut size = [0u8; 8];
        stream.read_exact(&mut size)?;
        let file_size = u64::from_be_bytes(size);

        let mut name_len_buf = [0u8; 4];
        stream.read_exact(&mut name_len_buf)?;
        let name_len = u32::from_be_bytes(name_len_buf);

        let mut name_buf = vec![0u8; name_len as usize];
        stream.read_exact(&mut name_buf)?;
        let file_name = String::from_utf8_lossy(&name_buf).to_string();

        println!("Receiving file: {} ({}) bytes)", file_name, file_size);

        let mut file = File::create(&file_name)?;

        let mut handle = stream.take(file_size);
        io::copy(&mut handle, &mut file)?;
        println!("File received and saved as {}", file_name);
        Ok(())
    } else {
        stream.write_all("n".as_bytes())?;
        Ok(())
    }
}
