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
        raw: bool
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
        rule: u8
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Cli { width, steps, rule, raw } => {
            let width = width.unwrap_or_else(|| termsize::get().unwrap().cols.into());
            let mut line = if let Some(input) = args.input {
                Line::from_string(input)
            } else {
                Line::center_enabled(width as usize)
            };
            
            assert_eq!(line.len(), width as usize, "The input must be the same size as the width");

            let rule = rule;
            
            for _ in 0..steps.unwrap_or_else(|| termsize::get().unwrap().rows.into()) {
                if raw {
                    line.0.iter().for_each(|b| print!("{}", if *b { "1" } else { "0" }));
                } else {
                    println!("{line}");
                }
                line = line.next(rule);
            }
        },
        Commands::Image { width, steps, rule } => {
            let mut img: RgbImage = ImageBuffer::new(width, steps);

            let mut line = if let Some(input) = args.input {
                Line::from_string(input)
            } else {
                Line::center_enabled(width as usize)
            };

            assert_eq!(line.len(), width as usize, "The input must be the same size as the width");

            let mut current_y = 0;

            for (x, y, pixel) in img.enumerate_pixels_mut() {
                if y != current_y {
                    line = line.next(rule);
                }
                *pixel = image::Rgb([u8::from(line.0[x as usize]) * 255, 0, 0]);
                current_y = y;
            }

            img.save("out.png").unwrap();
        }
    }
}
