use fsplay::{Config, JsFramework::*};
use include_dir::{include_dir, Dir};
use std::process::Command;

const TEMPLATES_DIR: Dir = include_dir!("./templates");

fn main() -> std::io::Result<()> {
    // // of course, you can retrieve a file by its full path
    // let gitignore = PROJECT_DIR.get_file("astro/.gitignore").unwrap();
    // // you can also inspect the file's contents
    // let body = gitignore.contents_utf8().unwrap();
    // println!("{}", body);
    // assert!(body.contains("dependencies"));

    let config = Config::new()?;

    match config.project_framework {
        Astro => {
            create_project(&config.project_name, "astro");
            println!("Astro project created");
        }

        SveltekitRegular => {
            create_project(&config.project_name, "sveltekit-regular");
            println!("SvelteKit Regular project created");
        }

        SveltekitFullstack => {
            create_project(&config.project_name, "sveltekit-regular");
            println!("SvelteKit Fullstack project created");
        }
    }

    Ok(())
}

fn create_project(name: &str, template_dir_name: &str) {
    // create a new directory with the project name
    std::fs::create_dir(name).unwrap();

    let template_dir = TEMPLATES_DIR.get_dir(template_dir_name).unwrap();

    // copy the contents of the astro template into the new directory
    copy_dir(template_dir, name).unwrap();
}

fn copy_dir(dir: &Dir, dest: &str) -> std::io::Result<()> {
    for file in dir.files() {
        let path = file.path();
        let contents = file.contents();
        let dest_path = format!(
            "{}/{}",
            dest,
            path.to_string_lossy().split('/').last().unwrap()
        );

        std::fs::write(&dest_path, contents)?;
    }

    for subdir in dir.dirs() {
        let path = subdir.path();
        let dest_path = format!(
            "{}/{}",
            dest,
            path.to_string_lossy().split('/').last().unwrap()
        );

        std::fs::create_dir(&dest_path)?;
        copy_dir(subdir, &dest_path)?;
    }

    Ok(())
}

#[allow(dead_code)]
fn run_dum() {
    let mut list_dir = Command::new("dum");
    list_dir.arg("-h");

    // Execute `ls` in the current directory of the program.
    list_dir.status().expect("process failed to execute");

    println!();

    // Change `ls` to execute in the root directory of the machine.
    // list_dir.current_dir("/");

    // And then execute `ls` again but in the root directory.
    // list_dir.status().expect("process failed to execute");
}
