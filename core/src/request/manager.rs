use scraper::{ Html, Selector };
use simple_concurrent_get;

use crate::{
    config,
    route::system::pair::{ SystemPair },
};



pub struct RequestManager {
    concurrent: usize,
    selectors: [Selector; 4],
}

impl RequestManager {
    pub fn new(concurrent: usize) -> color_eyre::Result<Self> {
        Ok(Self {
            concurrent,
            selectors: [
                Selector::parse(r#"div[id="navtools"]"#).unwrap(),
                Selector::parse(r#"table[class="tablelist table-tooltip"]"#).unwrap(),
                Selector::parse(r#"tr"#).unwrap(),
                Selector::parse(r#"td"#).unwrap(),
            ],
        })
    }

    pub fn get(&self, pairs: impl Iterator<Item=SystemPair>) -> std::iter::Zip<SystemPair, u64> {
        let pair_urls = pairs
            .iter()
            .map(|pair| {
                format!("{}{}{}:{}{}",
                    config::ROUTE_SEARCH_URL_PREFIX,
                    self.args().route_option.as_url(),
                    pair.left().read().unwrap().name(),
                    pair.right().read().unwrap().name(),
                    config::ROUTE_SEARCH_URL_SUFFIX,
                )
            });

        // distance is ordered as pairs' order
        let distances = simple_concurrent_get::ordered::concurrent_get(pair_urls, self.concurrent)
            .into_iter()
            .map(|result| match result {
                Err(e) => return,
                Ok(response) => {
                    let text: String = futures_executor::block_on(|| async { response.text().await.unwrap() });

                    self.parse_text_into_length(&text)
                }
            });

        // So, Zipping it doesn't mess up pair and distance
        std::iter::zip(
            pairs,
            distances,
        )
    }

    fn parse_text_into_length(&self, text: &String) -> u64 {
        let distance: u64 = Html::parse_document(text)
            .select(self.selectors[0])
            .next()
            .expect("Unexpected response format")
            .select(self.selectors[1])
            .next()
            .expect("System Name Invalid")
            .select(self.selectors[3])
            .last()
            .unwrap()
            .select(self.selectors[4])
            .next()
            .unwrap()
            .inner_html()
            .replace('.', "")
            .trim()
            .parse()
            .expect("Failed to parse route length");

        distance - 1 // route start from self
    }
}
