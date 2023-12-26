use scraper::{Html, Selector};
use thirtyfour::prelude::*;

use super::session::Session;

pub struct EventUrl {
    pub name: String,
    pub url: Option<String>,
}

pub async fn load_events(session: &Session, event_id: &str) -> WebDriverResult<Vec<EventUrl>> {
    let url = format!(
        "https://www.tournamentsoftware.com/sport/events.aspx?id={}",
        event_id.to_lowercase()
    );

    let html = session.load_content(url).await?;
    let fragment = Html::parse_fragment(&html);

    let selector = Selector::parse("table.admintournamentevents tbody tr td a").unwrap();

    let event_urls = fragment
        .select(&selector)
        .map(|el| EventUrl {
            name: el.inner_html(),
            url: el.attr("href").map(|v| v.into()),
        })
        .collect::<Vec<_>>();

    Ok(event_urls)
}
