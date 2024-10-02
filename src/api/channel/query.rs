use async_graphql::*;

#[derive(Default)]
pub struct ChannelQuery;

#[Object]
impl ChannelQuery {
    async fn dummy(&self) -> async_graphql::Result<String> {
        Ok("dummy".to_string())
    }
    // async fn channel_count(&self, ctx: &Context<'_>) -> async_graphql::Result<i64> {
    //     use crate::schema::channels::dsl::*;

    //     let conn = &mut ctx.data::<crate::DBPool>()?.get()?;
    //     let len = channels.count().get_result(conn)?;

    //     Ok(len)
    // }
}
