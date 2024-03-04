use std::{env, error::Error, fs, process::Command};

use crate::{
    config::Config,
    util::{get_exit_code, get_git_username},
};

#[derive(Debug)]
pub struct AppBuilder {
    mod_name: String,
    cur_path: String,
    config: Config,
}

type AppBuilderError = Box<dyn Error>;

impl AppBuilder {
    pub fn new(config: Config) -> Result<Self, AppBuilderError> {
        let git_user_name = get_git_username();
        let cur_path_buf = env::current_dir()?;
        let cur_path = match cur_path_buf.to_str() {
            Some(s) => s.to_string(),
            None => return Err("cur path is not a valid string".into()),
        };
        let mod_name = match &git_user_name {
            Some(name) => format!("github.com/{}/{}", name, config.app_name),
            None => config.app_name.clone(),
        };
        Ok(Self {
            mod_name,
            cur_path,
            config,
        })
    }

    pub fn build(&self) -> Result<(), AppBuilderError> {
        self.make_needed_directories()?;
        self.make_needed_files()?;
        self.run_go_mod_init()?;
        self.run_pnpm_init()?;
        self.install_tailwind()?;
        self.run_tailwindcss_init()?;
        Ok(())
    }

    fn run_go_mod_init(&self) -> Result<(), AppBuilderError> {
        let cur_dir = self.cur_path.clone() + "/" + &self.config.app_name;
        println!("running go mod init");
        let mut cmd = Command::new("go");
        cmd.arg("mod")
            .arg("init")
            .arg(&self.mod_name)
            .current_dir(cur_dir);
        let output = cmd.output()?;
        let exit_code = get_exit_code(Ok(output.status));
        if exit_code != 0 {
            return Err("failed to run go mod init".into());
        }
        Ok(())
    }

    fn run_pnpm_init(&self) -> Result<(), AppBuilderError> {
        if !self.config.tailwind {
            return Ok(());
        }
        println!("running pnpm init");
        let cur_dir = self.cur_path.clone() + "/" + &self.config.app_name;
        let mut cmd = Command::new("pnpm");
        cmd.arg("init").current_dir(cur_dir);
        let output = cmd.output()?;
        let exit_code = get_exit_code(Ok(output.status));
        if exit_code != 0 {
            return Err("failed to run go mod init".into());
        }
        Ok(())
    }

    fn install_tailwind(&self) -> Result<(), AppBuilderError> {
        if !self.config.tailwind {
            return Ok(());
        }
        println!("installing tailwind");
        let cur_dir = self.cur_path.clone() + "/" + &self.config.app_name;
        let mut cmd = Command::new("pnpm");
        cmd.arg("add")
            .arg("-D")
            .arg("tailwindcss")
            .current_dir(cur_dir);
        let output = cmd.output()?;
        let exit_code = get_exit_code(Ok(output.status));
        if exit_code != 0 {
            return Err("failed to run go mod init".into());
        }
        Ok(())
    }

    fn run_tailwindcss_init(&self) -> Result<(), AppBuilderError> {
        if !self.config.tailwind {
            return Ok(());
        }
        println!("initializing tailwind");
        let cur_dir = self.cur_path.clone() + "/" + &self.config.app_name;
        let mut cmd = Command::new("npx");
        cmd.arg("tailwindcss").arg("init").current_dir(cur_dir);
        let output = cmd.output()?;
        let exit_code = get_exit_code(Ok(output.status));
        if exit_code != 0 {
            return Err("failed to run go mod init".into());
        }
        Ok(())
    }

    fn make_needed_directories(&self) -> Result<(), AppBuilderError> {
        let dirs = self.get_needed_dirs();
        for dir in dirs {
            fs::create_dir(dir)?;
        }
        Ok(())
    }

    fn make_needed_files(&self) -> Result<(), AppBuilderError> {
        let files = self.get_needed_files();
        for file in files {
            let _ = fs::File::create(file)?;
        }
        Ok(())
    }

    fn get_needed_dirs(&self) -> Vec<String> {
        let first_letter = self.config.app_name.as_bytes()[0] as char;
        let custom_ctx_name = format!("{}ctx", first_letter);
        let mut needed = vec![
            format!("{}/{}", self.cur_path, self.config.app_name),
            format!("{}/{}/cmd", self.cur_path, self.config.app_name),
            format!(
                "{}/{}/cmd/{}",
                self.cur_path, self.config.app_name, self.config.app_name
            ),
            format!("{}/{}/internal", self.cur_path, self.config.app_name),
            format!("{}/{}/internal/routes", self.cur_path, self.config.app_name),
            format!(
                "{}/{}/internal/{}",
                self.cur_path, self.config.app_name, custom_ctx_name
            ),
            format!("{}/{}/public", self.cur_path, self.config.app_name),
        ];
        if self.config.sessions {
            let env_dir = format!("{}/{}/internal/env", self.cur_path, self.config.app_name);
            needed.push(env_dir);
        }
        if self.config.turso {
            let db_dir = format!("{}/{}/testdb", self.cur_path, self.config.app_name);
            needed.push(db_dir);
        }
        if self.config.tailwind {
            let css_dir = format!("{}/{}/css", self.cur_path, self.config.app_name);
            needed.push(css_dir);
        }
        needed
    }

    fn get_needed_files(&self) -> Vec<String> {
        let first_letter = self.config.app_name.as_bytes()[0] as char;
        let custom_ctx_name = format!("{}ctx", first_letter);
        let mut needed = vec![
            format!("{}/{}/main.go", self.cur_path, self.config.app_name),
            format!("{}/{}/Makefile", self.cur_path, self.config.app_name),
            format!("{}/{}/.env", self.cur_path, self.config.app_name),
            format!("{}/{}/.gitignore", self.cur_path, self.config.app_name),
            format!(
                "{}/{}/cmd/{}/main.go",
                self.cur_path, self.config.app_name, self.config.app_name
            ),
            format!(
                "{}/{}/internal/routes/root.go",
                self.cur_path, self.config.app_name
            ),
            format!(
                "{}/{}/internal/{}/{}.go",
                self.cur_path, self.config.app_name, custom_ctx_name, custom_ctx_name
            ),
            format!(
                "{}/{}/public/index.html",
                self.cur_path, self.config.app_name
            ),
        ];
        if self.config.sessions {
            let env_lib_file = format!(
                "{}/{}/internal/env/env.go",
                self.cur_path, self.config.app_name
            );
            needed.push(env_lib_file);
        }
        if self.config.turso {
            let db_file = format!(
                "{}/{}/testdb/testdb.db",
                self.cur_path, self.config.app_name
            );
            needed.push(db_file);
        }
        if self.config.tailwind {
            let css_file = format!("{}/{}/css/index.css", self.cur_path, self.config.app_name);
            needed.push(css_file);
        }
        return needed;
    }
}
