use anyhow::Result;
use futures_util::Stream;

use crate::types::{OuterResponse, PublishedFileDetails};

pub mod types;

pub fn query_all_files(
    client: reqwest::Client,
    api_key: String,
    app_id: u32,
) -> impl Stream<Item = Result<Vec<PublishedFileDetails>>> {
    ez_stream::try_unbounded(move |tx| async move {
        let mut how_many_more = None;
        let mut cursor = "*".to_string();
        loop {
            let resp = client
                .get("https://api.steampowered.com/IPublishedFileService/QueryFiles/v1")
                .query(&[
                    ("key", api_key.as_str()),
                    ("appid", app_id.to_string().as_str()),
                    ("query_type", "1"),
                    ("return_details", "1"),
                    ("return_vote_data", "1"),
                    ("numperpage", "100"),
                    ("cursor", &cursor),
                ])
                .send()
                .await?;
            let json: OuterResponse = resp.json().await?;

            if how_many_more.is_none() {
                how_many_more = Some(json.response.total);
            }

            match json.response.published_file_details {
                Some(details) if !details.is_empty() => {
                    *how_many_more.as_mut().unwrap() -= i64::try_from(details.len()).unwrap();
                    tx.send(details)?;
                }
                _ => break,
            }

            match json.response.next_cursor {
                Some(x) => {
                    cursor = x;
                }
                None => break,
            }

            if how_many_more.unwrap() <= 0 {
                break;
            }
        }

        Ok(())
    })
}
