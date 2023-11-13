use reqwest::Error;
use scraper::{Html, Selector};
use std::time::Duration;
use tokio::time::sleep;


async fn scraper() {
    let url = "https://www.cinema.mv/schedule";
    let mut interval = tokio::time::interval(Duration::from_secs(180));

    loop {
        interval.tick().await;
        let resp = reqwest::get(url).await?;
        let body = resp.text().await?;
        let fragment = Html::parse_document(&body);
        let selector = Selector::parse("movie button").unwrap();

        for element in fragment.select(&selector){
            let movie_info = element.text().collect()::<Vec<_>>().join("");
            println!("{}", movie_info);
        }


    }
}