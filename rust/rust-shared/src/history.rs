use binrw::binrw;

use crate::gd::object::GDObject;

#[binrw]
#[brw(little)]
pub struct History {
    #[bw(calc(actions.len() as u32))]
    count: u32,
    #[br(count = count)]
    pub actions: Vec<HistoryItem>,
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct HistoryItem {
    pub obj: [u8; 26],
    pub objkey: [u8; 20],
    pub time: u32,
    pub username: [u8; 20],
}

impl HistoryItem {
    pub fn time(&self) -> u32 {
        self.time
    }
}
