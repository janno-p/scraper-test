use scraper::{Html, Selector};
use thirtyfour::prelude::*;

use super::session::Session;

#[derive(Debug)]
pub enum DrawType {
    RoundRobin(u32),
    Elimination(u32),
    QualifyingDraw(u32, u32),
}

#[derive(Debug)]
pub enum DrawStage {
    Qualifying,
    MainDraw,
    Playoff,
}

pub struct DrawInfo {
    pub name: String,
    pub url: Option<String>,
    pub r#type: DrawType,
    pub stage: DrawStage,
    pub consolation: Option<String>,
}

fn parse_draw_type(type_str: &str, size_str: &str) -> Option<DrawType> {
    match type_str {
        "Round Robin" => Some(DrawType::RoundRobin(size_str.parse::<u32>().unwrap())),
        "Elimination" => Some(DrawType::Elimination(size_str.parse::<u32>().unwrap())),
        "Qualifying Draw" => {
            println!("{}", size_str);
            let mut ratio = size_str.split('>');
            let initial_size = ratio
                .next()
                .map(|x| x.trim().parse::<u32>().unwrap())
                .unwrap();
            let target_size = ratio
                .next()
                .map(|x| x.trim().parse::<u32>().unwrap())
                .unwrap();
            Some(DrawType::QualifyingDraw(initial_size, target_size))
        }
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

pub async fn load_draws(session: &Session, event_id: &str) -> WebDriverResult<Vec<DrawInfo>> {
    let url = format!(
        "https://www.tournamentsoftware.com/sport/draws.aspx?id={}",
        event_id.to_lowercase()
    );

    let html = session.load_content(url).await?;
    let fragment = Html::parse_fragment(&html);

    let draw_selector = Selector::parse("table tbody tr").unwrap();

    let draws = fragment
        .select(&draw_selector)
        .map(|row_el| {
            let cell_els: Vec<_> = row_el.select(&Selector::parse("td").unwrap()).collect();
            let name_el = cell_els[0]
                .select(&Selector::parse("a").unwrap())
                .next()
                .unwrap();
            let size_str = cell_els[1].text().next().unwrap();
            let type_str = cell_els[2].text().next().unwrap();
            let stage_str = cell_els[3].text().next().unwrap();
            let consolation = match cell_els[4].inner_html().as_str() {
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
