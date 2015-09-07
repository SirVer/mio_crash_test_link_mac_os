extern crate mio;

use mio::unix::UnixSocket;

#[no_mangle]
pub extern "C" fn connect() {
    println!("In connect.");
    let _ = UnixSocket::stream().unwrap().connect("/tmp");
}
