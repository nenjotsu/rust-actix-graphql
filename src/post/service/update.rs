use crate::errors::ServiceResult;
use crate::post::model::{InsertablePost, Post, PostData, SlimPost};
// use actix_web::web;
use diesel::prelude::*;

// pub fn update(id: i32, post_data: PostData, pool: web::Data<Pool>) -> ServiceResult<SlimPost> {
//     let conn = &db_connection(&pool)?;
//     update_post(id, post_data, conn)
// }

pub fn update_post(
    post_id: i32,
    post_data: PostData,
    conn: &PgConnection,
) -> ServiceResult<SlimPost> {
    use crate::schema::posts::dsl::{body, id, posts, published, title};

    let post: InsertablePost = post_data.into();
    let updated_post: Post = diesel::update(posts.filter(id.eq(post_id)))
        .set((published.eq(true), title.eq(post.title), body.eq(post.body)))
        .get_result(conn)?;

    Ok(updated_post.into())
}
