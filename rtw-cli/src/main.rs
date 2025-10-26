use clap::Parser;
use rtw::render_scene;

#[derive(Debug, clap::Parser)]
struct Args {
    output: String,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    render_scene(&args.output)?;

    Ok(())
}
