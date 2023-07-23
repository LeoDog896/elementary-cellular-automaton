use clap::{Parser, Subcommand};
use elementary_cellular_automaton::Line;
use image::{ImageBuffer, RgbImage};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional input to use. If not provided, the center pixel will be enabled
    #[arg(short, long)]
    input: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Cli {
        /// The horizontal size of the board. Default is the CLI column amount
        #[arg(short, long)]
        width: Option<usize>,

        /// The height (steps) of the board. Default is the CLI row amount
        #[arg(short, long)]
        steps: Option<usize>,

        /// The rule to use
        #[arg(short, long)]
        rule: u8,

        /// If the output should be raw
        #[arg(short, long)]
        raw: bool,
    },
    Image {
        /// The horizontal size of the board.
        #[arg(short, long)]
        width: u32,

        /// The vertical size of the board.
        #[arg(short, long)]
        steps: u32,

        /// The rule to use
        #[arg(short, long)]
        rule: u8,
    },
}

fn gradient(from: image::Rgb<u8>, to: image::Rgb<u8>, step: f64) -> image::Rgb<u8> {
    let r = from.0[0] as f64 + (to.0[0] as f64 - from.0[0] as f64) * step;
    let g = from.0[1] as f64 + (to.0[1] as f64 - from.0[1] as f64) * step;
    let b = from.0[2] as f64 + (to.0[2] as f64 - from.0[2] as f64) * step;
    image::Rgb([r as u8, g as u8, b as u8])
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Cli {
            width,
            steps,
            rule,
            raw,
        } => {
            let width = width.unwrap_or_else(|| termsize::get().unwrap().cols.into());
            let mut line = args.input.map_or_else(
                || Line::center_enabled(width),
                Line::from_string,
            );

            assert_eq!(
                line.len(),
                width,
                "The input must be the same size as the width"
            );

            let rule = rule;

            for _ in 0..steps.unwrap_or_else(|| termsize::get().unwrap().rows.into()) {
                if raw {
                    line.0
                        .iter()
                        .for_each(|b| print!("{}", if *b { "1" } else { "0" }));
                } else {
                    println!("{line}");
                }
                line = line.next(rule);
            }
        }
        Commands::Image { width, steps, rule } => {
            let mut img: RgbImage = ImageBuffer::new(width, steps);

            let mut line = args.input.map_or_else(
                || Line::center_enabled(width as usize),
                |input| Line::from_string(input),
            );

            assert_eq!(
                line.len(),
                width as usize,
                "The input must be the same size as the width"
            );

            let mut current_y = 0;

            for (x, y, pixel) in img.enumerate_pixels_mut() {
                if y != current_y {
                    line = line.next(rule);
                }
                *pixel = if line.0[x as usize] {
                    gradient(
                        image::Rgb([34, 146, 164]),
                        image::Rgb([217, 108, 6]),
                        y as f64 / steps as f64,
                    )
                } else {
                    image::Rgb([0, 21, 36])
                };
                current_y = y;
            }

            img.save("out.png").unwrap();
        }
    }
}
