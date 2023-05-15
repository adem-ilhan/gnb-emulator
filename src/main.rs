pub mod GtpHeader;
use rand::Rng;
use std::net::UdpSocket;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(
        short,
        long,
        help = "Generate same teid value with different gnb source"
    )]
    random: bool,

    #[structopt(
        short,
        long,
        value_name = "COUNT",
        help = "Number of packet to generate"
    )]
    count: Option<usize>,

    #[structopt(short, long, help = "Generate the same teid with same gnb souce")]
    same: bool,

    #[structopt(short, long, value_name = "TEID", help = "Teid value")]
    teid: Option<u32>,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let mut message_count = 0;
    let mut icmp = vec![
        0x45, 0x00, 0x00, 0x54, 0x0b, 0x8b, 0x40, 0x00, 0x40, 0x01, 0x5c, 0xff, 0xc0, 0xa8,
        0x01, //c0 a8 01 67 ip source 192.168.1.103
        0x67, 0x08, 0x08, 0x08, 0x08, 0x08, 0x00, 0xea, 0x67, 0x00, 0x01, 0x00, 0x02, 0x30, 0x18,
        0x61, 0x64, 0x00, 0x00, 0x00, 0x00, 0xbb, 0x45, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10,
        0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
        0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e,
        0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
    ];
    let mut message_header = GtpHeader::GtpHeader::new();
    message_header.msg_len = icmp.len() as u16;
    let mut res: Vec<u8> = message_header.to_bytes();
    res.append(&mut icmp);

    res[4] = 0xAA;
    res[5] = 0xAA;
    res[6] = 0xAA;
    res[7] = 0xAA;

    match opt.teid {
        Some(t) => {
            let mut teid: Vec<u8> = Vec::new();
            teid.append(&mut t.to_be_bytes().to_vec());
            res[4] = teid[0];
            res[5] = teid[1];
            res[6] = teid[2];
            res[7] = teid[3];
        }
        None => {
            println!("teid value is 0xAA 0xAA 0xAA 0xAA");
        }
    }

    match opt.count {
        Some(c) => message_count = c,
        None => {
            println!("error: --count is required");
            std::process::exit(1);
        }
    }

    if opt.same {
        let socket = UdpSocket::bind("127.0.0.2:2152").expect("Error while binding");
        for i in 0..message_count {
            let _ = socket.send_to(&res, "127.0.0.1:2152");
        }
    }

    if opt.random {
        let mut source: Vec<u8> = vec![127, 0, 0, 4]; // we cant take random ip, we need s.point

        for i in 0..message_count {
            if source[3] == 255 {
                std::process::exit(1);
            }
            let mut ip_string: String = source
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(".");
            dbg!(&ip_string);
            ip_string.push_str(":2152");
            let socket = UdpSocket::bind(ip_string).expect("error while bind");
            let _ = socket.send_to(&res, "127.0.0.1:2152");
            source[3] += 1;
        }
    }
    Ok(())
}
