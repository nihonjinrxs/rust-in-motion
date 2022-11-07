pub fn save_status(text: &str) -> Result<i64, &'static str> {
    if text.len() > 200 {
        println!("{} of 200 bytes...", text.len());
        return Err("status text is too long");
    }

    // Super common, so we can replace the entire match with ? operator
    // let record = match save_to_database(text) {
    //     Ok(status_record) => status_record,
    //     Err(status_err) => Err(status_err),
    // };
    let record = save_to_database(text)?;

    Ok(record.id)
}

fn save_to_database(text: &str) -> Result<StatusRecord, &'static str> {
    // fake implementation that always fails
    Err("database unavailable")
}

struct StatusRecord {
    id: i64,
    text: String,
    created_at: std::time::Instant,
}

