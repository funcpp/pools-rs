mod util;

use crate::{pg, DateTime};

// Models
mod channel;
use channel::{prelude::*, TYPENAME as NODE_CHANNEL};

pub mod comment;
use comment::{prelude::*, TYPENAME as NODE_COMMENT};

pub mod post;
use post::{prelude::*, TYPENAME as NODE_POST};

use chrono::TimeZone;

// GraphQL schema

use async_graphql::{
    Context, EmptySubscription, Interface, MergedObject, Object, Result, Schema, ID,
};

// GraphQL Node

#[derive(Interface)]
#[graphql(field(name = "id", ty = "ID"))]
enum Node {
    Channel(ChannelNode),
    Post(PostNode),
    Comment(CommentNode),
}

use crate::DBPool;
use diesel::prelude::*;

#[derive(Default)]
struct BaseQuery;

async fn get_node(conn: &mut PgConnection, node_id: ID) -> Result<Node> {
    let (typename, eid) = util::decode_node_id(&node_id)?;
    let typename = typename.as_str();

    match typename {
        NODE_CHANNEL => Ok(Node::Channel(pg::channel::get_by_id(conn, eid)?.into())),
        NODE_POST => Ok(Node::Post(pg::post::get_by_id(conn, eid)?.into())),
        NODE_COMMENT => Ok(Node::Comment(pg::comment::get_by_id(conn, eid)?.into())),
        _ => Err("Unknown type".into()),
    }
}

#[Object]
impl BaseQuery {
    async fn built_at(&self) -> DateTime {
        let built_at = option_env!("BUILT_AT").unwrap_or("Unknown");
        let built_at = built_at.parse::<i64>().unwrap_or(0);
        chrono::Utc.timestamp_opt(built_at, 0).single().unwrap()
    }

    async fn node(&self, ctx: &Context<'_>, id: ID) -> Result<Node> {
        let conn = &mut ctx.data::<DBPool>()?.get()?;

        get_node(conn, id).await
    }

    async fn nodes(&self, ctx: &Context<'_>, ids: Vec<ID>) -> Result<Vec<Node>> {
        let conn = &mut ctx.data::<DBPool>()?.get()?;

        let mut nodes = Vec::new();
        for node_id in ids {
            nodes.push(get_node(conn, node_id).await?);
        }

        Ok(nodes)
    }
}

// export GQLs...

#[derive(MergedObject, Default)]
pub struct Query(BaseQuery, ChannelQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(ChannelMutation, PostMutation, CommentMutation);

pub type GQLSchema = Schema<Query, Mutation, EmptySubscription>;
