use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "enormous-server", about = "Enormous Cloud server side")]
pub struct Args {
    #[structopt(
        short,
        long,
        default_value = "postgres://postgres:password@localhost/enormous",
        env = "DATABASE_URL"
    )]
    pub database_url: String,
    #[structopt(long, default_value = "5", env = "DATABASE_MAX_CONN")]
    pub database_conn: u32,
    #[structopt(long, default_value = "./dist", env = "STATIC_DIR")]
    pub static_dir: String,
    #[structopt(short, long, default_value = "0.0.0.0:8000", env = "LISTEN")]
    pub addr: String,
}

pub fn parse() -> anyhow::Result<Args> {
    let res = Args::from_args();
    tracing::info!("{:?}", res);
    // todo: check static dir exists
    Ok(res)
}
