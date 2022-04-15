use crate::database::Pool;
use crate::errors::ServiceError;
use crate::post::model::{PostData, SinglePost, SlimPost};
use crate::post::service as post;
use actix_identity::RequestIdentity;
use actix_web::dev::Payload;
use actix_web::{web, Error, FromRequest, HttpRequest, HttpResponse};

impl FromRequest for SinglePost {
    type Error = Error;
    type Future = futures::future::Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let identity = req.get_identity();

        let slim_post = if let Some(identity) = identity {
            match serde_json::from_str::<SlimPost>(&identity) {
                Err(e) => return futures::future::err(e.into()),
                Ok(y) => Ok(Some(y)),
            }
        } else {
            Ok(None)
        };

        futures::future::ready(slim_post.map(SinglePost))
    }
}

pub async fn create(
    post_data: web::Json<PostData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    post::create(post_data.into_inner(), pool).map(|res| HttpResponse::Ok().json(&res))
}
