use super::schema::channels::{self, dsl};
use crate::{DateTime, EntityId};
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = channels)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Channel {
    pub id: EntityId,
    pub name: String,
    pub parent_id: Option<EntityId>,
    pub created_at: DateTime,
}

#[derive(Insertable)]
#[diesel(table_name = channels)]
pub struct NewChannel {
    pub name: String,
    pub parent_id: Option<EntityId>,
}

pub fn create_channel(conn: &mut PgConnection, new_channel: NewChannel) -> QueryResult<Channel> {
    diesel::insert_into(dsl::channels)
        .values(&new_channel)
        .get_result(conn)
}

pub fn get_by_id(conn: &mut PgConnection, channel_id: EntityId) -> QueryResult<Channel> {
    dsl::channels.filter(dsl::id.eq(channel_id)).first(conn)
}

pub fn get_children(conn: &mut PgConnection, parent_id: EntityId) -> QueryResult<Vec<Channel>> {
    dsl::channels
        .filter(dsl::parent_id.eq(parent_id))
        .load(conn)
}
