use clap::Parser;

#[derive(Debug, clap::Parser)]
struct Args {
    output: String,
}

fn main() {
    println!("Hello from rtw-cli!");

    let args = Args::parse();
    dbg!(args);
}
