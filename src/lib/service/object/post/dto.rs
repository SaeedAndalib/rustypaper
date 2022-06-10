use crate::domain::post::field::{Content, IsPublished, Shortcode, Title};
use crate::service::ServiceError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePostObject {
    pub title: Title,
    pub content: Content,
    pub is_published: IsPublished,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAllPostsObject {
    pub skip: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetPostObject {
    pub shortcode: Shortcode,
}

impl FromStr for GetPostObject {
    type Err = ServiceError;

    fn from_str(shortcode: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            shortcode: shortcode.parse()?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePostObject {
    pub title: Option<Title>,
    pub content: Option<Content>,
    pub shortcode: Shortcode,
    pub is_published: Option<IsPublished>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeletePostObject {
    pub shortcode: Shortcode,
}
