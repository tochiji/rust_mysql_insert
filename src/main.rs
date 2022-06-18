use std::process;

use mysql_test::run;

#[tokio::main]
async fn main() {
    let key = "DB_URL";
    let db_url = dotenv::var(key).unwrap();

    match run(&db_url).await {
        Ok(()) => process::exit(0),
        Err(e) => {
            eprintln!("{:?}", e);
            process::exit(1);
        }
    }
}
