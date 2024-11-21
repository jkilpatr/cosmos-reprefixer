use deep_space::address::Address as CosmosAddress;
use clap::Parser;

fn re_prefix(input: CosmosAddress, prefix: String) {
    let re_prefixed = input.to_bech32(prefix).unwrap();
    println!("{} -> {}", input, re_prefixed);
    let re_prefixed: CosmosAddress = re_prefixed.parse().unwrap();
    assert_eq!(re_prefixed.to_vec(), input.to_vec())
}

#[derive(Parser)]
struct Args {
    #[clap(long)]
    address: String,
    #[clap(long)]
    prefix: String,
}

fn main() {
    let args = Args::parse();
    let address: CosmosAddress = args.address.parse().unwrap();
    re_prefix(address, args.prefix);
}