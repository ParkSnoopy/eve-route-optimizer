use itertools::Itertools;

use crate::{
    config,
    state::GlobalState,
    types::{ Route, RouteOption },
};

use scraper::{ Html, Selector };



/*fn _url_to_route(url: &String) -> Route {
    url.trim_start_matches(config::ROUTE_SEARCH_URL_PREFIX).trim_end_matches(config::ROUTE_SEARCH_URL_POSTFIX).split(':').map(|s| s.to_owned()).collect()
}*/

fn route_to_url(route_option: &RouteOption, route: &Route) -> String {
    format!("{}{}{}{}",
        config::ROUTE_SEARCH_URL_PREFIX,
        match route_option {
            RouteOption::Fastest => "",
            RouteOption::Highsec => "2:",
            RouteOption::LowNull => "3:",
        },
        route.iter().join(":"),
        config::ROUTE_SEARCH_URL_POSTFIX,
    )
}

fn paint_yellow(s: &str) -> String {
    nu_ansi_term::Color::Yellow.paint(s).to_string()
}

pub fn parse_text_into_length(text: String) -> usize {
    let selector_00 = Selector::parse(r#"div[id="navtools"]"#).unwrap();
    let selector_01 = Selector::parse(r#"table[class="tablelist table-tooltip"]"#).unwrap();
    let selector_02 = Selector::parse(r#"tr"#).unwrap();
    let selector_03 = Selector::parse(r#"td"#).unwrap();

    Html::parse_document(&text)
        .select(&selector_00)
        .next()
        .expect(&paint_yellow("[ ERR ] Couldn't find `NAVTOOLS` div"))
        .select(&selector_01)
        .next()
        .expect(&paint_yellow("[ ERR ] Couldn't find Route Table"))
        .select(&selector_02)
        .last()
        .expect(&paint_yellow("[ ERR ] Table is empty"))
        .select(&selector_03)
        .next()
        .expect(&paint_yellow("[ ERR ] `TR` has no `TD`"))
        .inner_html()
        .replace('.', "")
        .trim()
        .parse()
        .expect(&paint_yellow("[ ERR ] Failed to parse route length"))
}

pub fn get_term_width() -> usize {
    match term_size::dimensions() {
        Some((width, _height)) => width,
        _ => 0,
    }
}

pub async fn get_route_length(global_state: &GlobalState, route: &Route) -> anyhow::Result<(usize, Route)> {
    let url = route_to_url(&global_state.cli_args.route_option, route);
    //dbg!(&url);

    let resp = global_state.req_client.get(&url).send().await?;
    let text = resp.text().await?;
    //println!("  [ ERR ]  while fetching '{url}' got '{text}'");

    let route_length = parse_text_into_length(text);

    //dbg!(&route, route_length);

    Ok(
        (
            route_length,
            route.to_vec(),
        )
    )
}
