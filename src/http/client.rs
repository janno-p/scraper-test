use thirtyfour::prelude::*;

pub struct Client {
    driver: WebDriver,
}

const BASE_URL: &str = "https://www.tournamentsoftware.com";
const WEB_DRIVER_URL: &str = "http://localhost:4444";

fn site_url(url: &str) -> String {
    format!("{}{}", BASE_URL, url)
}

impl Client {
    pub async fn init() -> anyhow::Result<Self> {
        let mut caps = DesiredCapabilities::firefox();
        if let Some(v) = option_env!("BROWSE_HEADLESS") {
            match v.to_lowercase().as_str() {
                "1" | "true" | "t" | "yes" | "y" => caps.set_headless()?,
                _ => {}
            }
        }

        let driver = WebDriver::new(WEB_DRIVER_URL, caps).await?;

        driver.goto(site_url("/")).await?;

        let accept_cookies_button = driver.find(By::Css("button.js-accept-basic")).await?;
        accept_cookies_button.click().await?;
        accept_cookies_button.wait_until().stale().await?;

        Ok(Self { driver })
    }

    pub async fn download(&self, url: &str) -> anyhow::Result<String> {
        self.driver.goto(site_url(url)).await?;

        let content = self.driver.find(By::Css("#content")).await?;
        content.wait_until().displayed().await?;

        let html = content.outer_html().await?;

        Ok(html)
    }

    pub async fn close(self) -> anyhow::Result<()> {
        self.driver.quit().await?;
        Ok(())
    }
}
