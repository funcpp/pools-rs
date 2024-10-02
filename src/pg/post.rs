use super::schema::posts::{self, dsl};
use crate::{DateTime, EntityId};
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct Post {
    pub id: EntityId,
    pub channel_id: EntityId,
    pub author_id: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub channel_id: EntityId,
    pub author_id: String,
    pub title: String,
    pub content: String,
}

pub fn get_by_id(conn: &mut PgConnection, post_id: EntityId) -> QueryResult<Post> {
    dsl::posts.filter(dsl::id.eq(post_id)).first(conn)
}

pub fn write_post(conn: &mut PgConnection, new_post: NewPost) -> QueryResult<Post> {
    diesel::insert_into(dsl::posts)
        .values(&new_post)
        .get_result(conn)
}
