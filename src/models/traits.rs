use serde::{Deserialize, Serialize};
pub trait Model: Serialize + Deserialize<'static> {
    #[allow(unused)]
    fn id(&self) -> &str;
    fn display_name() -> &'static str;
    fn headers() -> Vec<&'static str>;
    fn to_row(&self) -> Vec<String>;
}
