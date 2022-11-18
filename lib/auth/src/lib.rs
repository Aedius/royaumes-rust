use jsonwebtokens::error::Error;
use jsonwebtokens::{encode, Algorithm, AlgorithmID, Verifier};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::marker::PhantomData;

pub trait Issuer {
    fn name() -> String;
    fn secret() -> String;
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct JwtToken<T: Issuer> {
    uuid: String,
    #[serde(rename = "iss")]
    issuer: String,
    _marker: PhantomData<T>,
}

#[derive(Debug)]
pub enum JwtError {
    Missing,
    Invalid,
}

impl<T: Issuer> JwtToken<T> {
    pub fn uuid(&self) -> &str {
        self.uuid.as_str()
    }

    pub fn create(id: String) -> String {
        let alg = Algorithm::new_hmac(AlgorithmID::HS256, T::secret()).unwrap();
        let header = json!({ "alg": alg.name() });
        let claims = json!(JwtToken {
            uuid: id,
            issuer: T::name(),
            _marker: PhantomData::<T>
        });
        encode(&header, &claims, &alg).unwrap()
    }

    pub fn check_claims(token: &str) -> Result<Self, String> {
        let alg = Algorithm::new_hmac(AlgorithmID::HS256, T::secret()).unwrap();

        let verifier = Verifier::create()
            .issuer(T::name())
            // .audience("application_id")
            .build()
            .unwrap();

        let claims: Result<Value, Error> = verifier.verify(token, &alg);

        match claims {
            Ok(claims) => {
                let claims: JwtToken<T> = serde_json::from_value(claims).unwrap();
                Ok(claims)
            }
            Err(e) => Err(format! {"error : {}", e}),
        }
    }
}

#[rocket::async_trait]
impl<'r, T: Issuer> FromRequest<'r> for JwtToken<T> {
    type Error = JwtError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("authorization") {
            None => Outcome::Failure((Status::Forbidden, JwtError::Missing)),
            Some(key) if key.len() > 7 => {
                let key = &key[7..key.len()];

                let checked = Self::check_claims(key);
                match checked {
                    Ok(token) => Outcome::Success(token),
                    Err(e) => {
                        println!("oupsi : {}", e);

                        Outcome::Failure((Status::Unauthorized, JwtError::Invalid))
                    }
                }
            }
            Some(_) => Outcome::Failure((Status::Forbidden, JwtError::Missing)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtokens::encode;
    use serde_json::json;

    #[derive(Debug, PartialEq)]
    struct IssuerA {}

    impl Issuer for IssuerA {
        fn name() -> String {
            "A".to_string()
        }

        fn secret() -> String {
            "ThisIsSecret".to_string()
        }
    }

    #[derive(Debug, PartialEq)]
    struct IssuerB {}

    impl Issuer for IssuerB {
        fn name() -> String {
            "B".to_string()
        }

        fn secret() -> String {
            "AlsoThis!".to_string()
        }
    }

    #[test]
    fn test_checked() {
        let init_token: JwtToken<IssuerA> = JwtToken {
            uuid: "123-456-789".to_string(),
            issuer: IssuerA::name(),
            _marker: PhantomData,
        };

        let alg = Algorithm::new_hmac(AlgorithmID::HS256, IssuerA::secret()).unwrap();
        let header = json!({ "alg": alg.name() });
        let claims = json!(init_token);
        let token = encode(&header, &claims, &alg).unwrap();

        // println!("{token}");

        let checked: JwtToken<IssuerA> = JwtToken::check_claims(token.as_str()).unwrap();
        assert_eq!(init_token, checked);
    }

    #[test]
    fn test_issuer() {
        let token = "eyJhbGciOiJIUzI1NiJ9.eyJfbWFya2VyIjpudWxsLCJpc3MiOiJBIiwidXVpZCI6IjEyMy00NTYtNzg5In0.xflj8p-LkhMax0p1Hextc7BfIf4SQgSrl5WEzwmOvlI";

        let checked: Result<JwtToken<IssuerA>, String> = JwtToken::check_claims(token);
        assert!(checked.is_ok(), "issuer is not a");
        let checked: Result<JwtToken<IssuerB>, String> = JwtToken::check_claims(token);
        assert!(checked.is_err(), "issuer is b");
    }
}
