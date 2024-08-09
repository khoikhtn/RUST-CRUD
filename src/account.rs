use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;

use handle_errors::Error;

use crate::types::{
	like::{Like, LikeId, NewLike},
	tweet::{Tweet, TweetId, NewTweet},
};

#[derive(Debug, Clone)]
pub struct Account {
	pub connection: PgPool,
}

impl Account {
	pub async fn new(db_url: &str) -> Self {
		let db_pool = match PgPoolOptions::new()
			.max_connections(5)
			.connect(db_url)
			.await
		{
			Ok(pool) => pool,
			Err(e) => panic!("Couldn't establish DB connection: {}", e),
		};

		Account {
			connection: db_pool,
		}
	}

	pub async fn get_tweets(
		&self,
	) -> Result<Vec<Tweet>, Error> {
		match sqlx::query("SELECT * FROM tweets")
		.map(|row: PgRow| Tweet {
			id: TweetId(row.get("id")),
			title: row.get("title"),
			content: row.get("content"),
			tags: row.get("tags"),
			likes: row.get("likes"),
		})
		.fetch_all(&self.connection)
		.await
		{
			Ok(tweets) => Ok(tweets),
			Err(e) => {
				tracing::event!(tracing::Level::ERROR, "{:?}", e);
				Err(Error::DatabaseQueryError)
			}
		}
	}

	pub async fn add_tweet(
		&self,
		new_tweet: NewTweet,
	) -> Result<Tweet, Error> {
		match sqlx::query(
			"INSERT INTO tweets (title, content, tags, likes) 
			 VALUES ($1, $2, $3, $4)
			 RETURNING id, title, content, tags, likes",
		)
		.bind(new_tweet.title)
		.bind(new_tweet.content)
		.bind(new_tweet.tags)
		.bind(0)
		.map(|row: PgRow| Tweet {
			id: TweetId(row.get("id")),
			title: row.get("title"),
			content: row.get("content"),
			tags: row.get("tags"),
			likes: row.get("likes"),
		})
		.fetch_one(&self.connection)
		.await
		{
			Ok(tweet) => Ok(tweet),
			Err(e) => {
				tracing::event!(tracing::Level::ERROR, "{:?}", e);
				Err(Error::DatabaseQueryError)
			}
		}
	}

	pub async fn update_tweet(
		&self,
		tweet: Tweet,
		tweet_id: i32,
	) -> Result<Tweet, Error> {
		match sqlx::query(
			"UPDATE tweets SET title = $1, content = $2, tags = $3
			 WHERE id = $4
			 RETURNING id, title, content, tags, likes",
		)
		.bind(tweet.title)
		.bind(tweet.content)
		.bind(tweet.tags)
		.bind(tweet_id)
		.map(|row: PgRow| Tweet {
			id: TweetId(row.get("id")),
			title: row.get("title"),
			content: row.get("content"),
			tags: row.get("tags"),
			likes: row.get("likes"),
		})
		.fetch_one(&self.connection)
		.await
		{
			Ok(tweet) => Ok(tweet),
			Err(e) => {
				tracing::event!(tracing::Level::ERROR, "{:?}", e);
				Err(Error::DatabaseQueryError)
			}
		}
	}

	pub async fn delete_tweet(
		&self,
		tweet_id: i32,
	) -> Result<bool, Error> {
		match sqlx::query("DELETE FROM tweets WHERE id = $1")
		.bind(tweet_id)
		.execute(&self.connection)
		.await
		{
			Ok(_) => Ok(true),
			Err(e) => {
				tracing::event!(tracing::Level::ERROR, "{:?}", e);
				Err(Error::DatabaseQueryError)
			}
		}
	}

	pub async fn add_like(
		&self,
		tweet_id: i32,
		action: String,
	) -> Result<Tweet, Error> {
		let update_likes = match action.as_str() {
			"like" => 1,
			"unlike" => -1,
			_ => return Err(Error::InvalidAction),
		};

		let result = sqlx::query(
				"UPDATE tweets SET likes = likes + $1 
				 WHERE id = $2 
				 RETURNING id, title, content, tags, likes",
			)
			.bind(update_likes)
			.bind(tweet_id)
			.map(|row: PgRow| Tweet {
				id: TweetId(row.get("id")),
				title: row.get("title"),
				content: row.get("content"),
				tags: row.get("tags"),
				likes: row.get("likes"),
			})
			.fetch_one(&self.connection)
			.await;

		match result {
			Ok(tweet) => Ok(tweet),
			Err(e) => {
				tracing::event!(tracing::Level::ERROR, "{:?}", e);
				Err(Error::DatabaseQueryError)
			}
		}
	}
}