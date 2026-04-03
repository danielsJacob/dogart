use crate::assets::{ENLARGED_ASCII, NORMAL_ASCII};
use arboard::Clipboard;

pub fn run(small: bool, large: bool) {
    if large {
        copy_text(ENLARGED_ASCII);
        println!("ENLARGED ASCII copied to clipboard.");
    } else if small {
        copy_text(NORMAL_ASCII);
        println!("NORMAL ASCII copied to clipboard.");
    } else {
        println!("Please specify -s (small) or -l (large) for copy.");
    }
}

fn copy_text(content: &str) {
    let mut clipboard = Clipboard::new().expect("Could not access clipboard");
    clipboard.set_text(content.to_string()).expect("Could not copy text");
}