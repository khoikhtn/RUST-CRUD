# Rust_CRUD
This repo contains 2 versions of a simple Twitter CRUD task: in_memory and RDBMS (each in its corresponding branch). Both use the commands below:
1. Run `cargo r`.
2. There are a total of 5 operations you can do:
   - Get all tweets: `curl http://localhost:3030/tweets`
   - Add a tweet: `curl -X POST http://localhost:3030/tweets -H "Content-Type: application/json" -d "{\"id\": \"1\", \"title\": \"Your tweet title\", \"content\": \"Your content\", \"tags\": [\"tag1\", \"tag2\"]}"`
   - Update a tweet: `curl -X PUT http://localhost:3030/tweets/{tweet_id} -H "Content-Type: application/json" -d "{\"id\": \"2\", \"title\": \"Updated Title\", \"content\": \"Updated Content\", \"tags\": [\"tag1\", \"tag2\"]}"`
   - Delete a tweet: `curl -X DELETE http://localhost:3030/tweets/{tweet_id}`
   - Like (unlike) a tweet: `curl -X POST http://localhost:3030/likes/{tweet_id}/{like/unlike}`
  
* Note that in the RDBMS code, I haven't backed up my db yet so please spend 1 minute to create one db called `rust_twitter`, then direct to the folder and run the migration: `sqlx migrate run --database-url postgresql://localhost:5432/rust_twitter`, remember to config your own URL in `main.rs`: `let account = account::Account::new("postgres://your_username:your_password@localhost:5432/rust_twitter").await;`
  
* <b>Self-evaluation</b>: This project only focuses on main functionalities and code structuring, I haven't got enough time to write test cases, learn to use Docker for containerizing purpose and implement the advanced functionalities.
