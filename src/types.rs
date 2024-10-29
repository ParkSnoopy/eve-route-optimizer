
pub type Route = Vec<String>;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum RouteOption {
    Fastest,
    Highsec,
    LowNull,
}
