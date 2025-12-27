use std::time::Duration;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let port_name = "/dev/ttyACM1";  
    let baud_rate = 9600;

    let serial_port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(1000))
        .open()?;

    let reader = io::BufReader::new(serial_port);

    println!("Listening on {}", port_name);

    for line in reader.lines() {
        match line {
            Ok(data) => println!("Received: {}", data),
            Err(e) => eprintln!("Error reading: {:?}", e),
        }
    }

    Ok(())
}
