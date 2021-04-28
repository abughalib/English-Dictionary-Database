#[cfg(test)]
mod tests{

  use super::super::*;
  use actix_web::{http::StatusCode, test};
  #[actix_rt::test]
  async fn test_index(){
    // let req = test::TestRequest::with_header("content-type", "text/plain")
    //     .to_request();
    let resp = index().await;

    assert_eq!(resp.status(), StatusCode::OK);
    
  }
}