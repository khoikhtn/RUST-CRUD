use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Tweet {
	pub id: TweetId,
	pub title: String,
	pub content: String,
	pub tags: Option<Vec<String>>,
	pub likes: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TweetId(pub String);