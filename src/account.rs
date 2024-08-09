use tokio::sync::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

use crate::types::{
	tweet::{Tweet, TweetId},
	like::{Like, LikeId},
};

#[derive(Debug, Clone)]
pub struct Account {
	pub tweets: Arc<RwLock<HashMap<TweetId, Tweet>>>,
	pub likes: Arc<RwLock<HashMap<LikeId, Like>>>,
}

impl Account {
	pub fn new() -> Self {
		Account {
			tweets: Arc::new(RwLock::new(Self::init())),
			likes: Arc::new(RwLock::new(HashMap::new())),
		}
	}

	fn init() -> HashMap<TweetId, Tweet> {
		let file = include_str!("../tweets.json");
		serde_json::from_str(file).expect("can't read tweets.json")
	}
}