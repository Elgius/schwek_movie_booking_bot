use reqwest::{self, Error};
use scraper::{Html, Selector};

pub async fn scraper_data() -> Result<(), reqwest::Error> {
    let url = "https://www.cinema.mv/";
    let resp = reqwest::get(url).await?;

    if !resp.status().is_success() {
        println!("Error: status code = {}", resp.status());
        let body_text = resp.text().await?;
        println!("Response body = {}", body_text);
    } else {
        let body_text = Html::parse_document(&resp.text().await?);
        let movie = Selector::parse(".center desktop").unwrap();

        if let Some(first_movie) = body_text.select(&movie).next() {
            let movies = first_movie.text().collect::<Vec<_>>();
            println!("{}", movies[0]);
        } else {
            println!("No elements matched the selector");
        }
    }
    Ok(())
}

pub async fn initialised() {
    println!("starting the scraper");
    let scraper_answer = scraper_data().await;
}
