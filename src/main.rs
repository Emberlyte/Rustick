use std::time::Duration;
use rusqlite::Connection;
use teloxide::{prelude::*, types::Recipient};

fn init_db() -> Connection {
    let conn = Connection::open("counter.db").unwrap();
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS counter (id INTEGER PRIMARY KEY CHECK (id = 1), value INTEGER NOT NULL);
         INSERT OR IGNORE INTO counter (id, value) VALUES (1, 0);",
    )
    .unwrap();
    conn
}

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();

    let raw = std::env::var("CHANNEL_ID").expect("CHANNEL_ID is not set");
    let channel = match raw.parse::<i64>() {
        Ok(id) => Recipient::Id(ChatId(id)),
        Err(_) => Recipient::ChannelUsername(raw)
    };

    let conn = init_db();

    let mut interval = tokio::time::interval(Duration::from_secs(600));

    loop {
        interval.tick().await;

        let value: i64 = conn
            .query_row( "UPDATE counter SET value = value + 1 WHERE id = 1 RETURNING value",
                        [],
                        |r| r.get(0),
            ).unwrap();

        if let Err(e) = bot.send_message(channel.clone(), value.to_string()).await {
            eprintln!("failed to send {value}: {e}");
        }
    }
}
