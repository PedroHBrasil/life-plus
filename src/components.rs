use dioxus::prelude::*;
// Define the Hackernews types
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            width: "100%",
            div {
                width: "50%",
                Stories {}
            }
            div {
                width: "50%",
                Preview {}
            }
        }
    })
}


// New
fn Stories(cx: Scope) -> Element {
    render! {
        StoryListing {
            story: StoryItem {
                id: 0,
                title: "hello hackernews".to_string(),
                url: None,
                text: None,
                by: "Author".to_string(),
                score: 0,
                descendants: 0,
                time: chrono::Utc::now(),
                kids: vec![],
                r#type: "".to_string(),
            }
        }
    }
}

// New
#[derive(Clone, Debug)]
enum PreviewState {
    Unset,
    Loading,
    Loaded(StoryPageData),
}

// New
fn Preview(cx: Scope) -> Element {
    let preview_state = PreviewState::Unset;
    match preview_state {
        PreviewState::Unset => render! {
            "Hover over a story to preview it here"
        },
        PreviewState::Loading => render! {
            "Loading..."
        },
        PreviewState::Loaded(story) => {
            let title = &story.item.title;
            let url = story.item.url.as_deref().unwrap_or_default();
            let text = story.item.text.as_deref().unwrap_or_default();
            render! {
                div {
                    padding: "0.5rem",
                    div {
                        font_size: "1.5rem",
                        a {
                            href: "{url}",
                            "{title}"
                        }
                    }
                    div {
                        dangerous_inner_html: "{text}",
                    }
                    for comment in &story.comments {
                        Comment { comment: comment.clone() }
                    }
                }
            }
        }
    }
}

#[inline_props]
fn Comment(cx: Scope, comment: Comment) -> Element<'a> {
    render! {
        div {
            padding: "0.5rem",
            div {
                color: "gray",
                "by {comment.by}"
            }
            div {
                dangerous_inner_html: "{comment.text}"
            }
            for kid in &comment.sub_comments {
                Comment { comment: kid.clone() }
            }
        }
    }
}

#[inline_props]
fn StoryListing(cx: Scope, story: StoryItem) -> Element {
    let StoryItem {
        title,
        url,
        by,
        score,
        time,
        kids,
        id,
        ..
    } = story;

    let url = url.as_deref().unwrap_or_default();
    let hostname = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");
    let score = format!("{score} {}", if *score == 1 { " point" } else { " points" });
    let comments = format!(
        "{} {}",
        kids.len(),
        if kids.len() == 1 {
            " comment"
        } else {
            " comments"
        }
    );
    let time = time.format("%D %l:%M %p");

    cx.render(rsx! {
        div {
            padding: "0.5rem",
            position: "relative",
            div {
                font_size: "1.5rem",
                a {
                    href: url,
                    "{title}"
                }
                a {
                    color: "gray",
                    href: "https://news.ycombinator.com/from?site={hostname}",
                    text_decoration: "none",
                    " ({hostname})"
                }
            }
            div {
                display: "flex",
                flex_direction: "row",
                color: "gray",
                div {
                    "{score}"
                }
                div {
                    padding_left: "0.5rem",
                    "by {by}"
                }
                div {
                    padding_left: "0.5rem",
                    "{time}"
                }
                div {
                    padding_left: "0.5rem",
                    "{comments}"
                }
            }
        }
    })
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryPageData {
    #[serde(flatten)]
    pub item: StoryItem,
    #[serde(default)]
    pub comments: Vec<Comment>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub id: i64,
    /// there will be no by field if the comment was deleted
    #[serde(default)]
    pub by: String,
    #[serde(default)]
    pub text: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    #[serde(default)]
    pub kids: Vec<i64>,
    #[serde(default)]
    pub sub_comments: Vec<Comment>,
    pub r#type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryItem {
    pub id: i64,
    pub title: String,
    pub url: Option<String>,
    pub text: Option<String>,
    #[serde(default)]
    pub by: String,
    #[serde(default)]
    pub score: i64,
    #[serde(default)]
    pub descendants: i64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    #[serde(default)]
    pub kids: Vec<i64>,
    pub r#type: String,
}