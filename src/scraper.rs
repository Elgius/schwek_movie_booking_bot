// this module is still under construction, do not run it, you have been warned


use reqwest::Error;
use scraper::{Html, Selector};
use std::time::Duration;
use tokio::time::{sleep, interval};

struct Movie {
    title: String,
    time: String,
    location: String,
    url: String,
}


pub async fn check_for_movie(movies: Vec<Movie>, movie_name:String){
    for movie in movies {
        if movie.title.to_lowercase().contains(&movie_name.to_lowercase()){
            let msg = format!(
                "MOVIE ON SALE!!🚨🚨🚨\n{}\n{}\n{},\nhttps://www.cinema.mv{}",
                movie.title, movie.time, movie.location, movie.url
            );
            println!("{}", msg)
        }
}
}


pub async fn scraper() -> Result<(), Error> {
    let url = "https://www.cinema.mv/schedule";
    let mut interval = interval(Duration::from_secs(180));

    loop {
        interval.tick().await;
        let resp = reqwest::get(url).await?;
        let body = resp.text().await?;
        let fragment = Html::parse_document(&body);
   
        let mut movies: Vec<Movie> = Vec::new();
        for element in fragment.descendants() {
            if let scraper::Node::Element(element) = element {
                let movie_info: Vec<_> = element.text().collect();
                let movie_info_str = movie_info.join(",");
                println!("{}", movie_info_str);
        
                // Parse movie_info_str into a Movie struct and push it to the movies vector
                let movie_data: Vec<&str> = movie_info_str.split(",").collect();
                if movie_data.len() == 4 {
                    let movie = Movie {
                        title: movie_data[0].to_string(),
                        time: movie_data[1].to_string(),
                        location: movie_data[2].to_string(),
                        url: movie_data[3].to_string(),
                    };
                    movies.push(movie);
                }
            }
        }
   
        // Replace "MovieName" with the actual name of the movie you're looking for
        check_for_movie(movies, "MovieName".to_string()).await;
    }
   }





pub async fn start(){
    if let Err(e) = scraper().await{
        eprintln!("Error occured at: {}", e);
    }
}