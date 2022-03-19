// use actix_web::dev::ServiceRequest;
// use actix_web::Error;
// use actix_web_httpauth::extractors::AuthenticationError;
// use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
//
// pub async fn validate_token(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
//     if credentials.token() == "mF_9.B5f-4.1JqM" {
//         Ok(req)
//     } else {
//         let config = req.app_data::<Config>().cloned()
//             .unwrap_or_default()
//             .scope("urn:example:channel=HBO&urn:example:rating=G,PG-13");
//
//         Err(AuthenticationError::from(config).into())
//     }
// }