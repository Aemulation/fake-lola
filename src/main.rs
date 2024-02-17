use std::io::prelude::*;
use std::os::unix::net::UnixListener;
use std::time::Duration;

use rmp_serde::encode;
use serde::Serialize;

const LOLA_BUFFER_SIZE: usize = 896;
const LOLA_PATH: &str = "/tmp/robocup";

#[derive(Default, Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct LolaNaoState<'a> {
    stiffness: [f32; 25],
    position: [f32; 25],
    temperature: [f32; 25],
    current: [f32; 25],
    battery: [f32; 4],
    accelerometer: [f32; 3],
    gyroscope: [f32; 3],
    angles: [f32; 2],
    sonar: [f32; 2],
    f_s_r: [f32; 8],
    touch: [f32; 14],
    status: [i32; 25],
    #[serde(borrow)]
    robot_config: [&'a str; 4],
}

fn run(listener: &UnixListener, write_buf: &mut [u8]) -> std::io::Result<()> {
    let mut read_buf = Vec::new();

    match listener.accept() {
        Ok((mut socket, addr)) => {
            socket.set_read_timeout(Some(Duration::from_millis(1)))?;
            println!("Got a client: {:?} - {:?}", socket, addr);

            loop {
                socket.write_all(write_buf)?;

                let _ = socket.read_to_end(&mut read_buf);
                read_buf.clear();
            }
        }
        Err(error) => println!("accept: {:?}", error),
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let _ = std::fs::remove_file(LOLA_PATH);
    let listener = UnixListener::bind(LOLA_PATH)?;

    let mut write_buf = [0u8; LOLA_BUFFER_SIZE];
    let lola_state = LolaNaoState::default();
    encode::write_named(&mut write_buf.as_mut_slice(), &lola_state).unwrap();

    loop {
        let _ = run(&listener, &mut write_buf);
    }
}
