pub fn tournament_url(tournament_id: &str) -> String {
    format!("/tournament/{}", tournament_id.to_uppercase())
}

pub fn events_url(tournament_id: &str) -> String {
    format!("/sport/events.aspx?id={}", tournament_id.to_lowercase())
}

pub fn event_draws_url(tournament_id: &str) -> String {
    format!("/sport/draws.aspx?id={}", tournament_id.to_lowercase())
}
