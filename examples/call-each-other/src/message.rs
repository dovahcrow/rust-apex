#[derive(Serialize, Deserialize)]
pub enum Message {
    Job { name: String },
    Joy { time: usize },
}
