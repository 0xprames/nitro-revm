pub mod command_parser;
pub mod encryption_helper;
pub mod protocol_helpers;
pub mod utils;

use alloy_primitives::{Address, U256};
use command_parser::ClientArgs;
use encryption_helper::{get_kms_client, EncryptionHelper};
use protocol_helpers::{send_loop, send_u64};

use nix::sys::socket::{connect, shutdown, socket};
use nix::sys::socket::{AddressFamily, Shutdown, SockAddr, SockFlag, SockType};
use nix::unistd::close;
use std::os::unix::io::{AsRawFd, RawFd};
use std::str::FromStr;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Payload {
    sender: Address,
    amount: U256,
}

// Maximum number of connection attempts
const MAX_CONNECTION_ATTEMPTS: usize = 5;

struct VsockSocket {
    socket_fd: RawFd,
}

impl VsockSocket {
    fn new(socket_fd: RawFd) -> Self {
        VsockSocket { socket_fd }
    }
}

impl Drop for VsockSocket {
    fn drop(&mut self) {
        shutdown(self.socket_fd, Shutdown::Both)
            .unwrap_or_else(|e| eprintln!("Failed to shut socket down: {:?}", e));
        close(self.socket_fd).unwrap_or_else(|e| eprintln!("Failed to close socket: {:?}", e));
    }
}

impl AsRawFd for VsockSocket {
    fn as_raw_fd(&self) -> RawFd {
        self.socket_fd
    }
}

/// Initiate a connection on an AF_VSOCK socket
fn vsock_connect(cid: u32, port: u32) -> Result<VsockSocket, String> {
    let sockaddr = SockAddr::new_vsock(cid, port);
    let mut err_msg = String::new();

    for i in 0..MAX_CONNECTION_ATTEMPTS {
        let vsocket = VsockSocket::new(
            socket(
                AddressFamily::Vsock,
                SockType::Stream,
                SockFlag::empty(),
                None,
            )
            .map_err(|err| format!("Failed to create the socket: {:?}", err))?,
        );
        match connect(vsocket.as_raw_fd(), &sockaddr) {
            Ok(_) => return Ok(vsocket),
            Err(e) => err_msg = format!("Failed to connect: {}", e),
        }

        // Exponentially backoff before retrying to connect to the socket
        std::thread::sleep(std::time::Duration::from_secs(1 << i));
    }

    Err(err_msg)
}

pub async fn client(args: ClientArgs) -> Result<(), String> {
    let vsocket = vsock_connect(args.cid, args.port)?;
    let fd = vsocket.as_raw_fd();

    let data = Payload {
        sender: Address::from_str(&"0xdafea492d9c6733ae3d56b7ed1adb60692c98bc5".to_string())
            .unwrap(),
        amount: U256::from(42),
    };
    let data = serde_json::to_string(&data).unwrap();
    let buf = data.as_bytes();
    // encrypt data
    let client = get_kms_client().await.unwrap();
    let encryption_payload = EncryptionHelper::new(client, args.key_id)
        .encrypt_data(buf)
        .await;

    println!("encryption_payload: {:#?}", encryption_payload);
    let len: u64 = buf.len().try_into().map_err(|err| format!("{:?}", err))?;
    send_u64(fd, len)?;
    send_loop(fd, buf, len)?;

    Ok(())
}
