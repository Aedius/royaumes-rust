use rocket::response::Responder;

#[derive(Debug, Responder)]
pub enum AuthError {
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 500)]
    AlreadyExist(String),
    #[response(status = 500)]
    WrongQuantity(String),
    #[response(status = 500)]
    Other(String),
}
