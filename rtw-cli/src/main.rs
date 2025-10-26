use clap::Parser;
use rtw::{RenderConfig, render_scene};

#[derive(Debug, clap::Parser)]
struct Args {
    output: String,
    #[arg(long, short)]
    single_thread: bool,
}

impl From<Args> for RenderConfig {
    fn from(value: Args) -> Self {
        Self {
            single_threaded: value.single_thread,
            output_file: value.output,
        }
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    render_scene(&RenderConfig::from(args))?;

    Ok(())
}
