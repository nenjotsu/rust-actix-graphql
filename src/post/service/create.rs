use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use crate::post::model::{InsertablePost, Post, PostData, SlimPost};
use actix_web::web;
use diesel::prelude::*;

pub fn create(post_data: PostData, pool: web::Data<Pool>) -> ServiceResult<SlimPost> {
    let conn = &db_connection(&pool)?;
    create_post(post_data, conn)
}

pub fn create_post(post_data: PostData, conn: &PgConnection) -> ServiceResult<SlimPost> {
    use crate::schema::posts::dsl::posts;

    let post: InsertablePost = post_data.into();
    let inserted_post: Post = diesel::insert_into(posts).values(&post).get_result(conn)?;
    Ok(inserted_post.into())
}
