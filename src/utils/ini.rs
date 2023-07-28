use ini::Ini;

pub fn database_config() -> Vec<String> {
    let config = Ini::load_from_file("RANAuth.ini").unwrap();
    let section = config.section(Some("Database")).unwrap();
    let db_user = section.get("DBUser").unwrap();
    let db_pass = section.get("DBPass").unwrap();
    let db_host = section.get("DBHost").unwrap();
    let db_port = section.get("DBPort").unwrap();
    let db_name = section.get("DBName").unwrap();

    let strings = vec![db_user.to_string(), db_pass.to_string(), db_host.to_string(), db_port.to_string(), db_name.to_string()];

    strings
}