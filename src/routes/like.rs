use warp::http::StatusCode;

use handle_errors::Error;
use crate::account::Account;
use crate::types::{
	tweet::TweetId,
};

pub async fn add_like(
    tweet_id: String,
    action: String,
    account: Account,
) -> Result<impl warp::Reply, warp::Rejection> {
    
    if let Some(tweet) = account.tweets.write().await.get_mut(&TweetId(tweet_id)) {
        match action.as_str() {
            "like" => {
                tweet.likes += 1;
                return Ok(warp::reply::with_status("Liked!", StatusCode::OK));
            }
            "unlike" => {
                if tweet.likes > 0 {
                    tweet.likes -= 1;
                    return Ok(warp::reply::with_status("Unliked!", StatusCode::OK));
                } else {
                    return Ok(warp::reply::with_status("Tweet does not have any like", StatusCode::OK));
                }
            }
            _ => return Ok(warp::reply::with_status("Invalid action", StatusCode::BAD_REQUEST)),
        }
    } else {
        return Err(warp::reject::custom(Error::TweetNotFound));
    }
}