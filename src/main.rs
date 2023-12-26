mod internal;

use colored::Colorize;
use internal::{draws::load_draws, events::load_events, session::Session};
use thirtyfour::prelude::*;

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
        println!(
            "{}: {}",
            "Type".yellow(),
            format!("{:?}", draw.r#type).cyan().bold()
        );
        println!(
            "{}: {}",
            "Stage".yellow(),
            format!("{:?}", draw.stage).cyan().bold()
        );
        println!(
            "{}: {}",
            "Consolation".yellow(),
            format!("{:?}", draw.consolation).cyan().bold()
        );
    }

    session.close().await?;

    Ok(())
}
