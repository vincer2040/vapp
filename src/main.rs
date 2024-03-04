use std::process::exit;

use config::{Config, ConfigBuilder};
use util::{read_line, yn_to_bool};

use crate::app_builder::AppBuilder;

mod app_builder;
mod config;
mod util;

fn main() {
    let config = match build_config() {
        Ok(c) => c,
        Err(e) => {
            println!("{:#?}", e);
            exit(1);
        }
    };
    println!("{:#?}", config);
    let app_builder = match AppBuilder::new(config) {
        Ok(ab) => ab,
        Err(e) => {
            println!("{:#?}", e);
            exit(1);
        }
    };
    match app_builder.build() {
        Ok(_) => (),
        Err(e) => {
            println!("{:#?}", e);
            exit(1);
        }
    };
}

fn build_config() -> std::io::Result<Config> {
    let app_name: String;
    loop {
        let app_name_input = read_line(Some("enter the app name: "))?;
        if is_valid_app_name(&app_name_input) {
            app_name = app_name_input;
            break;
        }
        continue;
    }
    let sessions: bool;
    loop {
        let sessions_input = read_line(Some("would you like to use gorilla sessions? [y/n]: "))?;
        let yn = yn_to_bool(&sessions_input);
        match yn {
            Some(val) => {
                sessions = val;
                break;
            }
            None => continue,
        }
    }
    let turso: bool;
    loop {
        let turso_input = read_line(Some("would you like use turso? [y/n]: "))?;
        let yn = yn_to_bool(&turso_input);
        match yn {
            Some(val) => {
                turso = val;
                break;
            }
            None => continue,
        }
    }
    let htmx: bool;
    loop {
        let htmx_input = read_line(Some("would you like use htmx? [y/n]: "))?;
        let yn = yn_to_bool(&htmx_input);
        match yn {
            Some(val) => {
                htmx = val;
                break;
            }
            None => continue,
        }
    }
    let tailwind: bool;
    loop {
        let tailwind_input = read_line(Some("would you like use tailwind? [y/n]: "))?;
        let yn = yn_to_bool(&tailwind_input);
        match yn {
            Some(val) => {
                tailwind = val;
                break;
            }
            None => continue,
        }
    }

    let config = ConfigBuilder::new()
        .add_app_name(app_name)
        .add_sessions(sessions)
        .add_turso(turso)
        .add_htmx(htmx)
        .add_tailwind(tailwind)
        .out();
    return Ok(config);
}

fn is_valid_app_name(input: &str) -> bool {
    match input {
        "" => false,
        _ => true,
    }
}
