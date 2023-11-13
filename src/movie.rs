use std::io;


pub fn movie() {
    println!("what movie do you want to watch?");

    let mut movie_choice = String::new();
    io::stdin().read_line(&mut movie_choice).expect("there was an error when trying to get the input");

    println!("the movie you have picked: {}", movie_choice);

}