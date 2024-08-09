use serde::{Deserialize, Serialize};

use crate::types::tweet::TweetId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Like {
	pub id: LikeId,
	pub tweet_id: TweetId,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LikeId(pub i32);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewLike {
	pub tweet_id: TweetId,
}