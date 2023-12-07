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

async fn load_events(session: &Session, event_id: &str) -> WebDriverResult<Vec<EventUrl>> {
    let url = format!("https://www.tournamentsoftware.com/sport/events.aspx?id={}", event_id.to_lowercase());

    let html = session.load_content(url).await?;
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

#[derive(Debug)]
enum DrawType {
    RoundRobin(u32),
    Elimination(u32),
    QualifyingDraw(u32, u32),
}

#[derive(Debug)]
enum DrawStage {
    Qualifying,
    MainDraw,
    Playoff,
}

struct DrawInfo {
    name: String,
    url: Option<String>,
    r#type: DrawType,
    stage: DrawStage,
    consolation: Option<String>,
}

fn parse_draw_type(type_str: &str, size_str: &str) -> Option<DrawType> {
    match type_str {
        "Round Robin" => Some(DrawType::RoundRobin(size_str.parse::<u32>().unwrap())),
        "Elimination" => Some(DrawType::Elimination(size_str.parse::<u32>().unwrap())),
        "Qualifying Draw" => {
            println!("{}", size_str);
            let mut ratio = size_str.split('>');
            let initial_size = ratio.next().map(|x| x.trim().parse::<u32>().unwrap()).unwrap();
            let target_size = ratio.next().map(|x| x.trim().parse::<u32>().unwrap()).unwrap();
            Some(DrawType::QualifyingDraw(initial_size, target_size))
        },
        _ => None,
    }
}

fn parse_draw_stage(stage_str: &str) -> Option<DrawStage> {
    match stage_str {
        "Main Draw" => Some(DrawStage::MainDraw),
        "Qualifying" => Some(DrawStage::Qualifying),
        "Playoff" => Some(DrawStage::Playoff),
        _ => None,
    }
}

async fn load_draws(session: &Session, event_id: &str) -> WebDriverResult<Vec<DrawInfo>> {
    let url = format!("https://www.tournamentsoftware.com/sport/draws.aspx?id={}", event_id.to_lowercase());

    let html = session.load_content(url).await?;
    let fragment = Html::parse_fragment(&html);

    let draw_selector = Selector::parse("table tbody tr").unwrap();

    let draws = fragment.select(&draw_selector)
        .map(|row_el| {
            let cell_els: Vec<_> = row_el.select(&Selector::parse("td").unwrap()).collect();
            let name_el = cell_els[0].select(&Selector::parse("a").unwrap()).next().unwrap();
            let size_str = cell_els[1].text().next().unwrap();
            let type_str = cell_els[2].text().next().unwrap();
            let stage_str = cell_els[3].text().next().unwrap();
            let consolation =
                match cell_els[4].inner_html().as_str() {
                    "" => None,
                    val => Some(val.into()),
                };
            DrawInfo {
                name: name_el.text().next().unwrap().into(),
                url: name_el.attr("href").map(|v| v.into()),
                r#type: parse_draw_type(type_str, size_str).unwrap(),
                stage: parse_draw_stage(stage_str).unwrap(),
                consolation,
            }
        })
        .collect::<Vec<_>>();

    Ok(draws)
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let event_id = "8E03F37D-88D4-4CCD-9B42-95BD2ABF6A3B";

    let session = Session::init().await?;

    let events = load_events(&session, event_id).await?;
    let draws = load_draws(&session, event_id).await?;

    for evt in events {
        if let Some(url) = evt.url {
            // tokio::time::sleep(Duration::from_secs(2)).await;
            let html = session.load_content(url).await?;
            println!("{} {} {}", "=====".cyan(), evt.name.green(), "=====".cyan());
            println!("{}", html);
        }
    }

    for draw in draws {
        println!("{}", format!("===== {} =====", draw.name).green());
        println!("{}: {}", "Url".yellow(), draw.url.unwrap().cyan().bold());
        println!("{}: {}", "Type".yellow(), format!("{:?}", draw.r#type).cyan().bold());
        println!("{}: {}", "Stage".yellow(), format!("{:?}", draw.stage).cyan().bold());
        println!("{}: {}", "Consolation".yellow(), format!("{:?}", draw.consolation).cyan().bold());
    }

    session.close().await?;

    Ok(())
}
