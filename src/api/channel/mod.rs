mod mutation;
mod query;

pub mod prelude {
    pub use super::mutation::ChannelMutation;
    pub use super::query::ChannelQuery;
    pub use super::ChannelNode;
}

use super::util::encode_node_id;
use crate::pg::channel::*;
use crate::{DateTime, EntityId};
use async_graphql::{ComplexObject, Context, Result, SimpleObject, ID};

pub const TYPENAME: &str = "Channel";

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct ChannelNode {
    #[graphql(skip)]
    pub eid: EntityId,
    pub name: String,
    #[graphql(skip)]
    pub parent_id: Option<EntityId>,
    pub created_at: DateTime,
}

impl From<Channel> for ChannelNode {
    fn from(channel: Channel) -> Self {
        ChannelNode {
            eid: channel.id,
            name: channel.name,
            parent_id: channel.parent_id,
            created_at: channel.created_at,
        }
    }
}

// resolver

#[ComplexObject]
impl ChannelNode {
    pub async fn id(&self) -> ID {
        encode_node_id(TYPENAME, self.eid)
    }

    async fn parent(&self, ctx: &Context<'_>) -> Result<Option<ChannelNode>> {
        let parent = match self.parent_id {
            Some(parent_id) => {
                let conn = &mut ctx.data::<crate::DBPool>()?.get()?;
                Some(get_by_id(conn, parent_id)?.into())
            }
            None => None,
        };

        Ok(parent)
    }

    async fn children(&self, ctx: &Context<'_>) -> Result<Vec<ChannelNode>> {
        let conn = &mut ctx.data::<crate::DBPool>()?.get()?;
        let children = get_children(conn, self.eid)?;
        Ok(children.into_iter().map(ChannelNode::from).collect())
    }
}
