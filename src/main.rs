use std::mem;
use std::net::UdpSocket;
use std::os::raw::c_double;

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

fn main() {
    let socket_in = UdpSocket::bind("0.0.0.0:9003").expect("couldn't bind receiving socket");
    let socket_out = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind sending socket");

    let buf_out = [127; 16];
    let mut buf_in = [0; 136];

    loop {
        socket_out.send_to(&buf_out, "127.0.0.1:9002").expect("couldn't send data");
        match socket_in.recv_from(&mut buf_in) {
            Ok(_) => {
                let packet: FDMPacket = unsafe { mem::transmute(buf_in) };
                println!("{:?}", packet);
            },
            Err(e) => {
                println!("couldn't recieve a datagram: {}", e);
            }
        }
    }
}
