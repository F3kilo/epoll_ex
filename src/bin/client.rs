use std::io::Read;
use std::net::TcpStream;
use std::os::fd::AsRawFd;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:55331").unwrap();
    stream.set_nonblocking(true).unwrap();
    let raw_fd = stream.as_raw_fd();

    let ep = epoll::create(false).unwrap();
    epoll::ctl(
        ep,
        epoll::ControlOptions::EPOLL_CTL_ADD,
        raw_fd,
        epoll::Event {
            events: (epoll::Events::EPOLLIN).bits(),
            data: 0,
        },
    )
    .unwrap();

    let mut buf = [epoll::Event::new(epoll::Events::empty(), 0); 1];
    while epoll::wait(ep, -1, &mut buf).is_ok() {
        let event = buf[0];
        println!("Event from epoll: {:?}", event);

        let events = epoll::Events::from_bits(event.events).unwrap();
        if events.contains(epoll::Events::EPOLLHUP) {
            println!("connection dropped");
            break;
        }

        let mut buf = [0u8; 1];
        if stream.read_exact(&mut buf).is_ok() {
            println!("Data red: {}", buf[0]);
        } else {
            println!("No more data");
            _ = stream.shutdown(std::net::Shutdown::Both);
        };
    }
}
