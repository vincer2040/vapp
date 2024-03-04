use std::{env, error::Error};

use crate::{
    config::Config,
    util::get_git_username,
};

type AppBuilderError = Box<dyn Error>;

#[derive(Debug)]
struct AppBuilderConfig {
    mod_name: String,
    path_to_project: String,
    config: Config,
    dirs_to_create: Vec<String>,
    file_to_text_map: std::collections::HashMap<String, String>,
}

impl AppBuilderConfig {
    pub fn new(config: Config) -> Result<Self, AppBuilderError> {
        let cur_path_buf = env::current_dir()?;
        let cur_path = match cur_path_buf.to_str() {
            Some(s) => s.to_string(),
            None => return Err("cur path is not a valid string".into()),
        };
        let path_to_project = cur_path + "/" + &config.app_name;
        let git_user_name = get_git_username();
        let mod_name = match &git_user_name {
            Some(name) => format!("github.com/{}/{}", name, config.app_name),
            None => config.app_name.clone(),
        };
        let mut res = Self {
            mod_name,
            path_to_project,
            config,
            dirs_to_create: Vec::new(),
            file_to_text_map: std::collections::HashMap::new(),
        };
        res.add_dirs_to_create();
        res.init_file_to_text_map();
        Ok(res)
    }

    fn add_dirs_to_create(&mut self) {
        let first_letter = self.config.app_name.as_bytes()[0] as char;
        let custom_ctx_name = format!("{}ctx", first_letter);
        let mut needed = vec![
            self.path_to_project.clone(),
            format!("{}/cmd", self.path_to_project),
            format!("{}/cmd/{}", self.path_to_project, self.config.app_name),
            format!("{}/internal", self.path_to_project),
            format!("{}/internal/routes", self.path_to_project),
            format!("{}/internal/{}", self.path_to_project, custom_ctx_name),
            format!("{}/public", self.path_to_project),
        ];
        if self.config.sessions {
            let env_dir = format!("{}/internal/env", self.path_to_project);
            needed.push(env_dir);
        }
        if self.config.turso {
            let db_dir = format!("{}/testdb", self.path_to_project);
            let db_lib_dir = format!("{}/internal/db", self.path_to_project);
            needed.push(db_dir);
            needed.push(db_lib_dir);
        }
        if self.config.tailwind {
            let css_dir = format!("{}/css", self.path_to_project);
            needed.push(css_dir);
        }
        self.dirs_to_create = needed;
    }

    fn init_file_to_text_map(&mut self) {
        let mut needed = vec![
            (format!("{}/main.go", self.path_to_project), self.get_main_go_text_content()),
            (format!("{}/.gitignore", self.path_to_project), self.get_gitignore_text_content()),
            (format!("{}/Makefile", self.path_to_project), self.get_makefile_text_content()),
        ];
        for (key, val) in needed {
            self.file_to_text_map.insert(key, val);
        }
    }

    fn get_main_go_text_content(&self) -> String {
        let template = include_str!("text/main_go");
        let mut res = template.replace("##name##", &self.config.app_name);
        res = res.replace("##mod_name##", &self.mod_name);
        return res;
    }

    fn get_gitignore_text_content(&self) -> String {
        let mut res = String::from("bin\n\n.env\n\n");
        if self.config.turso {
            res += "testdb\n\n";
        }
        if self.config.tailwind {
            res += "public/css\n\n";
        }
        return res;
    }

    fn get_makefile_text_content(&self) -> String {
        let mut res = String::from(".PHONY: all\n");
        res += "all:\n";
        res += "\tgo build -o bin/main\n";
        return res
    }
}

#[derive(Debug)]
pub struct AppBuilder {
    config: AppBuilderConfig,
}

impl AppBuilder {
    pub fn new(config: Config) -> Result<Self, AppBuilderError> {
        let conf = AppBuilderConfig::new(config)?;
        return Ok(Self { config: conf });
    }
}
