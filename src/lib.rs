use console::style;
use console::Term;
// use ctrlc;
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input, MultiSelect};
use std::{
    fmt::{Debug, Display},
    io::Result as IoResult,
    process,
};
use CssFramework::*;
use JsFramework::*;

#[derive(Debug, Clone, Copy)]
pub enum JsFramework {
    SveltekitRegular,
    SveltekitFullstack,
    Astro,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CssFramework {
    Unocss,
    VanillaExtract,
    Tailwind,
}

// impl display trait that returns a string
impl Display for JsFramework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SveltekitRegular => write!(f, "Sveltekit Regular"),
            SveltekitFullstack => write!(f, "Sveltekit Fullstack"),
            Astro => write!(f, "Astro"),
        }
    }
}

impl Display for CssFramework {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unocss => write!(f, "Unocss"),
            VanillaExtract => write!(f, "Vanilla Extract"),
            Tailwind => write!(f, "Tailwind"),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub project_name: String,
    pub project_framework: JsFramework,
    pub css_frameworks: Vec<CssFramework>,
    pub backend_tools: Option<Vec<String>>,
}

impl Config {
    pub fn new() -> IoResult<Self> {
        play_intro();

        let project_name: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Project Name")
            .with_initial_text("")
            .default("adroyt-app".into())
            .validate_with(|input: &String| -> Result<(), String> {
                if input.contains(".") {
                    Err(format!("'{input}' is not a valid project name"))
                } else {
                    Ok(())
                }
            })
            .interact_text()?;

        let project_name = project_name.trim().to_string();

        let project_framework = get_project_framework()?;
        let css_frameworks = use_multiselect(UseMultiselectParams {
            options: vec![Unocss, VanillaExtract, Tailwind],
            prompt: "Select CSS Framework(s), (don't select both unocss & tailwind)",
            excerpts: Some([Unocss, Tailwind]),
            excerpt_err: "You can't select both unocss and tailwind",
            defaults: Some(&[true]),
        })?;

        let backend_tools = match project_framework {
            SveltekitFullstack => Some(use_multiselect(UseMultiselectParams {
                options: vec![
                    "lucia-auth".to_string(),
                    "prisma".to_string(),
                    "pocketbase".to_string(),
                    "trpc".to_string(),
                ],
                prompt: "Select Backend Package(s), (don't select both prisma & pocketbase)",
                excerpts: Some(["prisma".to_string(), "pocketbase".to_string()]),
                defaults: Some(&[false, false, false, true]),
                excerpt_err: "You can't select both prisma & pocketbase",
            })?),
            _ => None,
        };

        Ok(Config {
            project_name,
            project_framework,
            css_frameworks,
            backend_tools,
        })
    }
}

pub fn play_intro() {
    let colored_intro = style("Welcome to the adroyt cli!").blue();
    let colored_intro = colored_intro.bold();
    let intro_text = format!(
        "{colored_intro} \n\
        This wizard will set you up",
    );

    println!("{}", intro_text);
}

fn get_project_framework() -> IoResult<JsFramework> {
    let frameworks = [Astro, SveltekitRegular, SveltekitFullstack];
    let selection_index = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&frameworks)
        .with_prompt("Select Framework")
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection_index {
        Some(i) => Ok(frameworks[i]),
        None => process::exit(1),
    }
}

struct UseMultiselectParams<'a, T> {
    options: Vec<T>,
    prompt: &'a str,
    excerpts: Option<[T; 2]>,
    defaults: Option<&'a [bool]>,
    excerpt_err: &'a str,
}

fn use_multiselect<T: Display + Eq + Clone>(config: UseMultiselectParams<T>) -> IoResult<Vec<T>> {
    let selection_indices = MultiSelect::with_theme(&ColorfulTheme::default())
        .items(&config.options)
        .with_prompt(config.prompt)
        .defaults(match config.defaults {
            Some(v) => v,
            None => &[],
        })
        .interact_on_opt(&Term::stderr())?;

    match selection_indices {
        Some(indexes) => {
            let mut selected = vec![];
            for i in indexes {
                selected.push(config.options[i].to_owned())
            }

            if let Some(excerpts) = &config.excerpts {
                let mut itered = selected.iter();
                if itered.any(|e| e == &excerpts[0]) && itered.any(|e| e == &excerpts[1]) {
                    println!("{}", style(config.excerpt_err).red());
                    return use_multiselect(config);
                }
            }

            Ok(selected)
        }
        None => process::exit(1),
    }
}
