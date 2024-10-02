use async_graphql::ID;
use base64::{engine::general_purpose::STANDARD as b64, Engine as _};

#[derive(Debug)]
pub struct NodeIdError {
    error: String,
    origin: Option<String>,
}

impl std::fmt::Display for NodeIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(origin) = &self.origin {
            write!(f, "{}: {}", self.error, origin)
        } else {
            write!(f, "{}", self.error)
        }
    }
}

impl From<base64::DecodeError> for NodeIdError {
    fn from(_: base64::DecodeError) -> Self {
        NodeIdError {
            error: "Error occurred while decoding base64".to_string(),
            origin: None,
        }
    }
}

pub fn encode_node_id(typename: &str, id: i32) -> ID {
    ID::from(b64.encode(format!("{}#{}", typename, id)))
}

pub fn decode_node_id(node_id: &ID) -> Result<(String, i32), NodeIdError> {
    let decoded: String = String::from_utf8(b64.decode(node_id.as_str())?).unwrap();
    let spl: Vec<&str> = decoded.split("#").collect::<Vec<&str>>();
    if spl.len() != 2 {
        return Err(NodeIdError {
            error: "Invalid node id".to_string(),
            origin: Some(node_id.to_string()),
        });
    }
    let typename = spl[0].to_string();
    let id = spl[1].parse::<i32>().unwrap();

    Ok((typename, id))
}
