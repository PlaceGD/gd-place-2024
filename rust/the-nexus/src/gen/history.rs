use std::io::Cursor;

use binrw::BinWrite;
use rust_shared::history::{History, HistoryItem};

use crate::dbconvert::decode_string;

pub fn make_get_history_fn() -> Vec<u8> {
    // read db.json

    let data = std::fs::read_to_string("C:/p/db-convert-2024/db2.json");
    let db: serde_json::Value = serde_json::from_str(&data.unwrap()).unwrap();

    println!(
        "{:?}",
        db.as_object()
            .unwrap()
            .keys()
            .map(|a| a.to_string())
            .collect::<Vec<String>>()
    ); // ["activeDonations", "announcement", "bannedUsers", "claimedDonations", "history", "levelName", "metaVariables", "objectCount", "objects", "reportedUsers", "totalObjectsDeleted", "totalObjectsPlaced", "userCount", "userDetails", "userName", "userPlaced"]

    let mut actions = Vec::new();

    for item in db["history"].as_object().unwrap().values() {
        if let Some(obj) = item.get("object") {
            actions.push(HistoryItem {
                obj: decode_string(obj.as_str().unwrap(), 126)
                    .try_into()
                    .unwrap(),
                objkey: item["objKey"]
                    .as_str()
                    .unwrap()
                    .as_bytes()
                    .try_into()
                    .unwrap(),
                time: item["time"].as_u64().unwrap() as u32,
                username: encode_username(item["username"].as_str().unwrap()),
            })
        } else {
            actions.push(HistoryItem {
                obj: [0; 26],
                objkey: item["objKey"]
                    .as_str()
                    .unwrap()
                    .as_bytes()
                    .try_into()
                    .unwrap(),
                time: item["time"].as_u64().unwrap() as u32,
                username: encode_username(item["username"].as_str().unwrap()),
            })
        }
    }

    actions.sort_by_key(|a| a.time());

    let mut writer = Cursor::new(Vec::new());
    History { actions: actions }.write(&mut writer).unwrap();
    writer.into_inner()
}

fn encode_username(username: &str) -> [u8; 20] {
    let mut bytes = [0; 20];
    let encoded = username.as_bytes();
    let len = encoded.len().min(20);
    bytes[..len].copy_from_slice(&encoded[..len]);
    bytes
}
