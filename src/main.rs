mod cli;
mod scanner;

use scanner::scan_ports;

#[tokio::main]
async fn main() {
    let opts = cli::parse();
    println!(
        "Scanning {} from {} to {} ",
        opts.address, opts.start_port, opts.end_port
    );
    let open_ports = scan_ports(opts.address, opts.start_port, opts.end_port).await;

    println!("\nOpen Ports:");
    for port in open_ports {
        println!("{} is open", port);
    }
}
