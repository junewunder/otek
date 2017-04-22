#![allow(dead_code, unused_variables, unused_must_use, unused_imports)]

/// # Otek.
///
/// ### Usage:
/// otek list
/// otek add <path> <name>
/// otek create <name> [-v VARIABLES...]
///
/// ### Options:
/// -h --help show this text
/// -v --vars key="value" variables to replace in the template project

#[macro_use]
extern crate nom;
extern crate regex;

mod syntax;
mod state;

use std::io::prelude::*;
use std::fs;
use std::path::{ Path, PathBuf };
use std::io::Result;
use std::fs::File;

// use syntax::OtekSyntax;
use state::OtekState;

fn main() {
    println!("");

    let to = Path::new("/Users/jacob/Desktop/tocopy");
    let from = Path::new("/Users/jacob/Desktop/thecopy");

    copy_folder(to, from);

    println!("");
}

fn copy_folder(from_path: &Path, to_path: &Path) -> Result<()> {
    for entry in fs::read_dir(from_path)? {

        let file = entry?;
        let copy_to = to_path.join(file.file_name());

        if file.path().is_dir() {

            fs::create_dir(&copy_to);
            return copy_folder(&file.path(), &copy_to)

        } else {

            fs::copy(&file.path(), &copy_to);

        }
    }

    Ok(())
}

// fn parse_file(state: OtekState, syntax: OtekSyntax) {
//
// }

/// Another doc
fn create_otek_folder() {

}

#[test]
fn test_copy_folder() {
    use std::process::Command;

    fs::create_dir(&Path::new("/tmp/otektest"));
    fs::create_dir(&Path::new("/tmp/otekcopy"));
    fs::create_dir(&Path::new("/tmp/otektest/testfolder"));

    File::create(Path::new("/tmp/otektest/somefile"));
    File::create(Path::new("/tmp/otektest/testfolder/afile"));

    copy_folder(&Path::new("/tmp/otektest"), &Path::new("/tmp/otekcopy"));

    let output = Command::new("find").arg("/tmp/otekcopy").output().unwrap();
    let expected_output = "/tmp/otekcopy\n/tmp/otekcopy/somefile\n/tmp/otekcopy/testfolder\n/tmp/otekcopy/testfolder/afile\n";

    // println!("{:?}", output.stdout.into_iter().collect());

    // assert_eq!(output.stdout, expected_output);
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::prelude::*;

    use std::fs;
    use std::path::{ Path, PathBuf };
    use std::io::Result;

    use std::fs::File;
}
