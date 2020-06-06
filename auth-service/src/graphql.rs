use async_graphql::*;

use crate::persistence::connection::PgPool;
use crate::persistence::model::NewUserEntity;
use crate::persistence::repository;
use crate::utils::{create_token, hash_password, verify};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub struct Query;

#[Object(extends)]
impl Query {}

pub struct Mutation;

#[Object(extends)]
impl Mutation {
    async fn create_user(&self, ctx: &Context<'_>, user: UserInput) -> ID {
        let conn = ctx.data::<PgPool>().get().expect("Can't get DB connection");

        let new_user = NewUserEntity {
            username: user.username,
            hash: hash_password(user.password.as_str()).expect("Can't get hash for password"),
            first_name: user.first_name,
            last_name: user.last_name,
            role: user.role,
        };

        let created_user_entity = repository::create(new_user, &conn).expect("Can't create user");

        created_user_entity.id.into()
    }

    async fn sign_in(&self, ctx: &Context<'_>, sign_in_data: SignInInput) -> String {
        let conn = ctx.data::<PgPool>().get().expect("Can't get DB connection");

        let user = repository::get_user(&sign_in_data.username, &conn).expect("Can't get hash for a user");

        if let Ok(matching) = verify(&user.hash, &sign_in_data.password) {
            if matching {
                return create_token(user);
            }
        }

        panic!("Can't authenticate a user")
    }
}

#[InputObject]
struct UserInput {
    username: String,
    password: String,
    first_name: String,
    last_name: String,
    role: String,
}

#[InputObject]
struct SignInInput {
    username: String,
    password: String,
}
