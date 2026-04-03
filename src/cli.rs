use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dogart")]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // inject tanner
    Init {
        #[arg(short = 'p', long = "picture")]
        picture: bool,

        #[arg(short = 't', long = "text")]
        normal: bool,

        // -e for enlarged, -et will combine e/t and run enlarged first
        #[arg(short = 'e', long = "enlarged", alias = "et")]
        enlarged: bool,

        #[arg(long)]
        force: bool,
    },

    // open ascii in clipboard for editor paste
    Copy {
        #[arg(short = 's', long = "small")]
        small: bool,

        #[arg(short = 'l', long = "large")]
        large: bool,
    },

    // verify tanner existence
    Verify,

    // remove tanner existence | never will implement this :)
    Remove,
}