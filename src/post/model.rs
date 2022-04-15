use crate::schema::*;
use shrinkwraprs::Shrinkwrap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, juniper::GraphQLObject)]
pub struct Post {
    pub id: i32,
    pub post_uuid: Uuid,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Insertable)]
#[table_name = "posts"]
pub struct InsertablePost {
    pub post_uuid: Uuid,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Deserialize, juniper::GraphQLInputObject)]
pub struct PostData {
    pub title: String,
    pub body: String,
    // pub published: bool,
}

impl From<PostData> for InsertablePost {
    fn from(post_data: PostData) -> Self {
        let PostData { title, body, .. } = post_data;

        Self {
            post_uuid: Uuid::new_v4(),
            title,
            body,
            published: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, juniper::GraphQLObject)]
pub struct SlimPost {
    pub id: i32,
    pub post_uuid: Uuid,
    pub title: String,
    pub body: String,
}

#[derive(Shrinkwrap, Clone, Default)]
pub struct SinglePost(pub Option<SlimPost>);

impl From<Post> for SlimPost {
    fn from(post: Post) -> Self {
        let Post {
            id,
            post_uuid,
            title,
            body,
            ..
        } = post;

        Self {
            id,
            post_uuid,
            title,
            body,
        }
    }
}
