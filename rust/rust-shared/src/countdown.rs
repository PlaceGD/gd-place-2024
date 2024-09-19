use binrw::binrw;

use crate::gd::object::GDObject;

pub const DIGIT_WIDTH: f32 = 30.0 * 10.0;
pub const DIGIT_HEIGHT: f32 = 30.0 * 15.0;
pub const DIGIT_SETS: usize = 3;

#[binrw]
#[brw(little)]
pub struct DigitLayerObjects {
    #[bw(calc(objs.len() as u32))]
    count: u32,
    #[br(count = count)]
    pub objs: Vec<GDObject>,
}

#[binrw]
#[brw(little)]
pub struct DigitLayers {
    #[bw(calc(layers.len() as u32))]
    count: u32,
    #[br(count = count)]
    pub layers: Vec<DigitLayerObjects>,
}

#[binrw]
#[brw(little)]
pub struct DigitSet(pub [DigitLayers; 10]);

#[binrw]
#[brw(little)]
pub struct CountdownDigitSets(pub [DigitSet; DIGIT_SETS]);
