use bpaf::Bpaf;
use std::net::{IpAddr, Ipv4Addr};

const MAX: u16 = 65535;
const IPFALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

#[derive(Debug, Clone, Bpaf)]
#[bpaf(
    options,
    version("0.1.0"),
    descr("Fast async TCP port sniffer"),
    header("port_sniffer 0.1.0\nnsvoriginals 🦀"),
    footer("Built with Rust & Tokio")
)]
pub struct Arguments {
    #[bpaf(
        long,
        short,
        argument("ADDRESS"),
        fallback(IPFALLBACK),
        display_fallback
    )]
    pub address: IpAddr,

    #[bpaf(
        long("start-port"),
        short('s'),
        argument("START_PORT"),
        fallback(1u16),
        display_fallback
    )]
    pub start_port: u16,

    #[bpaf(
        long("end-port"),
        short('e'),
        argument("END_PORT"),
        fallback(MAX),
        display_fallback
    )]
    pub end_port: u16,
}

pub fn parse() -> Arguments {
    arguments().run()
}
