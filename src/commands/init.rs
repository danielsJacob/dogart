use std::fs;
use std::path::Path;
use crate::assets::*;
use crate::signature::SIGNATURE;

pub fn run(picture: bool, normal: bool, enlarged: bool, force: bool) {
    if enlarged {
        create_text("TANNER_ASCII_LARGE.txt", ENLARGED_ASCII, force);
    } else if normal {
        create_text("TANNER_ASCII.txt", NORMAL_ASCII, force);
    } else if picture {
        create_image("TANNER.png", TANNER_PICTURE, force);
    } else {
        println!("Please specify -p (picture), -t (normal ASCII), or -et (enlarged ASCII).");
    }
}

fn create_text(path: &str, content: &str, force: bool) {
    guard(path, force);

    let file_content = format!("{}\n\n{}", content, SIGNATURE);
    fs::write(path, file_content).expect("Failed to write file");
    println!("{path} created.");
}

fn create_image(filename: &str, data: &[u8], force: bool) {
    guard(filename, force);
    fs::write(filename, data).expect("Failed to write image");
    println!("{filename} created.");
}

fn guard(path: &str, force: bool) {
    if Path::new(path).exists() && !force {
        panic!("File already exists. Use --force to overwrite.");
    }
}