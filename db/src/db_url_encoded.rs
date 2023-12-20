pub fn db_url_encoded() -> String {
    dotenvy::dotenv().ok();

    let db_host = dotenvy::var("DB_HOST").expect("DB_HOST not set");
    let db_port = dotenvy::var("DB_PORT").expect("DB_PORT not set");
    let db_name = dotenvy::var("DB_NAME").expect("DB_NAME not set");
    let db_user = dotenvy::var("DB_USER").expect("DB_USER not set");
    let db_password = dotenvy::var("DB_PASSWORD").expect("DB_PASSWORD not set");

    let db_url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name
    );

    println!("{}", db_url);
    db_url
}
