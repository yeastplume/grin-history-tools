use structopt::StructOpt;

/// Juniper (GraphQl API), Diesel PostgreSQL, session authentication and JWT boilerplate server
#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "grin_seed_check")]
pub(crate) struct Opt {
    /// Database URL
    #[structopt(long, env = "DATABASE_URL")]
    pub database_url: String,
}
