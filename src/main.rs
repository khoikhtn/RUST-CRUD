use handle_errors::return_error;
use warp::{http::Method, Filter};

mod routes;
mod account;
mod types;

#[tokio::main]
async fn main() {
    let account = account::Account::new("postgres://postgres:phKt290221@localhost:5432/rust_twitter").await;

    sqlx::migrate!()
        .run(&account.clone().connection)
        .await
        .expect("Cannot migrate DB");

    let account_filter = warp::any().map(move || account.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[
            Method::PUT,
            Method::DELETE,
            Method::GET,
            Method::POST,
        ]);

    let get_tweets = warp::get()
        .and(warp::path("tweets"))
        .and(warp::path::end())
        .and(account_filter.clone())
        .and_then(routes::tweet::get_tweets);

    let add_tweet = warp::post()
        .and(warp::path("tweets"))
        .and(warp::path::end())
        .and(account_filter.clone())
        .and(warp::body::json())
        .and_then(routes::tweet::add_tweet);

    let update_tweet = warp::put()
        .and(warp::path("tweets"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(account_filter.clone())
        .and(warp::body::json())
        .and_then(routes::tweet::update_tweet);

    let delete_tweet = warp::delete()
        .and(warp::path("tweets"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(account_filter.clone())
        .and_then(routes::tweet::delete_tweet);

    let add_like = warp::post()
        .and(warp::path("likes"))
        .and(warp::path::param::<i32>())
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(account_filter.clone())
        .and_then(routes::like::add_like);

    let routes = get_tweets
        .or(add_tweet)
        .or(update_tweet)
        .or(delete_tweet)
        .or(add_like)
        .with(cors)
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}