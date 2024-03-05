use std::{env, error::Error, io::Write, process::Command};

use crate::{
    config::Config,
    util::{get_exit_code, get_git_username},
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
            format!("{}/internal/env", self.path_to_project),
            format!("{}/internal/render", self.path_to_project),
            format!("{}/public", self.path_to_project),
        ];
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
        let first_letter = self.config.app_name.as_bytes()[0] as char;
        let custom_ctx_name = format!("{}ctx", first_letter);
        let mut needed = vec![
            (
                format!("{}/main.go", self.path_to_project),
                self.get_main_go_text_content(),
            ),
            (
                format!("{}/.gitignore", self.path_to_project),
                self.get_gitignore_text_content(),
            ),
            (
                format!("{}/.env", self.path_to_project),
                self.get_dot_env_text_content(),
            ),
            (
                format!("{}/Makefile", self.path_to_project),
                self.get_makefile_text_content(),
            ),
            (
                format!("{}/internal/routes/root.go", self.path_to_project),
                self.get_root_go_text_content(),
            ),
            (
                format!(
                    "{}/internal/{}/{}.go",
                    self.path_to_project, custom_ctx_name, custom_ctx_name
                ),
                self.get_custom_ctx_text_content(),
            ),
            (
                format!("{}/internal/env/env.go", self.path_to_project),
                self.get_env_text_content(),
            ),
            (
                format!("{}/internal/render/render.go", self.path_to_project),
                self.get_render_go_text_content(),
            ),
            (
                format!("{}/public/index.html", self.path_to_project),
                self.get_index_html_text_content(),
            ),
            (
                format!(
                    "{}/cmd/{}/main.go",
                    self.path_to_project, self.config.app_name
                ),
                self.get_cmd_main_go_text_content(),
            ),
        ];

        if self.config.turso {
            let db_file = (
                format!("{}/internal/db/db.go", self.path_to_project),
                self.get_db_go_text_content(),
            );
            needed.push(db_file);
        }

        if self.config.tailwind {
            let css_file = (
                format!("{}/css/index.css", self.path_to_project),
                self.get_css_text_content(),
            );
            needed.push(css_file);
        }

        for (key, val) in needed {
            self.file_to_text_map.insert(key, val);
        }
    }

    fn get_main_go_text_content(&self) -> String {
        let template = include_str!("text/main_go");
        let mut res = template.replace("##name##", &self.config.app_name);
        res = res.replace(
            "##mod_name##",
            &format!("{}/cmd/{}", self.mod_name, self.config.app_name),
        );
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
        if self.config.air {
            res += "tmp\n\n";
        }
        return res;
    }

    fn get_dot_env_text_content(&self) -> String {
        let res = include_str!("text/env").to_string();
        return res;
    }

    fn get_makefile_text_content(&self) -> String {
        let mut res = String::from(".PHONY: all\n");
        res += "all:\n";
        res += "\tgo build -o bin/main\n\n";
        if self.config.air {
            res += ".PHONY: dev\n";
            res += ".dev:\n";
            res += "\tair";
            if self.config.tailwind {
                res += " & pnpm css\n\n";
            } else {
                res += "\n\n";
            }
        }
        return res;
    }

    fn get_root_go_text_content(&self) -> String {
        let first_letter = self.config.app_name.as_bytes()[0] as char;
        let custom_ctx_name = format!("{}ctx", first_letter);
        let custom_ctx_type = format!("{}Ctx", first_letter.to_uppercase());
        let template = include_str!("text/root_go");
        let mut res = template.replace("##mod_name##", &self.mod_name);
        res = res.replace("##ctx##", &custom_ctx_name);
        res = res.replace("##Ctx##", &custom_ctx_type);
        return res;
    }

    fn get_custom_ctx_text_content(&self) -> String {
        let first_letter = self.config.app_name.as_bytes()[0] as char;
        let custom_ctx_name = format!("{}ctx", first_letter);
        let custom_ctx_type = format!("{}Ctx", first_letter.to_uppercase());
        let template = include_str!("text/custom_ctx_go");
        let mut res = template.replace("##ctx##", &custom_ctx_name);
        res = res.replace("##Ctx##", &custom_ctx_type);
        let mut imports_replacement = if self.config.sessions {
            "\"github.com/labstack/echo/v4\"\n\t\"github.com/gorilla/sessions\"".to_string()
        } else {
            "\"github.com/labstack/echo/v4\"".to_string()
        };
        if self.config.turso {
            imports_replacement += &format!("\n\t\"{}/internal/db\"", self.mod_name);
        }
        res = res.replace("##imports##", &imports_replacement);
        if self.config.sessions {
            res = res.replace("##session_store##", "Store *sessions.CookieStore")
        } else {
            res = res.replace("##session_store##", "")
        }
        if self.config.turso {
            res = res.replace("##db##", "DB *db.DB")
        } else {
            res = res.replace("##db##", "")
        }
        return res;
    }

    fn get_env_text_content(&self) -> String {
        let template = include_str!("text/env_go");
        let mut res = template.to_string();

        if self.config.sessions {
            res += r#"func GetSessionSecret() string {
    return os.Getenv("SESSION_SECRET")
}"#;
        }

        if self.config.turso {
            res += "\n\n";
            res += r#"func GetDBUrl() string {
    isProduction := os.Getenv("PRODUCTION")
    if isProduction == "true" {
        return os.Getenv("PROD_DB_URL")
    } else {
        return os.Getenv("DBURL")
    }
}"#;
        }
        return res;
    }

    fn get_render_go_text_content(&self) -> String {
        let res = include_str!("text/render_go").to_string();
        return res;
    }

    fn get_index_html_text_content(&self) -> String {
        let template = include_str!("text/index_html");
        let mut res = template.to_string();
        res = res.replace("##name##", &self.config.app_name);
        if self.config.tailwind {
            res = res.replace(
                "##css##",
                "<link rel=\"stylesheet\" href=\"/css/index.css\">",
            );
            res = res.replace(
                "##title##",
                &format!("<h1 class=\"text-xl\">{}</h1>", self.config.app_name),
            );
        } else {
            res = res.replace("##css##", "");
            res = res.replace("##title##", &format!("<h1>{}</h1>", self.config.app_name));
        }
        if self.config.htmx {
            res = res.replace(
                "##htmx##",
                "<script src=\"https://unpkg.com/htmx.org@1.9.10\"></script>",
            );
        } else {
            res = res.replace("##htmx##", "");
        }
        return res;
    }

    fn get_db_go_text_content(&self) -> String {
        let res = include_str!("text/db_go").to_string();
        return res;
    }

    fn get_css_text_content(&self) -> String {
        let res = include_str!("text/index_css").to_string();
        return res;
    }

    fn get_cmd_main_go_text_content(&self) -> String {
        let mut res: String;
        let first_letter = self.config.app_name.as_bytes()[0] as char;
        let custom_ctx_name = format!("{}ctx", first_letter);
        if self.config.turso && self.config.sessions && self.config.tailwind {
            res = include_str!("text/cmd_main_go_full").to_string();
        } else if self.config.turso && self.config.sessions {
            res = include_str!("text/cmd_main_go_turso_session").to_string();
        } else if self.config.turso && self.config.tailwind {
            res = include_str!("text/cmd_main_go_turso_tailwind").to_string();
        } else if self.config.turso {
            res = include_str!("text/cmd_main_go_turso").to_string();
        } else if self.config.sessions && self.config.tailwind {
            res = include_str!("text/cmd_main_go_session_tailwind").to_string();
        } else if self.config.sessions {
            res = include_str!("text/cmd_main_go_session").to_string();
        } else if self.config.tailwind {
            res = include_str!("text/cmd_main_go_tailwind").to_string();
        } else {
            res = include_str!("text/cmd_main_go").to_string();
        }
        res = res.replace("##mod_name##", &self.mod_name);
        res = res.replace("##ctx##", &custom_ctx_name);
        return res;
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

    pub fn build(&self) -> Result<(), AppBuilderError> {
        self.create_dirs()?;
        self.create_files()?;
        self.run_go_mod_init()?;
        self.run_pnpm_init()?;
        self.install_tailwind()?;
        self.initialize_tailwind()?;
        self.initialize_air()?;
        self.run_go_mod_tidy()?;
        self.run_go_fmt()?;
        println!("done");
        Ok(())
    }

    fn run_go_mod_init(&self) -> Result<(), AppBuilderError> {
        println!("running go mod init");
        let mut cmd = Command::new("go");
        cmd.arg("mod")
            .arg("init")
            .arg(&self.config.mod_name)
            .current_dir(&self.config.path_to_project);
        let output = cmd.output()?;
        let exit_code = get_exit_code(Ok(output.status));
        if exit_code != 0 {
            return Err("failed to run go mod init".into());
        }
        Ok(())
    }

    fn run_pnpm_init(&self) -> Result<(), AppBuilderError> {
        if !self.config.config.tailwind {
            return Ok(());
        }
        println!("running pnpm init");
        let mut cmd = Command::new("pnpm");
        cmd.arg("init").current_dir(&self.config.path_to_project);
        let output = cmd.output()?;
        let exit_code = get_exit_code(Ok(output.status));
        if exit_code != 0 {
            return Err("failed to run pnpm init".into());
        }
        Ok(())
    }

    fn install_tailwind(&self) -> Result<(), AppBuilderError> {
        if !self.config.config.tailwind {
            return Ok(());
        }
        println!("installing tailwind");
        let mut cmd = Command::new("pnpm");
        cmd.arg("add")
            .arg("-D")
            .arg("tailwindcss")
            .current_dir(&self.config.path_to_project);
        let output = cmd.output()?;
        let exit_code = get_exit_code(Ok(output.status));
        if exit_code != 0 {
            return Err("failed to install tailwind".into());
        }
        Ok(())
    }

    fn initialize_tailwind(&self) -> Result<(), AppBuilderError> {
        if !self.config.config.tailwind {
            return Ok(());
        }
        println!("initializing tailwind");
        let mut cmd = Command::new("npx");
        cmd.arg("tailwindcss")
            .arg("init")
            .current_dir(&self.config.path_to_project);
        let output = cmd.output()?;
        let exit_code = get_exit_code(Ok(output.status));
        if exit_code != 0 {
            return Err("failed to initailize tailwind".into());
        }
        Ok(())
    }

    fn initialize_air(&self) -> Result<(), AppBuilderError> {
        if !self.config.config.air {
            return Ok(());
        }
        println!("initializing air");
        let mut cmd = Command::new("air");
        cmd.arg("init").current_dir(&self.config.path_to_project);
        let output = cmd.output()?;
        let exit_code = get_exit_code(Ok(output.status));
        if exit_code != 0 {
            return Err("failed to initialize air".into());
        }
        Ok(())
    }

    fn run_go_mod_tidy(&self) -> Result<(), AppBuilderError> {
        println!("running go mod tidy");
        let mut cmd = Command::new("go");
        cmd.arg("mod")
            .arg("tidy")
            .current_dir(&self.config.path_to_project);
        let output = cmd.output()?;
        let exit_code = get_exit_code(Ok(output.status));
        if exit_code != 0 {
            return Err("failed to run go mod tidy".into());
        }
        Ok(())
    }

    fn run_go_fmt(&self) -> Result<(), AppBuilderError> {
        println!("running go fmt");
        let mut cmd = Command::new("go");
        cmd.arg("fmt")
            .arg("./...")
            .current_dir(&self.config.path_to_project);
        let output = cmd.output()?;
        let exit_code = get_exit_code(Ok(output.status));
        if exit_code != 0 {
            return Err("failed to run go fmt".into());
        }
        Ok(())
    }

    fn create_dirs(&self) -> Result<(), AppBuilderError> {
        for dir in &self.config.dirs_to_create {
            std::fs::create_dir(dir)?;
        }
        Ok(())
    }

    fn create_files(&self) -> Result<(), AppBuilderError> {
        for (file, content) in &self.config.file_to_text_map {
            let mut file = std::fs::File::create(file)?;
            file.write(content.as_bytes())?;
        }
        Ok(())
    }
}
