use dotenv;


fn telegram_caller(){
    dotenv::dotenv().ok();

    let key = dotenv::var("KEY").unwrap();

    println!("{}", key);
}