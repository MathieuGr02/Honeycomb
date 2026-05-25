use clap::Args;
use serde::Serialize;

use crate::{
    traits::ToJsonBody,
    types::{image::Image, name::Name},
};

#[derive(Debug, Args, Serialize)]
pub struct RunArguments {
    #[serde(flatten)]
    #[arg(short, long, value_parser = Name::from_str, required = false)]
    name: Name,
    #[serde(flatten)]
    #[arg(short, long, value_parser = Image::from_str, required = true)]
    image: Image,
    #[arg(short, long, required = false, default_value = "false")]
    detach: bool,
    //#[arg(short, long, required = false)]
    //attach_stderr: bool,
    //#[arg(short, long, required = true)]
    //attach_stdin: bool,
    //#[arg(short, long, required = true)]
    //attach_stdout: bool,
}

impl ToJsonBody for RunArguments {
    fn to_json_body(&self) -> Result<String, String> {
        return Ok(serde_json::to_string(self).unwrap());
    }
}
