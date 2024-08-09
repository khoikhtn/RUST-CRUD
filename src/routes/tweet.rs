use std::collections::HashMap;

use warp::http::StatusCode;

use crate::account::Account;
use crate::types::tweet::{NewTweet, Tweet};

pub async fn get_tweets(
	account: Account,
) -> Result<impl warp::Reply, warp::Rejection> {
	match account.get_tweets().await {
		Ok(res) => Ok(warp::reply::json(&res)),
		Err(e) => Err(warp::reject::custom(e)),
	}
}

pub async fn add_tweet(
	account: Account,
	new_tweet: NewTweet,
) -> Result<impl warp::Reply, warp::Rejection> {
	match account.add_tweet(new_tweet).await {
		Ok(_) => {
			Ok(warp::reply::with_status("Tweet added", StatusCode::OK))
		}
		Err(e) => Err(warp::reject::custom(e)),
	}
}

pub async fn update_tweet(
	id: i32,
	account: Account,
	tweet: Tweet,
) -> Result<impl warp::Reply, warp::Rejection> {
	match account.update_tweet(tweet, id).await {
		Ok(res) => Ok(warp::reply::json(&res)),
		Err(e) => Err(warp::reject::custom(e)),
	}
}

pub async fn delete_tweet(
	id: i32,
	account: Account,
) -> Result<impl warp::Reply, warp::Rejection> {
	match account.delete_tweet(id).await {
		Ok(_) => Ok(warp::reply::with_status(
			format!("Tweet {} deleted", id),
			StatusCode::OK,
		)),
		Err(e) => Err(warp::reject::custom(e)),
	}
}