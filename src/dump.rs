use std::{
    error::Error,
    fs::{copy, create_dir_all, write},
    process,
};

fn main() {}

fn path_concat(s: &[&str]) -> String {
    let mut paths = String::from(".");

    for i in s {
        paths.push('/');
        paths.push_str(i);
    }

    paths
}

#[allow(unused)]
fn rando() -> Result<(), Box<dyn Error>> {
    const DIR: &str = "./cool/yo";
    // :NOTE: `create_dir_all` doesn't completely destroy the directories
    // it only creates what doesn't existe
    match create_dir_all(DIR) {
        Ok(_) => {
            println!("directories created!");

            // let base_write = format!("{DIR}/new.ts");

            write(
                path_concat(&[DIR, "new.ts"]),
                "let goodloord = \"This shit is crezy\"",
            )
            .unwrap_or_else(|err| {
                eprint!("An error occured: {err}");
                process::exit(1);
            });

            copy("./copy/test.js", path_concat(&["cool", "fucker"]))
                .unwrap_or_else(|err| {
                    eprint!("An error occured: {err}");
                    process::exit(1);
                });
        }
        Err(e) => {
            eprint!("An error occured: {e}");
            process::exit(1);
        }
    };

    Ok(())
}

extern crate cfonts;

use cfonts::{
    say, Align, BgColors, Colors::Rgb, Env, Fonts, Options, Rgb::Val,
};

fn show_cfont() {
    // using cfonts may be a bit of overkill 😅
    say(Options {
        text: String::from("adroyt"),
        font: Fonts::FontBlock,
        colors: vec![Rgb(Val(148, 95, 57))],
        background: BgColors::Transparent,
        align: Align::Left,
        letter_spacing: 1,
        line_height: 1,
        spaceless: false,
        max_length: 0,
        gradient: Vec::new(),
        independent_gradient: false,
        transition_gradient: false,
        env: Env::Cli,
        ..Options::default()
    });
}
