use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug)]
pub struct GameInfos {
    pub player_id: u64,
    pub fighter_id_1: u32,
    pub wins: u32,
    pub loses: u32,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct DelGame {
    pub id: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PlayerData {
    pub id: u64,
    pub player_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RemovePlayerData {
    pub id: u64,
}

#[derive(Serialize, Debug)]
pub struct FighterData {
    pub id: u32,
    pub fighter_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BaseStatsData {
    pub player_id: u64,
    pub wins: u32,
    pub loses: u32,
}
