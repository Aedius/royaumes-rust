use crate::auth::{JWT_ISSUER, JWT_SECRET};
use jsonwebtokens::error::Error;
use jsonwebtokens::{Algorithm, AlgorithmID, Verifier};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct JwtToken {
    pub uuid: String,
    #[serde(rename = "iss")]
    pub issuer: String,
}

#[derive(Debug)]
pub enum JwtError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtToken {
    type Error = JwtError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match req.headers().get_one("authorization") {
            None => Outcome::Failure((Status::Forbidden, JwtError::Missing)),
            Some(key) if key.len() > 7 => {
                let key = &key[7..key.len()];
                let alg = Algorithm::new_hmac(AlgorithmID::HS256, JWT_SECRET).unwrap();

                let verifier = Verifier::create()
                    .issuer(JWT_ISSUER)
                    // .audience("application_id")
                    .build()
                    .unwrap();

                let claims: Result<Value, Error> = verifier.verify(&key, &alg);

                match claims {
                    Ok(claims) => {
                        let claims: JwtToken = serde_json::from_value(claims).unwrap();
                        Outcome::Success(claims)
                    }
                    Err(e) => {
                        println!("error : {}", e);

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
    use crate::auth::JWT_ISSUER;
    use jsonwebtokens::encode;
    use serde_json::json;

    #[test]
    fn test_add() {
        let init_token = JwtToken {
            uuid: "123-456-789".to_string(),
            issuer: JWT_ISSUER.to_string(),
        };

        let alg = Algorithm::new_hmac(AlgorithmID::HS256, JWT_SECRET).unwrap();
        let header = json!({ "alg": alg.name() });
        let claims = json!(init_token);
        let token = encode(&header, &claims, &alg).unwrap();

        let verifier = Verifier::create()
            .issuer(JWT_ISSUER)
            // .audience("application_id")
            .build()
            .unwrap();

        let claims: Value = verifier.verify(token.as_str(), &alg).unwrap();
        let claims: JwtToken = serde_json::from_value(claims).unwrap();

        assert_eq!(init_token, claims);
    }
}
