use crate::global;
use crate::util::global::Global;
use serde::Deserialize;
use ts_rs::TS;

#[derive(Clone, Debug, PartialEq, Deserialize, TS)]
#[ts(export)]
pub struct Credentials {
  pub username: String,
  pub password: String,
}

static CREDENTIALS: Global<Option<Credentials>> = global!(None);

pub fn set_credentials(credentials: &Credentials) {
  CREDENTIALS.set(Some(credentials.clone()))
}
