use ini::Ini;
use std::sync::Arc;

pub struct ConfigState {
    pub config: Arc<Config>,
}

impl ConfigState {
    pub fn new(config: Arc<Config>) -> Self {
        ConfigState { config }
    }
}

pub struct Config {
    pub db_user: String,
    pub db_pass: String,
    pub db_host: String,
    pub db_port: String,
    pub db_web: String,
    pub ran_game: String,
    pub ran_log: String,
    pub ran_shop: String,
    pub ran_user: String,
    pub api_host: String,
    pub api_port: i32,
}

impl Config {
    pub fn load() -> Self {
        let config = Ini::load_from_file("RANApi.ini").unwrap();

        let db_section = config.section(Some("Database")).unwrap();
        let app_section = config.section(Some("Application")).unwrap();

        //? Info : This the side for Application Settings
        let api_host = app_section.get("APIHost").unwrap().to_string();
        let api_port = app_section.get("APIPort").unwrap().parse::<i32>().unwrap();
        let game_server_count = app_section
            .get("GServerCount")
            .unwrap()
            .parse::<i32>()
            .unwrap();

        //? Info : Database Settings
        let db_user = db_section.get("DBUser").unwrap().to_string();
        let db_pass = db_section.get("DBPass").unwrap().to_string();
        let db_host = db_section.get("DBHost").unwrap().to_string();
        let db_port = db_section.get("DBPort").unwrap().to_string();
        let db_web = db_section.get("DBRanWeb").unwrap().to_string();

        // TODO: Add server count
        let ran_game = db_section.get("DBRanGame").unwrap().to_string();
        let ran_log = db_section.get("DBRanLog").unwrap().to_string();
        // TODO: End

        let ran_shop = db_section.get("DBRanShop").unwrap().to_string();
        let ran_user = db_section.get("DBRanUser").unwrap().to_string();

        Config {
            db_user,
            db_pass,
            db_host,
            db_port,
            db_web,
            ran_game,
            ran_log,
            ran_shop,
            ran_user,
            api_host,
            api_port,
        }
    }

    pub fn get_app_server(&self) -> String {
        format!("{}:{}", self.api_host, self.api_port)
    }

    pub fn get_database_url(&self) -> String {
        format!(
            "sqlserver://{}:{}@{}:{}",
            self.db_user, self.db_pass, self.db_host, self.db_port
        )
    }

    pub fn get_ran_user_db(&self) -> String {
        format!("{}", self.ran_user)
    }
}
