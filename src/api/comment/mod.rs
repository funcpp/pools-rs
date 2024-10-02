mod mutation;

pub mod prelude {
    pub use super::mutation::CommentMutation;
    pub use super::CommentNode;
}

use super::{util::encode_node_id, PostNode};
use crate::pg::comment::*;
use crate::{DateTime, EntityId};
use async_graphql::{ComplexObject, Context, Result, SimpleObject, ID};

pub const TYPENAME: &str = "Comment";

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct CommentNode {
    #[graphql(skip)]
    pub eid: EntityId,
    pub post_id: EntityId,
    pub parent_id: Option<EntityId>,
    pub author_id: String,
    pub content: String,
    pub created_at: DateTime,
    pub is_deleted: Option<bool>,
    pub deleted_at: Option<DateTime>,
}

impl From<Comment> for CommentNode {
    fn from(entity: Comment) -> Self {
        CommentNode {
            eid: entity.id,
            post_id: entity.post_id,
            parent_id: entity.parent_id,
            author_id: entity.author_id,
            content: entity.content,
            created_at: entity.created_at,
            is_deleted: entity.is_deleted,
            deleted_at: entity.deleted_at,
        }
    }
}

#[ComplexObject]
impl CommentNode {
    pub async fn id(&self) -> ID {
        encode_node_id(TYPENAME, self.eid)
    }

    async fn post(&self, ctx: &Context<'_>) -> Result<PostNode> {
        let conn = &mut ctx.data::<crate::DBPool>()?.get()?;
        let post = crate::pg::post::get_by_id(conn, self.post_id)?;
        Ok(post.into())
    }

    async fn parent(&self, ctx: &Context<'_>) -> Result<Option<CommentNode>> {
        let parent = match self.parent_id {
            Some(parent_id) => {
                let conn = &mut ctx.data::<crate::DBPool>()?.get()?;
                Some(get_by_id(conn, parent_id)?.into())
            }
            None => None,
        };

        Ok(parent)
    }

    async fn children(&self, ctx: &Context<'_>) -> Result<Vec<CommentNode>> {
        let conn = &mut ctx.data::<crate::DBPool>()?.get()?;
        let children = get_children(conn, self.eid)?;

        Ok(children.into_iter().map(|c| c.into()).collect())
    }
}
