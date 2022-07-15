use clap::Parser;
mod get;

#[derive(Parser)]
struct CLI {
    host: String,
}

fn main() {
    let args = CLI::parse();
    let host: &str = &*args.host;

    let _i = get::get(host);
}
