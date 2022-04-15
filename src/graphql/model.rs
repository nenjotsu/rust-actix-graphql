use crate::cli_args::Opt;
use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use crate::jwt::model::{DecodedToken, Token};
use crate::post::model::{Post, PostData, SlimPost};
use crate::post::service as post;
use crate::user::model::{LoggedUser, SlimUser, User, UserData};
use crate::user::service as user;
use crate::user::service::token::ClaimsResponse;
use diesel::PgConnection;
use juniper::Context as JuniperContext;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct Context {
    pub opt: Opt,
    pub db: Arc<PooledConnection>,
    pub user: LoggedUser,
    pub token: DecodedToken,
}

impl JuniperContext for Context {}

impl Context {
    pub fn new(token: DecodedToken, user: LoggedUser, pool: PooledConnection, opt: Opt) -> Self {
        Self {
            opt,
            token,
            user,
            db: Arc::new(pool),
        }
    }
}

pub(crate) struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    pub fn users(
        context: &Context,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<User>> {
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        crate::user::has_role(&context.user, "user")?;

        user::list::find_all_users(&context, limit, offset)
    }

    pub fn generate_token(context: &Context) -> ServiceResult<Token> {
        user::token::generate(&context)
    }

    pub fn decode_token(context: &Context) -> ServiceResult<&ClaimsResponse> {
        user::token::decode(&context)
    }

    pub fn posts(
        context: &Context,
        limit: Option<i32>,
        offset: Option<i32>,
        keyword: Option<String>,
    ) -> ServiceResult<Vec<Post>> {
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);
        let keyword: String = keyword.unwrap_or("".to_string());

        post::list::find_all_posts(&context, limit, offset, keyword)
    }
}

pub(crate) struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    pub fn register(context: &Context, data: UserData) -> ServiceResult<SlimUser> {
        use crate::user::service::register::create_user;
        let conn: &PgConnection = &context.db;

        Ok(create_user(data, conn)?)
    }
    pub fn createPost(context: &Context, data: PostData) -> ServiceResult<SlimPost> {
        use crate::post::service::create::create_post;
        let conn: &PgConnection = &context.db;

        Ok(create_post(data, conn)?)
    }
    pub fn updatePost(context: &Context, id: i32, data: PostData) -> ServiceResult<SlimPost> {
        use crate::post::service::update::update_post;
        let conn: &PgConnection = &context.db;

        Ok(update_post(id, data, conn)?)
    }
}

pub(crate) type Schema = juniper::RootNode<'static, QueryRoot, Mutation>;

pub(crate) fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, Mutation {})
}
