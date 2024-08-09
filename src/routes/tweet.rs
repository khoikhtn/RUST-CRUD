use warp::http::StatusCode;

use handle_errors::Error;
use crate::account::Account;
use crate::types::tweet::{Tweet, TweetId};

pub async fn get_tweets(
	account: Account,
) -> Result<impl warp::Reply, warp::Rejection> {
	let res: Vec<Tweet> = account.tweets.read().await.values().cloned().collect();
	Ok(warp::reply::json(&res))
}

pub async fn add_tweet(
	account: Account,
	tweet: Tweet,
) -> Result<impl warp::Reply, warp::Rejection> {
	account.tweets.write().await.insert(tweet.clone().id, tweet);
	
	Ok(warp::reply::with_status("Tweeted!", StatusCode::OK))
}

pub async fn update_tweet(
	id: String,
	account: Account,
	updated_tweet: Tweet,
) -> Result<impl warp::Reply, warp::Rejection> {
	match account.tweets.write().await.get_mut(&TweetId(id)) {
		Some(tweet) => *tweet = updated_tweet,
		None => return Err(warp::reject::custom(Error::TweetNotFound)),
	}

	Ok(warp::reply::with_status("Tweet updated", StatusCode::OK))
}

pub async fn delete_tweet(
	id: String,
	account: Account,
) -> Result<impl warp::Reply, warp::Rejection> {
	match account.tweets.write().await.remove(&TweetId(id)) {
		Some(_) => (),
		None => return Err(warp::reject::custom(Error::TweetNotFound)),
	}

	Ok(warp::reply::with_status("Tweet deleted!", StatusCode::OK))
}