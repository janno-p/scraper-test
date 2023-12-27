use sqlx::{Pool, Sqlite};

use super::client::Client;

pub struct Browser {
    #[allow(dead_code)]
    client: Client,
    #[allow(dead_code)]
    database: Pool<Sqlite>,
}

impl Browser {
    pub async fn init(database: Pool<Sqlite>) -> anyhow::Result<Self> {
        let client = Client::init().await?;
        let browser = Self { client, database };
        Ok(browser)
    }
}
