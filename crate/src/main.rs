use clap::Parser;

/// Look up the ISO 3166-1 alpha-2 country code for IPv4 or IPv6 addresses.
#[derive(Parser)]
#[command(name = "iptocc", version, about, arg_required_else_help = true)]
struct Cli {
    /// IP address(es) to look up (IPv4 or IPv6).
    #[arg(required = true)]
    addresses: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    if cli.addresses.len() == 1 {
        match iptocc::country_code(&cli.addresses[0]) {
            Some(cc) => println!("{cc}"),
            None => std::process::exit(1),
        }
        return;
    }
    let results = iptocc::country_codes(&cli.addresses);
    for (addr, result) in cli.addresses.iter().zip(results.iter()) {
        match result {
            Some(cc) => println!("{addr} {cc}"),
            None => println!("{addr} -"),
        }
    }
}
