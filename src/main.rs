use nix::unistd::read;
use nix::NixPath;
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use nix::errno::Errno;

const BUF_SIZE: usize = 1024;

fn main() -> Result<(), String> {
    cat()
}

fn cat() -> Result<(), String> {
    let env = std::env::args();
    if env.len() <= 1 {
        return Result::Err(String::from("no filename provided"));
    }

    for arg in env.skip(1) {
        let fd = match open(arg.as_str(), OFlag::O_RDONLY, Mode::empty()) {
            Ok(f) => f,
            Err(e) => match e {
                Errno::ENOENT => return Result::Err(format!("file '{}' not found", arg)),
                _ => return Result::Err(format!("some unknown error happened: {:?}", e)),
            }
        }; 

        let mut buf = [0u8; BUF_SIZE];
        read(fd, &mut buf).unwrap();
        println!("{}",  String::from_utf8_lossy(&buf));
    }

    return Ok(());
}
