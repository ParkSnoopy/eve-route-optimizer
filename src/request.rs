use std::sync::LazyLock;

use scraper::{
    Html,
    Selector,
};

use crate::{
    config,
    route::RouteOption,
    system::SystemPair,
    trace,
};



fn avoid_to_embed() -> String {
    let mut s = String::new();
    match &crate::CLI_ARGS.read().unwrap().avoid {
        None => {},
        Some(unord_route) => {

            for system in (&unord_route).into_iter() {
                s += &format!(":-{}", system.name());
            }
        }
    }
    s
}

pub fn make_url(system_pair: &SystemPair) -> String {
    format!(
        "{}{}{}:{}{}",
        config::ROUTE_SEARCH_URL_PREFIX,
        match crate::CLI_ARGS.read().unwrap().route_option {
            RouteOption::Fastest => "",
            RouteOption::Highsec => "2:",
            RouteOption::LowNull => "3:",
        },
        system_pair.left().read().unwrap().name(),
        system_pair.right().read().unwrap().name(),
        avoid_to_embed(),
    )
}

static SEL_0: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(r#"div[id="navtools"]"#).unwrap());
static SEL_1: LazyLock<Selector> =
    LazyLock::new(|| Selector::parse(r#"table[class="tablelist table-tooltip"]"#).unwrap());
static SEL_2: LazyLock<Selector> = LazyLock::new(|| Selector::parse(r#"tr"#).unwrap());
static SEL_3: LazyLock<Selector> = LazyLock::new(|| Selector::parse(r#"td"#).unwrap());

pub fn parse_text_into_length(text: &String) -> u64 {
    //trace::debug(format!("Parsing text from request: {}", &text[..20]));
    let distance: u64 = Html::parse_document(text)
        .select(&SEL_0)
        .next()
        .expect(&trace::string::error("Unexpected response format"))
        .select(&SEL_1)
        .next()
        .expect(&trace::string::error("System Name Invalid"))
        .select(&SEL_2)
        .last()
        .unwrap()
        .select(&SEL_3)
        .next()
        .unwrap()
        .inner_html()
        .replace('.', "")
        .trim()
        .parse()
        .expect(&trace::string::error("Failed to parse route length"));

    distance - 1 // route start from self
}
