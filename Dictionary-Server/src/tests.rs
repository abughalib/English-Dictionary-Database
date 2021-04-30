#[cfg(test)]
mod tests{

  use super::super::*;
  use super::super::routes::*;
  
  use actix_web::{http::StatusCode};
  #[actix_rt::test]
  async fn test_index(){

    let resp = index().await;

    assert_eq!(resp.status(), StatusCode::OK);
    
  }
}