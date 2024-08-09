use warp::http::StatusCode;

use handle_errors::Error;
use crate::account::Account;
use crate::types::like::NewLike;

pub async fn add_like(
	tweet_id: i32,
	action: String,
	account: Account,
	//new_like: NewLike,
) -> Result<impl warp::Reply, warp::Rejection> {
	match account.add_like(tweet_id, action.clone()).await {
		Ok(_) => {
			let message = match action.as_str() {
				"like" => "Liked!",
				"unlike" => "Unliked!",
				_ => return Err(warp::reject::custom(Error::InvalidAction)),
			};

			Ok(warp::reply::with_status(message, StatusCode::OK))
		}
		Err(e) => Err(warp::reject::custom(e)),
	}
}