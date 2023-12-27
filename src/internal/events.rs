use scraper::{Html, Selector};

pub struct EventUrl {
    pub name: String,
    pub url: Option<String>,
}

pub async fn parse_events(html: &str) -> anyhow::Result<Vec<EventUrl>> {
    let fragment = Html::parse_fragment(html);

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
