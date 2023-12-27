pub struct Tournament {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    occurred_at: i64,
    #[allow(dead_code)]
    updated_at: i64,
}

pub fn parse_tournament(_html: &str) -> anyhow::Result<Tournament> {
    Ok(Tournament {
        name: "Test".into(),
        occurred_at: -1,
        updated_at: -1,
    })
}
