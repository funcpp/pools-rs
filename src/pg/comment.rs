use super::schema::comments::{self, dsl};
use crate::{DateTime, EntityId};
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Comment {
    pub id: EntityId,
    pub post_id: EntityId,
    pub parent_id: Option<EntityId>,
    pub author_id: String,
    pub content: String,
    pub created_at: DateTime,
    pub is_deleted: Option<bool>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = comments)]
pub struct NewComment {
    pub post_id: EntityId,
    pub parent_id: Option<EntityId>,
    pub author_id: String,
    pub content: String,
}

pub fn get_by_id(conn: &mut PgConnection, comment_id: EntityId) -> QueryResult<Comment> {
    dsl::comments.filter(dsl::id.eq(comment_id)).first(conn)
}

pub fn get_children(conn: &mut PgConnection, parent_id: EntityId) -> QueryResult<Vec<Comment>> {
    dsl::comments
        .filter(dsl::parent_id.eq(parent_id))
        .load(conn)
}

pub fn write_comment(conn: &mut PgConnection, new_comment: NewComment) -> QueryResult<Comment> {
    diesel::insert_into(dsl::comments)
        .values(&new_comment)
        .get_result(conn)
}

pub fn delete_comment(conn: &mut PgConnection, comment_id: EntityId) -> QueryResult<Comment> {
    diesel::update(dsl::comments.filter(dsl::id.eq(comment_id)))
        .set(dsl::is_deleted.eq(true))
        .get_result(conn)
}
