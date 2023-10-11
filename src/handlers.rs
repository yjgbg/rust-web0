use actix_web::Error;

#[actix_web::get("/")]
pub async fn hello() -> Result<String,Error> {
  Ok("Hello".to_string())
}