// This function parses either common date strings or a date in the format YYYY-MM-DD
pub fn parse_date(date: &str) -> chrono::NaiveDate {
    match date {
        "today" => chrono::Utc::now().date_naive(),
        "yesterday" => chrono::Utc::now().date_naive() - chrono::Duration::days(1),
        "start" => chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(), // Unix epoch
        _ => chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap(),
    }
}

// This function parses a string of the form key=value into a tuple of (key, value)
pub fn parse_option(option: &str) -> (String, String) {
    let mut split = option.splitn(2, '=');
    let key = split.next().unwrap().to_string();
    let value = split.next().unwrap().to_string();
    (key, value)
}
