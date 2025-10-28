use clap::Parser;
use rtw::{RenderConfig, render_scene};

#[derive(Debug, clap::Parser)]
struct Args {
    output: String,
    #[arg(long, short)]
    multi_threaded: bool,
    #[arg(long, short)]
    scene: String,
}

impl From<Args> for RenderConfig {
    fn from(value: Args) -> Self {
        Self {
            multi_threaded: value.multi_threaded,
            output_file: value.output,
        }
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let scene = args.scene.to_owned();

    render_scene(&scene, &RenderConfig::from(args))?;

                Ok(())
}
