use serde::{Serialize, Deserialize};
/**The result of the statement refresh extraction

`SUCCESS`: The statements were successfully extracted and can be listed via `/statements/list/` and downloaded via `/statements/download/`.

`FAILURE`: The statements failed to be extracted.*/
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum StatementsRefreshCompleteResult {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILURE")]
    Failure,
}
