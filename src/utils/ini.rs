use ini::Ini;

pub struct Config {
    pub db_user: String,
    pub db_pass: String,
    pub db_host: String,
    pub db_port: String,
    pub db_name: String,
    pub app_host: String,
    pub app_port: i32,
}

impl Config {
    pub fn load() -> Self {
        let config = Ini::load_from_file("RANAuth.ini").unwrap();

        let db_section = config.section(Some("Database")).unwrap();
        let app_section = config.section(Some("Application")).unwrap();

        let db_user = db_section.get("DBUser").unwrap().to_string();
        let db_pass = db_section.get("DBPass").unwrap().to_string();
        let db_host = db_section.get("DBHost").unwrap().to_string();
        let db_port = db_section.get("DBPort").unwrap().to_string();
        let db_name = db_section.get("DBName").unwrap().to_string();

        let app_host = app_section.get("AuthHost").unwrap().to_string();
        let app_port = app_section.get("AuthPort").unwrap().parse::<i32>().unwrap();

        Config {
            db_user,
            db_pass,
            db_host,
            db_port,
            db_name,
            app_host,
            app_port,
        }
    }

    pub fn app_server(&self) -> String {
        format!("{}:{}", self.app_host, self.app_port)
    }

    pub fn database_url(&self) -> String {
        format!(
            "sqlserver://{}:{}@{}:{}/{}",
            self.db_user, self.db_pass, self.db_host, self.db_port, self.db_name
        )
    }
}