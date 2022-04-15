use crate::errors::ServiceResult;
use crate::post::model::Post;
use diesel::prelude::*;

pub fn delete_post(post_id: i32, conn: &PgConnection) -> ServiceResult<Post> {
    use crate::schema::posts::dsl::{id, posts};
    let deleted_record: Post = diesel::delete(posts.filter(id.eq(post_id))).get_result(conn)?;

    Ok(deleted_record)
}
