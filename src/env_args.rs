use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EnvArgs {
    pub database_url: String,
}
