use std::mem;
use std::net::UdpSocket;
use std::os::raw::{c_double, c_float};

#[repr(C)]
#[derive(Debug)]
pub struct FDMPacket {
    // packet timestamp
    timestamp: c_double,
    // IMU angular velocity
    angular: [c_double; 3],
    // IMU linear acceleration
    linear: [c_double; 3],
    // IMU quaternion orientation
    quaternion: [c_double; 4],
    // Model velocity in NED frame
    velocity: [c_double; 3],
    // Model position in NED frame
    position: [c_double; 3],
}

use dcmimu::DCMIMU;

type ServoPacket = [c_float; 16];

fn main() {
    let mut dcmimu = DCMIMU::new();

    let socket_in = UdpSocket::bind("0.0.0.0:9003").expect("couldn't bind receiving socket");
    let socket_out = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind sending socket");

    let mut buf_in = [0; 136];

    let mut command: ServoPacket = [0.0; 16];
    command[0] = 0.0;
    command[1] = 0.0;
    command[2] = 0.0;
    command[3] = 0.0;

    let buf_out: [u8; 64] = unsafe { mem::transmute(command) };

    let mut last_time: f32 = 0.0;
    socket_out
        .send_to(&buf_out, "127.0.0.1:9002")
        .expect("couldn't send data");
    match socket_in.recv_from(&mut buf_in) {
        Ok(_) => {
            let packet: FDMPacket = unsafe { mem::transmute(buf_in) };
            last_time = packet.timestamp as f32;
        }
        Err(e) => {
            println!("couldn't recieve a datagram: {}", e);
        }
    }

    loop {
        socket_out
            .send_to(&buf_out, "127.0.0.1:9002")
            .expect("couldn't send data");
        match socket_in.recv_from(&mut buf_in) {
            Ok(_) => {
                let packet: FDMPacket = unsafe { mem::transmute(buf_in) };
                let dt = (packet.timestamp as f32) - last_time;
                last_time = packet.timestamp as f32;
                let gyro = packet.angular;
                let accel = packet.linear;
                let (dcm, _gyro_biases) = dcmimu.update(
                    (gyro[0] as f32, gyro[1] as f32, gyro[2] as f32),
                    (accel[0] as f32, accel[1] as f32, accel[2] as f32),
                    dt,
                );
                println!(
                    "Roll: {:3.2}; yaw: {:3.2}; pitch: {:3.2}",
                    dcm.roll.to_degrees(),
                    dcm.yaw.to_degrees(),
                    dcm.pitch.to_degrees()
                );
            }
            Err(e) => {
                println!("couldn't recieve a datagram: {}", e);
            }
        }
    }
}
