use std::fmt::Display;
use clap::{Parser, Subcommand};
use image::{ImageBuffer, RgbImage};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
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

#[derive(Clone)]
struct Line(Vec<bool>);

impl Line {
    fn center_enabled(size: usize) -> Self {
        let mut vec = vec![false; size];
        vec[size / 2] = true;
        Self(vec)
    }

    fn next(&self, rule: u8) -> Self {
        Self(
            self.0.iter().enumerate().map(|(i, _)| {
                let left = if i == 0 { &false } else { self.0.get(i - 1).unwrap() };
                let center = self.0.get(i).unwrap();
                let right = self.0.get(i + 1).unwrap_or(&false);
                let index = u8::from(*left) << 2 | u8::from(*center) << 1 | u8::from(*right);
                rule >> index & 1 == 1
            }).collect()
        )
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &b in &self.0 {
            write!(f, "{}", if b { "█" } else { " " })?;
        }
        Ok(())
    }
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Cli { width, steps, rule, raw } => {
            let mut line = Line::center_enabled(width.unwrap_or_else(|| termsize::get().unwrap().cols.into()));
            let rule = rule;
            
            for _ in 0..steps.unwrap_or_else(|| termsize::get().unwrap().rows.into()) {
                if raw {
                    line.0.iter().for_each(|&b| print!("{}", if b { "1" } else { "0" }));
                } else {
                    println!("{line}");
                }
                line = line.next(rule);
            }
        },
        Commands::Image { width, steps, rule } => {
            let mut img: RgbImage = ImageBuffer::new(width, steps);

            let mut line = Line::center_enabled(width as usize);
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
