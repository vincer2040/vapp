
#[derive(Debug)]
pub struct Config {
    pub app_name: String,
    pub sessions: bool,
    pub turso: bool,
    pub htmx: bool,
    pub tailwind: bool,
    pub air: bool,
}

pub struct ConfigBuilder {
    app_name: Option<String>,
    sessions: Option<bool>,
    turso: Option<bool>,
    htmx: Option<bool>,
    tailwind: Option<bool>,
    air: Option<bool>,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        ConfigBuilder {
            app_name: None,
            sessions: None,
            turso: None,
            htmx: None,
            tailwind: None,
            air: None,
        }
    }

    pub fn add_app_name(mut self, app_name: String) -> Self {
        self.app_name = Some(app_name);
        self
    }

    pub fn add_sessions(mut self, value: bool) -> Self {
        self.sessions = Some(value);
        self
    }

    pub fn add_turso(mut self, value: bool) -> Self {
        self.turso = Some(value);
        self
    }

    pub fn add_htmx(mut self, value: bool) -> Self {
        self.htmx = Some(value);
        self
    }

    pub fn add_tailwind(mut self, value: bool) -> Self {
        self.tailwind = Some(value);
        self
    }

    pub fn add_air(mut self, value: bool) -> Self {
        self.air = Some(value);
        self
    }

    pub fn out(self) -> Config {
        Config {
            app_name: match self.app_name {
                Some(name) => name,
                None => "".to_string(),
            },
            sessions: match self.sessions {
                Some(value) => value,
                None => false,
            },
            turso: match self.turso {
                Some(value) => value,
                None => false,
            },
            htmx: match self.htmx {
                Some(value) => value,
                None => false,
            },
            tailwind: match self.tailwind {
                Some(value) => value,
                None => false,
            },
            air: match self.air {
                Some(value) => value,
                None => false,
            },
        }
    }
}
