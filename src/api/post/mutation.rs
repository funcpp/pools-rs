use super::*;
use crate::api::util::decode_node_id;
use crate::pg::post::*;
use async_graphql::*;

#[derive(Default)]
pub struct PostMutation;

#[Object]
impl PostMutation {
    async fn write_post(
        &self,
        ctx: &Context<'_>,
        channel_id: ID,
        title: String,
        content: String,
    ) -> async_graphql::Result<PostNode> {
        let conn = &mut ctx.data::<crate::DBPool>()?.get()?;

        let channel_id = decode_node_id(&channel_id)?.1;

        let new_post = NewPost {
            channel_id,
            author_id: "anonymous".to_string(),
            title,
            content,
        };

        let entity: Post = write_post(conn, new_post)?;

        Ok(PostNode::from(entity))
    }
}
