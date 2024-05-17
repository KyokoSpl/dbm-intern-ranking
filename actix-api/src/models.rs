use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Debug)]
pub struct GameInfos {
    pub player_id: u32,
    pub first_fighter_id: u32,
    pub second_fighter_id: u32,
    pub third_fighter_id: u32,
    pub fourth_fighter_id: u32,
    pub fifth_fighter_id: u32,
    pub wins: u32,
    pub loses: u32
}

#[derive(Serialize, Debug)]
pub struct PlayerData {
    pub id: u32,
    pub player_name: String
}

#[derive(Serialize, Debug)]
pub struct FighterData {
    pub id: u32,
    pub fighter_name: String
}
