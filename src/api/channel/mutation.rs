use super::*;
use crate::api::util::decode_node_id;
use crate::pg::channel::*;
use async_graphql::*;

#[derive(Default)]
pub struct ChannelMutation;

#[Object]
impl ChannelMutation {
    async fn create_channel(
        &self,
        ctx: &Context<'_>,
        name: String,
        parent_id: Option<ID>,
    ) -> async_graphql::Result<ChannelNode> {
        let conn = &mut ctx.data::<crate::DBPool>()?.get()?;

        let parent_id = match parent_id {
            Some(parent_id) => Some(decode_node_id(&parent_id)?.1),
            None => None,
        };

        let new_channel = NewChannel { name, parent_id };

        let channel = create_channel(conn, new_channel)?;

        Ok(channel.into())
    }
}
