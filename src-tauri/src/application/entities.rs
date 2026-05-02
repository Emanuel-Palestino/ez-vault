use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EnvironmentCreate {
    pub name: String,
    pub note: String,
}
