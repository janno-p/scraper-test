use std::time::Duration;

use colored::Colorize;
use scraper::{Html, Selector};
use thirtyfour::prelude::*;

struct Session {
    driver: WebDriver,
}

impl Session {
    async fn init() -> WebDriverResult<Self> {
        let caps = DesiredCapabilities::firefox();
        // caps.set_headless()?;

        let driver = WebDriver::new("http://localhost:4444", caps).await?;

        driver.goto("https://www.tournamentsoftware.com/").await?;

        let accept_cookies_button = driver.find(By::Css("button.js-accept-basic")).await?;
        accept_cookies_button.click().await?;
        accept_cookies_button.wait_until().stale().await?;

        Ok(Self { driver })
    }

    async fn load_content(&self, url: String) -> WebDriverResult<String> {
        println!("goto: {}", url.red());
        self.driver.goto(url).await?;

        println!("find-content");
        let content = self.driver.find(By::Css("#content")).await?;

        println!("content-displayed");
        content.wait_until().displayed().await?;

        println!("outer-html");
        let html = content.outer_html().await?;

        Ok(html)
    }

    async fn close(self) -> WebDriverResult<()> {
        self.driver.quit().await
    }
}

struct EventUrl {
    name: String,
    url: Option<String>,
}

async fn parse_events(session: &Session) -> WebDriverResult<Vec<EventUrl>> {
    let html = session.load_content("https://www.tournamentsoftware.com/sport/events.aspx?id=8e03f37d-88d4-4ccd-9b42-95bd2abf6a3b".into()).await?;
    let fragment = Html::parse_fragment(&html);

    let selector = Selector::parse("table.admintournamentevents tbody tr td a").unwrap();

    let event_urls = fragment.select(&selector)
        .map(|el| {
            EventUrl {
                name: el.inner_html(),
                url: el.attr("href").map(|v| v.into()),
            }
        })
        .collect::<Vec<_>>();

    Ok(event_urls)
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let session = Session::init().await?;

    let events = parse_events(&session).await?;

    for evt in events {
        if let Some(url) = evt.url {
            // tokio::time::sleep(Duration::from_secs(2)).await;
            let html = session.load_content(url).await?;
            println!("{} {} {}", "=====".cyan(), evt.name.green(), "=====".cyan());
            println!("{}", html);
        }
    }

    session.close().await?;

    Ok(())
}
