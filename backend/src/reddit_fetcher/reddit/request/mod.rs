#![allow(dead_code)]

use params::FeedSorting;

use super::model::MoreComments;
pub mod params;
mod tests;

/// Represents a request to the Reddit API.
///
/// Used to fetch posts, comments, and user/subreddit information.
/// The request is then converted into an HTTP request parts tuple.
///
/// # Example
///
/// ```
/// use params::{FeedSorting, FeedSortingTime};
///
/// let req = SubredditPosts {
///     subreddit: "Polska".to_string(),
///     sorting: FeedSorting::New
/// };

/// let (url, query) = req.into_http_request_parts();

/// assert_eq!(url, "https://oauth.reddit.com/r/Polska/new.json");
/// assert_eq!(query, vec![("limit", "100".to_string()), ("sort", "new".to_string())]);
/// ```
#[derive(Debug)]
pub struct SubredditPostsRequest {
    /// The subreddit name.
    pub subreddit: String,
    pub sorting: FeedSorting,
    pub after: Option<String>,
}

/// Fetch information about a subreddit.
#[derive(Debug)]
pub struct SubredditAboutRequest {
    pub subreddit: String,
}

/// Fetch posts from a user's profile.
#[derive(Debug)]
pub struct UserPostsRequest {
    /// The user's username.
    pub username: String,
    pub sorting: FeedSorting,
    pub after: Option<String>,
}

/// Fetch information about a user.
#[derive(Debug)]
pub struct UserAboutRequest {
    /// The user's username.
    pub username: String,
}

/// Fetch comments from a post.
#[derive(Debug)]
pub struct PostCommentsRequest {
    /// The subreddit name.
    pub subreddit: String,
    /// The post's ID, eg. 1eubxgg
    pub post_id: String,
    pub sorting: FeedSorting,
    pub after: Option<String>,
}

/// The parts of an HTTP request: URL and query parameters.
pub type RequestParts = (String, Vec<(&'static str, String)>);

pub trait RedditRequest {
    fn to_request_parts(&self) -> RequestParts;
    fn resource_name(&self) -> String;
}

impl RedditRequest for SubredditPostsRequest {
    fn to_request_parts(&self) -> RequestParts {
        let url = format!(
            "https://oauth.reddit.com/r/{}/{}.json",
            self.subreddit,
            self.sorting.to_string()
        );

        let mut query = vec![];
        if let Some(time) = self.sorting.time() {
            query.push(("t", time.to_string()));
        }
        query.push(("limit", "100".to_string()));

        if let Some(after) = &self.after {
            query.push(("after", after.to_string()));
        }

        (url, query)
    }
    fn resource_name(&self) -> String {
        format!("r/{}", self.subreddit)
    }
}

impl RedditRequest for SubredditAboutRequest {
    fn to_request_parts(&self) -> RequestParts {
        let url = format!("https://oauth.reddit.com/r/{}/about.json", self.subreddit);
        (url, vec![])
    }
    fn resource_name(&self) -> String {
        format!("r/{}", self.subreddit)
    }
}

impl RedditRequest for UserPostsRequest {
    fn to_request_parts(&self) -> RequestParts {
        let url = format!("https://oauth.reddit.com/user/{}.json", self.username);

        let mut query = vec![("sort", self.sorting.to_string())];
        if let Some(time) = self.sorting.time() {
            query.push(("t", time.to_string()));
        }
        query.push(("limit", "100".to_string()));

        if let Some(after) = &self.after {
            query.push(("after", after.to_string()));
        }

        (url, query)
    }
    fn resource_name(&self) -> String {
        format!("u/{}", self.username)
    }
}

impl RedditRequest for UserAboutRequest {
    fn to_request_parts(&self) -> RequestParts {
        let url = format!("https://oauth.reddit.com/user/{}/about.json", self.username);
        (url, vec![])
    }
    fn resource_name(&self) -> String {
        format!("u/{}", self.username)
    }
}

impl RedditRequest for PostCommentsRequest {
    fn to_request_parts(&self) -> RequestParts {
        let url = format!(
            "https://oauth.reddit.com/r/{}/comments/{}.json",
            self.subreddit, self.post_id
        );

        let mut query = vec![("sort", self.sorting.to_string())];
        if let Some(time) = self.sorting.time() {
            query.push(("t", time.to_string()));
        }
        query.push(("limit", "100".to_string()));

        if let Some(after) = &self.after {
            query.push(("after", after.to_string()));
        }

        (url, query)
    }
    fn resource_name(&self) -> String {
        format!("r/{}/comments/{}", self.subreddit, self.post_id)
    }
}

impl MoreComments {
    pub fn into_request_parts(self) -> Vec<RequestParts> {
        let url = "https://oauth.reddit.com/api/morechildren".to_string();

        self.children
            .chunks(100)
            .map(|chunk| {
                let query = vec![
                    ("link_id", self.parent_id.clone()),
                    ("children", chunk.join(",")),
                    ("api_type", "json".to_string()),
                ];

                (url.clone(), query)
            })
            .collect()
    }
}
