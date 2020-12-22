use super::http::{Request, Response, StatusCode, Method};
use super::server::Handler;
pub struct WebsiteHandler;

impl Handler for WebsiteHandler{
  fn handle_request(&mut self, request: &Request) -> Response{
    match request.method(){
      Method::GET => match request.path(){
        "/" => Response::new(StatusCode::Ok, Some("<h1>main page</h1>".to_string())),
        "/example" => Response::new(StatusCode::Ok, Some("<h1>example route</h1>".to_string())),
        _=> Response::new(StatusCode::NotFound, None),
      },
      _=> Response::new(StatusCode::NotFound, None)
    }
  }
}