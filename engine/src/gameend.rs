
#[derive(Debug)]
pub enum GameEnd {
  WhiteWon(String),
  BlackWon(String),
  Draw(String)
}