mod mutation;

pub mod prelude {
    pub use super::mutation::PostMutation;
    pub use super::PostNode;
}

use super::util::encode_node_id;
use crate::pg::post::*;
use crate::{DateTime, EntityId};
use async_graphql::{ComplexObject, SimpleObject, ID};

pub const TYPENAME: &str = "Post";

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct PostNode {
    #[graphql(skip)]
    pub eid: EntityId,
    pub post_id: EntityId,
    pub parent_id: Option<EntityId>,
    pub content: String,
    pub created_at: DateTime,
    pub is_deleted: Option<bool>,
    pub deleted_at: Option<DateTime>,
}

impl From<Post> for PostNode {
    fn from(entity: Post) -> Self {
        PostNode {
            eid: entity.id,
            post_id: entity.channel_id,
            parent_id: None,
            content: entity.content,
            created_at: entity.created_at,
            is_deleted: None,
            deleted_at: None,
        }
    }
}

#[ComplexObject]
impl PostNode {
    pub async fn id(&self) -> ID {
        encode_node_id(TYPENAME, self.eid)
    }
}
