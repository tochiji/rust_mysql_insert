use mysql::PooledConn;

pub fn get_conn(url: &str) -> Result<PooledConn, mysql::Error> {
    let builder = mysql::OptsBuilder::from_opts(mysql::Opts::from_url(url)?);
    let pool = mysql::Pool::new(builder.ssl_opts(mysql::SslOpts::default()))?;
    let conn = pool.get_conn()?;

    Ok(conn)
}

pub static INSERT_QUERY: &str = r"
    INSERT INTO test_tokyo_corona (
        `no`,
        `kohyo_date`,
        `hasyo_date`,
        `kakutei_date`,
        `code`,
        `prefecture`,
        `city`,
        `age`,
        `gender`,
        `status`,
        `symptoms`,
        `occupation`,
        `residence`,
        `contact_history_flag`,
        `travel_history_flag`,
        `discharge_flag`,
        `note`
    ) VALUES (
        :no,
        :kohyo_date,
        :hasyo_date,
        :kakutei_date,
        :code,
        :prefecture,
        :city,
        :age,
        :gender,
        :status,
        :symptoms,
        :occupation,
        :residence,
        :contact_history_flag,
        :travel_history_flag,
        :discharge_flag,
        :note
    )";
