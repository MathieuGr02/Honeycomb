pub trait ToJsonBody {
    fn to_json_body(&self) -> Result<String, String>;
}
