use mysql::{params, prelude::Queryable};

use crate::{
    corona_csv_row::Row,
    db::{get_conn, INSERT_QUERY},
};
mod corona_csv_row;
mod db;

pub async fn run(db_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let csv_url =
        "https://stopcovid19.metro.tokyo.lg.jp/data/130001_tokyo_covid19_patients_2021.csv";

    let csv_str = reqwest::get(csv_url).await?.text().await?;

    let mut reader = csv::Reader::from_reader(csv_str.as_bytes());

    let headers = match reader.headers() {
        Ok(h) => h,
        Err(e) => return Err(Box::new(e)),
    };
    println!("{:?}", headers);

    let list = reader
        .deserialize()
        .filter_map(|r| r.ok())
        .take(10)
        .collect::<Vec<Row>>();

    println!("{:?}", list);

    let mut conn = get_conn(db_url)?;

    // キーが重複している場合は、エラーになる
    conn.exec_batch(
        INSERT_QUERY,
        list.iter().map(|p| {
            params! {
                "no" => &p.no,
                "kohyo_date" => &p.kohyo_date,
                "hasyo_date" => &p.hasyo_date,
                "kakutei_date" => &p.kakutei_date,
                "code" => &p.code,
                "prefecture" => &p.prefecture,
                "city" => &p.city,
                "age" => &p.age,
                "gender" => &p.gender,
                "status" => &p.status,
                "symptoms" => &p.symptoms,
                "occupation" => &p.occupation,
                "residence" => &p.residence,
                "contact_history_flag" => &p.contact_history_flag,
                "travel_history_flag" => &p.travel_history_flag,
                "discharge_flag" => &p.discharge_flag,
                "note" => &p.note
            }
        }),
    )?;

    Ok(())
}
