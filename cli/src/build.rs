use clap::Args;
use serde::{Deserialize, Serialize};

use crate::{
    traits::ToJsonBody,
    types::{image::Image, name::Name},
};

#[derive(Debug, Args, Serialize)]
pub struct BuildArguments {
    #[serde(flatten)]
    #[arg(short, long, value_parser=Name::from_str, required=false)]
    name: Name,
    #[serde(flatten)]
    #[arg(short, long, value_parser=Image::from_str, required=true)]
    image: Image,
}

impl ToJsonBody for BuildArguments {
    fn to_json_body(&self) -> Result<String, String> {
        return Ok(serde_json::to_string(self).unwrap());
    }
}
