use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct EnvArgs {
    database_url: String,
}
