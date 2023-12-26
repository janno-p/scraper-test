use colored::Colorize;
use thirtyfour::prelude::*;

pub struct Session {
    driver: WebDriver,
}

impl Session {
    pub async fn init() -> WebDriverResult<Self> {
        let caps = DesiredCapabilities::firefox();
        // caps.set_headless()?;

        let driver = WebDriver::new("http://localhost:4444", caps).await?;

        driver.goto("https://www.tournamentsoftware.com/").await?;

        let accept_cookies_button = driver.find(By::Css("button.js-accept-basic")).await?;
        accept_cookies_button.click().await?;
        accept_cookies_button.wait_until().stale().await?;

        Ok(Self { driver })
    }

    pub async fn load_content(&self, url: String) -> WebDriverResult<String> {
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

    pub async fn close(self) -> WebDriverResult<()> {
        self.driver.quit().await
    }
}
