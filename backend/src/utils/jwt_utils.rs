use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, EncodingKey, Header}; // , decode, DecodingKey, Validation
use std::error::Error;
use dotenv::dotenv;

// Represent the payload of the jwt
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
  sub: String,
  exp: usize,
  // Allow us to extract the email and username from the connected user.
  email: String,
  username: String,
}

pub fn generate_token(u_id: i32, u_email: String, u_username: String) -> Result<String, Box<dyn Error>>{
  dotenv().ok();

  let jwt_secret = std::env::var("JWT_SECRET")?;
  let encoded_key = EncodingKey::from_secret(jwt_secret.as_bytes());

  // Content of the jwt token
  let claims = Claims{
    sub: u_id.to_string(),
    exp: (Utc::now() + Duration::hours(1)).timestamp() as usize,
    email: u_email,
    username: u_username,
  };

  // Generate the jwt token itslef
  let token = encode(
    &Header::default(), // HS256
    &claims,
    &encoded_key,
  )?;

  // println!("Token: {}", token);
  Ok(token)
}

// pub fn validate_token(token: &str) -> Result<Claims, Box<dyn Error>>{
//   let jwt_secret = std::env::var("JWT_SECRET")?;
//   let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());

//   let validation = Validation::default();

//   let token_data = decode(
//     token,
//     &decoding_key,
//     &validation)?;
//   Ok(token_data.claims)
// }