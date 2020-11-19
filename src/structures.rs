/// The format a valid SQLlite thread entry should have.
#[derive(FromForm, Debug, Serialize, Deserialize)]
pub struct Thread {
    pub board: String,
    pub name: String,
    pub comment: String
}

/// The format a valid SQLlite thread entry should have.
#[derive(FromForm, Debug, Serialize, Deserialize, PartialEq)]
pub struct Board {
    pub tag: String,
    pub name: String,
    pub nsfw: i32,
    pub disabled: i32
}
