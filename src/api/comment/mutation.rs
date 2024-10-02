use super::*;
use crate::api::util::decode_node_id;
use crate::pg::comment::*;
use async_graphql::*;

#[derive(Default)]
pub struct CommentMutation;

#[Object]
impl CommentMutation {
    async fn write_comment(
        &self,
        ctx: &Context<'_>,
        post_id: ID,
        parent_id: Option<ID>,
        content: String,
    ) -> async_graphql::Result<CommentNode> {
        let conn = &mut ctx.data::<crate::DBPool>()?.get()?;

        let post_id = decode_node_id(&post_id)?.1;

        let new = NewComment {
            post_id,
            parent_id: parent_id.map(|id| decode_node_id(&id).unwrap().1),
            author_id: "anonymous".to_string(),
            content,
        };

        let entity = write_comment(conn, new)?;

        Ok(entity.into())
    }

    async fn delete_comment(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> async_graphql::Result<CommentNode> {
        let conn = &mut ctx.data::<crate::DBPool>()?.get()?;

        let (typename, id) = decode_node_id(&id)?;

        if typename != "Comment" {
            return Err("Invalid ID".into());
        }

        let entity: Comment = delete_comment(conn, id)?;
        Ok(entity.into())
    }
}
