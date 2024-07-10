pub fn parse_date(date: &str) -> chrono::NaiveDate {
    match date {
        "today" => chrono::Utc::now().date_naive(),
        "yesterday" => chrono::Utc::now().date_naive() - chrono::Duration::days(1),
        "start" => chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(), // Unix epoch
        _ => chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap(),
    }
}
