use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use crate::graphql::model::Context;
use crate::post::model::Post;
use diesel::prelude::*;

pub(crate) fn find_all_posts(
    context: &Context,
    limit: i32,
    offset: i32,
    keyword: String,
) -> ServiceResult<Vec<Post>> {
    use crate::schema::posts::dsl::*;
    let conn: &PooledConnection = &context.db;

    let mut k = String::from(keyword);
    k.push_str("%");

    Ok(posts
        .filter(title.ilike(k))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Post>(conn)?)
}
