use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize)]
pub struct GameInfos {
    pub playername: u32,
    pub chars: u32,
    pub games_played: u32,
    pub wins: u32,
    pub loses: u32,
}

#[derive(Debug, Serialize)]
pub struct PlayerData {
    pub id: u32,
    pub playername: String,
}

#[derive(Debug, Serialize)]
pub struct FighterData {
    pub id: u32,
    pub fighter_name: String,
}
