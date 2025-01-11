mod presentation;

use clap::Parser;
use presentation::FrostyPine;

fn main() {
    let args: FrostyPine = FrostyPine::parse();
}
