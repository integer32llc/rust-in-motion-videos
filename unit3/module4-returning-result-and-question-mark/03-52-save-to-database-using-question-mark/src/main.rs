fn save_status(text: &str) -> Result<i64, &'static str> {
    if text.len() > 200 {
        return Err("status text is too long");
    }

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

fn main() {
    println!("Hello, world!");
}
