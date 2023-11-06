
use the_nexus::objects::{ObjectCategory::*, ObjectInfo};
use the_nexus::packing::SpriteInfo;


pub fn get_object_info(id: u32) -> Option<ObjectInfo> {
    Some(match id {
        1700 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Utilities,
},227 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -4.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},470 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},1734 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},224 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},223 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1607 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},155 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},218 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -6.0,
    tintable: true,
    solid: false,
    category: Spikes,
},1051 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -2.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},107 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 4.0,
    tintable: true,
    solid: false,
    category: Deco,
},1753 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},477 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},914 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1862 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},2 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Blocks,
},47 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},694 => ObjectInfo {
    place_offset_x: 15.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},1600 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},505 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},1519 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Utilities,
},86 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},17 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -8.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},659 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},642 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},928 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},641 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},479 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1205 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},919 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -10.0,
    tintable: true,
    solid: false,
    category: Utilities,
},1133 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1867 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1728 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -7.5,
    tintable: true,
    solid: false,
    category: Spikes,
},186 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1267 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1004 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},406 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -8.0,
    tintable: true,
    solid: false,
    category: Deco,
},158 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -1.5,
    tintable: true,
    solid: false,
    category: GroundDeco,
},143 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},1833 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1834 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},7 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Blocks,
},1273 => ObjectInfo {
    place_offset_x: 5.0,
    place_offset_y: -5.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},1266 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},149 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},719 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -7.5,
    tintable: true,
    solid: false,
    category: Deco,
},878 => ObjectInfo {
    place_offset_x: 15.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},1707 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1765 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},696 => ObjectInfo {
    place_offset_x: 15.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},106 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 18.0,
    tintable: true,
    solid: false,
    category: Deco,
},1869 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},177 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Spikes,
},1022 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},721 => ObjectInfo {
    place_offset_x: -11.5,
    place_offset_y: -11.5,
    tintable: true,
    solid: false,
    category: Deco,
},474 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},1721 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -11.0,
    tintable: true,
    solid: false,
    category: Spikes,
},183 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1279 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1134 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},695 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},1003 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1846 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1764 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},179 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -6.0,
    tintable: true,
    solid: false,
    category: Spikes,
},1885 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1870 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},136 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},1889 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Spikes,
},648 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1338 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},222 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},895 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},1859 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},1741 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1874 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},409 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},41 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 20.0,
    tintable: true,
    solid: false,
    category: Deco,
},504 => ObjectInfo {
    place_offset_x: 5.0,
    place_offset_y: -5.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},1203 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},1715 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -12.5,
    tintable: true,
    solid: false,
    category: Spikes,
},480 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},646 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1274 => ObjectInfo {
    place_offset_x: 5.0,
    place_offset_y: -5.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},472 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},35 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -13.0,
    tintable: false,
    solid: false,
    category: Utilities,
},115 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -5.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},471 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},909 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -7.5,
    tintable: true,
    solid: false,
    category: Deco,
},414 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -9.0,
    tintable: true,
    solid: false,
    category: Deco,
},1606 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1277 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1058 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1831 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},931 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1751 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},1278 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1602 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},408 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -12.5,
    tintable: true,
    solid: false,
    category: Deco,
},185 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1002 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},4 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Blocks,
},932 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Blocks,
},937 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},700 => ObjectInfo {
    place_offset_x: 15.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},154 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1697 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Utilities,
},141 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},3 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Blocks,
},1718 => ObjectInfo {
    place_offset_x: 15.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Spikes,
},1724 => ObjectInfo {
    place_offset_x: 15.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Spikes,
},929 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},139 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},739 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},395 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},467 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},20 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -2.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},67 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -12.0,
    tintable: false,
    solid: false,
    category: Utilities,
},1829 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},1207 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},11 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},99 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},150 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},647 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},148 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},1710 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1875 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},935 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1861 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1699 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Utilities,
},1888 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},54 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},1722 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -11.0,
    tintable: true,
    solid: false,
    category: Spikes,
},1882 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},476 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1759 => ObjectInfo {
    place_offset_x: 10.5,
    place_offset_y: 9.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},1332 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -12.5,
    tintable: false,
    solid: false,
    category: Utilities,
},1618 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Utilities,
},132 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},1736 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},211 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},188 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1708 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},394 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1596 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1878 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},469 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},907 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -4.5,
    tintable: true,
    solid: false,
    category: Deco,
},933 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},701 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},411 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1333 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},420 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -2.5,
    tintable: true,
    solid: false,
    category: GroundDeco,
},1019 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1584 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Utilities,
},101 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},1619 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},678 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1758 => ObjectInfo {
    place_offset_x: -7.25,
    place_offset_y: 7.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},649 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},216 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Spikes,
},494 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},699 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},49 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -2.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},1720 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -11.0,
    tintable: true,
    solid: false,
    category: Spikes,
},135 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -11.0,
    tintable: true,
    solid: false,
    category: Spikes,
},881 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1204 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},874 => ObjectInfo {
    place_offset_x: -7.5,
    place_offset_y: -7.5,
    tintable: true,
    solid: false,
    category: Blocks,
},1247 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1054 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -7.5,
    tintable: true,
    solid: false,
    category: Utilities,
},13 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},60 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},156 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},650 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},873 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 7.5,
    tintable: true,
    solid: false,
    category: Blocks,
},1330 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},84 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},1586 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Utilities,
},877 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},1892 => ObjectInfo {
    place_offset_x: -7.5,
    place_offset_y: -9.75,
    tintable: true,
    solid: false,
    category: Spikes,
},138 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1756 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},745 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},1583 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Utilities,
},680 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1005 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},917 => ObjectInfo {
    place_offset_x: -11.25,
    place_offset_y: -11.25,
    tintable: true,
    solid: false,
    category: Deco,
},643 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1813 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},1755 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},1825 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},242 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},478 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},133 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},53 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},645 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},113 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 1.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},938 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},939 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -6.0,
    tintable: true,
    solid: false,
    category: Deco,
},266 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},644 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},458 => ObjectInfo {
    place_offset_x: -7.5,
    place_offset_y: -9.75,
    tintable: true,
    solid: false,
    category: Spikes,
},882 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1050 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -2.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},410 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},36 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},111 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},1060 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1020 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1847 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},503 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -5.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},1830 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1873 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},5 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1835 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},83 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Blocks,
},1706 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1603 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},6 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Blocks,
},187 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1844 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1876 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1723 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Spikes,
},1604 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1754 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1209 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},1605 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1879 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},660 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},473 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},228 => ObjectInfo {
    place_offset_x: -7.5,
    place_offset_y: -7.5,
    tintable: true,
    solid: false,
    category: GroundDeco,
},396 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1868 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},482 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1871 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},178 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -8.0,
    tintable: true,
    solid: false,
    category: Spikes,
},890 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},140 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -13.0,
    tintable: false,
    solid: false,
    category: Utilities,
},1866 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1880 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1872 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},52 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},1280 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1281 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1890 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -9.0,
    tintable: true,
    solid: false,
    category: Spikes,
},413 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},702 => ObjectInfo {
    place_offset_x: 15.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},85 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1210 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},1865 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},159 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -1.5,
    tintable: true,
    solid: false,
    category: GroundDeco,
},21 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -8.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},1620 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},217 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -9.0,
    tintable: true,
    solid: false,
    category: Spikes,
},883 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},48 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 2.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},1053 => ObjectInfo {
    place_offset_x: -7.5,
    place_offset_y: -7.5,
    tintable: true,
    solid: false,
    category: Utilities,
},419 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -2.5,
    tintable: true,
    solid: false,
    category: GroundDeco,
},871 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1891 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -6.0,
    tintable: true,
    solid: false,
    category: Spikes,
},1597 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1883 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1740 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1843 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1731 => ObjectInfo {
    place_offset_x: -11.5,
    place_offset_y: -11.5,
    tintable: true,
    solid: false,
    category: Spikes,
},16 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -1.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},722 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1021 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},110 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 2.0,
    tintable: true,
    solid: false,
    category: Deco,
},1705 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1730 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -7.5,
    tintable: true,
    solid: false,
    category: Spikes,
},1582 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Utilities,
},1832 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1837 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},896 => ObjectInfo {
    place_offset_x: 15.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},1717 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Spikes,
},1864 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},693 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},1863 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},45 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},1766 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},916 => ObjectInfo {
    place_offset_x: -7.5,
    place_offset_y: -7.5,
    tintable: true,
    solid: false,
    category: Deco,
},1331 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},870 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},19 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 4.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},15 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 6.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},184 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1848 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},658 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},481 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1704 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},1206 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},1735 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},412 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1137 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1877 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},734 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},880 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},934 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},51 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},698 => ObjectInfo {
    place_offset_x: 15.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},1881 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1059 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},46 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},18 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 4.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},1845 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},10 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},1052 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -2.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},930 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},889 => ObjectInfo {
    place_offset_x: 15.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},273 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1768 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},679 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1729 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -7.5,
    tintable: true,
    solid: false,
    category: Spikes,
},259 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},927 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},460 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},1001 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1601 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},87 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1884 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},12 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},697 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},1725 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -9.0,
    tintable: true,
    solid: false,
    category: Spikes,
},1698 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Utilities,
},114 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -2.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},157 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -1.5,
    tintable: true,
    solid: false,
    category: GroundDeco,
},1767 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1709 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},888 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},97 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},918 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Utilities,
},1135 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1757 => ObjectInfo {
    place_offset_x: -7.5,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1202 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 13.5,
    tintable: true,
    solid: true,
    category: Outlines,
},137 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},869 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},50 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},1339 => ObjectInfo {
    place_offset_x: 15.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},908 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -7.5,
    tintable: true,
    solid: false,
    category: Deco,
},1136 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},468 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 14.25,
    tintable: true,
    solid: true,
    category: Outlines,
},
        _ => return None,
    })
}
    


pub fn get_main_sprite(id: u32) -> Option<SpriteInfo> {
    Some(match id {
        467 => SpriteInfo {
    pos: (
        2165,
        2273,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1135 => SpriteInfo {
    pos: (
        462,
        519,
    ),
    size: (
        118,
        74,
    ),
    rotated: false,
    offset: (
        -1.0,
        0.0,
    ),
},919 => SpriteInfo {
    pos: (
        3672,
        1303,
    ),
    size: (
        120,
        74,
    ),
    rotated: false,
    offset: (
        0.0,
        -17.0,
    ),
},1706 => SpriteInfo {
    pos: (
        711,
        1201,
    ),
    size: (
        240,
        240,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},218 => SpriteInfo {
    pos: (
        3293,
        1245,
    ),
    size: (
        80,
        76,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},159 => SpriteInfo {
    pos: (
        3852,
        1334,
    ),
    size: (
        120,
        97,
    ),
    rotated: false,
    offset: (
        0.0,
        2.5,
    ),
},1051 => SpriteInfo {
    pos: (
        2852,
        2178,
    ),
    size: (
        120,
        104,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1054 => SpriteInfo {
    pos: (
        3854,
        1022,
    ),
    size: (
        120,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},419 => SpriteInfo {
    pos: (
        2467,
        0,
    ),
    size: (
        120,
        102,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},148 => SpriteInfo {
    pos: (
        3550,
        156,
    ),
    size: (
        110,
        110,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},53 => SpriteInfo {
    pos: (
        1879,
        839,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},722 => SpriteInfo {
    pos: (
        2728,
        2076,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},888 => SpriteInfo {
    pos: (
        3245,
        2033,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},641 => SpriteInfo {
    pos: (
        125,
        1875,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1136 => SpriteInfo {
    pos: (
        2348,
        0,
    ),
    size: (
        115,
        74,
    ),
    rotated: false,
    offset: (
        0.5,
        0.0,
    ),
},460 => SpriteInfo {
    pos: (
        1275,
        1667,
    ),
    size: (
        114,
        176,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1876 => SpriteInfo {
    pos: (
        2041,
        2081,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},937 => SpriteInfo {
    pos: (
        3102,
        830,
    ),
    size: (
        310,
        163,
    ),
    rotated: false,
    offset: (
        0.0,
        0.5,
    ),
},1868 => SpriteInfo {
    pos: (
        3520,
        394,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1519 => SpriteInfo {
    pos: (
        3372,
        177,
    ),
    size: (
        36,
        37,
    ),
    rotated: false,
    offset: (
        -1.0,
        -1.5,
    ),
},645 => SpriteInfo {
    pos: (
        714,
        2153,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1764 => SpriteInfo {
    pos: (
        0,
        415,
    ),
    size: (
        32,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},482 => SpriteInfo {
    pos: (
        3333,
        706,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1767 => SpriteInfo {
    pos: (
        4053,
        2133,
    ),
    size: (
        32,
        12,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},874 => SpriteInfo {
    pos: (
        3694,
        649,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},158 => SpriteInfo {
    pos: (
        2617,
        332,
    ),
    size: (
        120,
        95,
    ),
    rotated: false,
    offset: (
        0.0,
        4.5,
    ),
},1605 => SpriteInfo {
    pos: (
        1215,
        1189,
    ),
    size: (
        78,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},223 => SpriteInfo {
    pos: (
        2989,
        364,
    ),
    size: (
        228,
        253,
    ),
    rotated: false,
    offset: (
        0.0,
        0.5,
    ),
},135 => SpriteInfo {
    pos: (
        3873,
        2151,
    ),
    size: (
        176,
        221,
    ),
    rotated: false,
    offset: (
        -8.0,
        -58.5,
    ),
},1728 => SpriteInfo {
    pos: (
        124,
        1568,
    ),
    size: (
        121,
        71,
    ),
    rotated: false,
    offset: (
        -0.5,
        -0.5,
    ),
},1021 => SpriteInfo {
    pos: (
        1336,
        295,
    ),
    size: (
        324,
        324,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},21 => SpriteInfo {
    pos: (
        3547,
        772,
    ),
    size: (
        166,
        50,
    ),
    rotated: false,
    offset: (
        -1.0,
        -1.0,
    ),
},739 => SpriteInfo {
    pos: (
        2289,
        2149,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},54 => SpriteInfo {
    pos: (
        3620,
        2167,
    ),
    size: (
        120,
        114,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},934 => SpriteInfo {
    pos: (
        2863,
        2054,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1604 => SpriteInfo {
    pos: (
        2091,
        213,
    ),
    size: (
        77,
        48,
    ),
    rotated: false,
    offset: (
        -0.5,
        0.0,
    ),
},1338 => SpriteInfo {
    pos: (
        3164,
        1245,
    ),
    size: (
        125,
        125,
    ),
    rotated: false,
    offset: (
        1.5,
        1.5,
    ),
},36 => SpriteInfo {
    pos: (
        1793,
        2081,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},101 => SpriteInfo {
    pos: (
        3266,
        1550,
    ),
    size: (
        120,
        355,
    ),
    rotated: false,
    offset: (
        27.0,
        0.5,
    ),
},227 => SpriteInfo {
    pos: (
        2741,
        369,
    ),
    size: (
        120,
        88,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},929 => SpriteInfo {
    pos: (
        2199,
        41,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},458 => SpriteInfo {
    pos: (
        3556,
        908,
    ),
    size: (
        50,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},188 => SpriteInfo {
    pos: (
        2803,
        620,
    ),
    size: (
        168,
        168,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1825 => SpriteInfo {
    pos: (
        1313,
        1089,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},933 => SpriteInfo {
    pos: (
        3040,
        1245,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},658 => SpriteInfo {
    pos: (
        914,
        391,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1708 => SpriteInfo {
    pos: (
        249,
        1977,
    ),
    size: (
        337,
        339,
    ),
    rotated: false,
    offset: (
        -0.5,
        -0.5,
    ),
},473 => SpriteInfo {
    pos: (
        4046,
        0,
    ),
    size: (
        9,
        9,
    ),
    rotated: false,
    offset: (
        -55.5,
        -55.5,
    ),
},150 => SpriteInfo {
    pos: (
        2920,
        898,
    ),
    size: (
        176,
        176,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},97 => SpriteInfo {
    pos: (
        2413,
        2261,
    ),
    size: (
        100,
        102,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},186 => SpriteInfo {
    pos: (
        2906,
        1713,
    ),
    size: (
        335,
        337,
    ),
    rotated: false,
    offset: (
        -0.5,
        -0.5,
    ),
},504 => SpriteInfo {
    pos: (
        3556,
        826,
    ),
    size: (
        78,
        78,
    ),
    rotated: false,
    offset: (
        1.0,
        1.0,
    ),
},406 => SpriteInfo {
    pos: (
        4002,
        905,
    ),
    size: (
        54,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},896 => SpriteInfo {
    pos: (
        873,
        1595,
    ),
    size: (
        240,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},477 => SpriteInfo {
    pos: (
        362,
        673,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1602 => SpriteInfo {
    pos: (
        1664,
        275,
    ),
    size: (
        62,
        62,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},84 => SpriteInfo {
    pos: (
        2283,
        1015,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},699 => SpriteInfo {
    pos: (
        3898,
        2047,
    ),
    size: (
        100,
        100,
    ),
    rotated: false,
    offset: (
        10.0,
        10.0,
    ),
},1861 => SpriteInfo {
    pos: (
        1449,
        747,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1266 => SpriteInfo {
    pos: (
        3426,
        145,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},869 => SpriteInfo {
    pos: (
        3526,
        1942,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1751 => SpriteInfo {
    pos: (
        1421,
        1277,
    ),
    size: (
        136,
        138,
    ),
    rotated: false,
    offset: (
        10.0,
        0.0,
    ),
},650 => SpriteInfo {
    pos: (
        2782,
        1806,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},698 => SpriteInfo {
    pos: (
        3412,
        270,
    ),
    size: (
        240,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1279 => SpriteInfo {
    pos: (
        430,
        0,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},649 => SpriteInfo {
    pos: (
        3100,
        997,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},183 => SpriteInfo {
    pos: (
        2221,
        1806,
    ),
    size: (
        337,
        339,
    ),
    rotated: false,
    offset: (
        -0.5,
        -0.5,
    ),
},7 => SpriteInfo {
    pos: (
        2587,
        431,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},113 => SpriteInfo {
    pos: (
        1709,
        1272,
    ),
    size: (
        498,
        157,
    ),
    rotated: false,
    offset: (
        0.0,
        -1.5,
    ),
},1278 => SpriteInfo {
    pos: (
        3570,
        648,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},11 => SpriteInfo {
    pos: (
        3457,
        826,
    ),
    size: (
        95,
        298,
    ),
    rotated: false,
    offset: (
        21.5,
        4.0,
    ),
},5 => SpriteInfo {
    pos: (
        1207,
        1847,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},481 => SpriteInfo {
    pos: (
        2256,
        1139,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1833 => SpriteInfo {
    pos: (
        2865,
        372,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        -60.0,
        -60.0,
    ),
},132 => SpriteInfo {
    pos: (
        1343,
        157,
    ),
    size: (
        186,
        134,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1870 => SpriteInfo {
    pos: (
        1742,
        2205,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1830 => SpriteInfo {
    pos: (
        4002,
        839,
    ),
    size: (
        62,
        62,
    ),
    rotated: false,
    offset: (
        29.0,
        29.0,
    ),
},1878 => SpriteInfo {
    pos: (
        1403,
        1419,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1877 => SpriteInfo {
    pos: (
        900,
        639,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},479 => SpriteInfo {
    pos: (
        3221,
        425,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},505 => SpriteInfo {
    pos: (
        0,
        1350,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1875 => SpriteInfo {
    pos: (
        3664,
        207,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1736 => SpriteInfo {
    pos: (
        3899,
        1734,
    ),
    size: (
        155,
        155,
    ),
    rotated: false,
    offset: (
        0.5,
        0.5,
    ),
},52 => SpriteInfo {
    pos: (
        2283,
        903,
    ),
    size: (
        120,
        100,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},211 => SpriteInfo {
    pos: (
        3744,
        2246,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},110 => SpriteInfo {
    pos: (
        1579,
        2026,
    ),
    size: (
        77,
        136,
    ),
    rotated: false,
    offset: (
        -0.5,
        0.0,
    ),
},1209 => SpriteInfo {
    pos: (
        666,
        342,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1843 => SpriteInfo {
    pos: (
        90,
        1999,
    ),
    size: (
        120,
        40,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},468 => SpriteInfo {
    pos: (
        584,
        590,
    ),
    size: (
        120,
        6,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1863 => SpriteInfo {
    pos: (
        714,
        2029,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},420 => SpriteInfo {
    pos: (
        2617,
        238,
    ),
    size: (
        120,
        90,
    ),
    rotated: false,
    offset: (
        0.0,
        6.0,
    ),
},1134 => SpriteInfo {
    pos: (
        3293,
        1325,
    ),
    size: (
        103,
        103,
    ),
    rotated: false,
    offset: (
        9.5,
        9.5,
    ),
},939 => SpriteInfo {
    pos: (
        0,
        2205,
    ),
    size: (
        66,
        35,
    ),
    rotated: false,
    offset: (
        0.0,
        17.5,
    ),
},702 => SpriteInfo {
    pos: (
        1664,
        593,
    ),
    size: (
        240,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1731 => SpriteInfo {
    pos: (
        1641,
        1668,
    ),
    size: (
        31,
        30,
    ),
    rotated: false,
    offset: (
        -0.5,
        0.0,
    ),
},696 => SpriteInfo {
    pos: (
        0,
        1019,
    ),
    size: (
        240,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1759 => SpriteInfo {
    pos: (
        955,
        1403,
    ),
    size: (
        275,
        188,
    ),
    rotated: false,
    offset: (
        0.5,
        1.0,
    ),
},41 => SpriteInfo {
    pos: (
        0,
        0,
    ),
    size: (
        77,
        278,
    ),
    rotated: false,
    offset: (
        0.5,
        0.0,
    ),
},99 => SpriteInfo {
    pos: (
        2156,
        767,
    ),
    size: (
        123,
        355,
    ),
    rotated: false,
    offset: (
        26.5,
        0.5,
    ),
},17 => SpriteInfo {
    pos: (
        1821,
        0,
    ),
    size: (
        21,
        50,
    ),
    rotated: false,
    offset: (
        0.5,
        0.0,
    ),
},1133 => SpriteInfo {
    pos: (
        2221,
        1494,
    ),
    size: (
        96,
        96,
    ),
    rotated: false,
    offset: (
        6.0,
        6.0,
    ),
},494 => SpriteInfo {
    pos: (
        3129,
        621,
    ),
    size: (
        88,
        176,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1867 => SpriteInfo {
    pos: (
        1393,
        1791,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},472 => SpriteInfo {
    pos: (
        4068,
        1175,
    ),
    size: (
        6,
        6,
    ),
    rotated: false,
    offset: (
        -57.0,
        -57.0,
    ),
},930 => SpriteInfo {
    pos: (
        2527,
        2179,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1765 => SpriteInfo {
    pos: (
        586,
        281,
    ),
    size: (
        32,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},184 => SpriteInfo {
    pos: (
        3758,
        810,
    ),
    size: (
        240,
        208,
    ),
    rotated: false,
    offset: (
        0.0,
        -34.0,
    ),
},469 => SpriteInfo {
    pos: (
        3209,
        2157,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1729 => SpriteInfo {
    pos: (
        1688,
        2081,
    ),
    size: (
        101,
        71,
    ),
    rotated: false,
    offset: (
        -10.5,
        -0.5,
    ),
},1331 => SpriteInfo {
    pos: (
        3390,
        1818,
    ),
    size: (
        132,
        335,
    ),
    rotated: false,
    offset: (
        23.0,
        0.5,
    ),
},470 => SpriteInfo {
    pos: (
        3661,
        1381,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},266 => SpriteInfo {
    pos: (
        3656,
        331,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1704 => SpriteInfo {
    pos: (
        1158,
        728,
    ),
    size: (
        136,
        138,
    ),
    rotated: false,
    offset: (
        10.0,
        0.0,
    ),
},49 => SpriteInfo {
    pos: (
        641,
        1070,
    ),
    size: (
        320,
        127,
    ),
    rotated: false,
    offset: (
        0.0,
        -0.5,
    ),
},1846 => SpriteInfo {
    pos: (
        847,
        887,
    ),
    size: (
        154,
        172,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},873 => SpriteInfo {
    pos: (
        3774,
        2058,
    ),
    size: (
        120,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},86 => SpriteInfo {
    pos: (
        264,
        1735,
    ),
    size: (
        214,
        217,
    ),
    rotated: false,
    offset: (
        0.0,
        0.5,
    ),
},895 => SpriteInfo {
    pos: (
        1917,
        2162,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1734 => SpriteInfo {
    pos: (
        2335,
        1483,
    ),
    size: (
        319,
        319,
    ),
    rotated: false,
    offset: (
        -0.5,
        -0.5,
    ),
},1835 => SpriteInfo {
    pos: (
        1358,
        1543,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},412 => SpriteInfo {
    pos: (
        2283,
        779,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},908 => SpriteInfo {
    pos: (
        2416,
        629,
    ),
    size: (
        116,
        65,
    ),
    rotated: false,
    offset: (
        0.0,
        0.5,
    ),
},1756 => SpriteInfo {
    pos: (
        635,
        212,
    ),
    size: (
        16,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2 => SpriteInfo {
    pos: (
        2003,
        839,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1330 => SpriteInfo {
    pos: (
        2651,
        2200,
    ),
    size: (
        130,
        130,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},408 => SpriteInfo {
    pos: (
        2323,
        0,
    ),
    size: (
        21,
        24,
    ),
    rotated: false,
    offset: (
        0.5,
        0.0,
    ),
},1735 => SpriteInfo {
    pos: (
        1005,
        1189,
    ),
    size: (
        206,
        210,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},882 => SpriteInfo {
    pos: (
        3978,
        1175,
    ),
    size: (
        86,
        120,
    ),
    rotated: false,
    offset: (
        17.0,
        0.0,
    ),
},1052 => SpriteInfo {
    pos: (
        140,
        1643,
    ),
    size: (
        120,
        104,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1741 => SpriteInfo {
    pos: (
        2658,
        1731,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},156 => SpriteInfo {
    pos: (
        36,
        415,
    ),
    size: (
        38,
        156,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},918 => SpriteInfo {
    pos: (
        2407,
        1030,
    ),
    size: (
        312,
        279,
    ),
    rotated: false,
    offset: (
        1.0,
        0.5,
    ),
},106 => SpriteInfo {
    pos: (
        3209,
        2281,
    ),
    size: (
        108,
        262,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1059 => SpriteInfo {
    pos: (
        630,
        1809,
    ),
    size: (
        107,
        110,
    ),
    rotated: false,
    offset: (
        6.5,
        -5.0,
    ),
},15 => SpriteInfo {
    pos: (
        4063,
        376,
    ),
    size: (
        26,
        168,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},679 => SpriteInfo {
    pos: (
        873,
        1719,
    ),
    size: (
        206,
        217,
    ),
    rotated: false,
    offset: (
        0.0,
        -0.5,
    ),
},1019 => SpriteInfo {
    pos: (
        1697,
        1557,
    ),
    size: (
        520,
        520,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},721 => SpriteInfo {
    pos: (
        1641,
        1702,
    ),
    size: (
        36,
        36,
    ),
    rotated: false,
    offset: (
        3.0,
        -3.0,
    ),
},1332 => SpriteInfo {
    pos: (
        1058,
        356,
    ),
    size: (
        115,
        26,
    ),
    rotated: false,
    offset: (
        -0.5,
        0.0,
    ),
},1207 => SpriteInfo {
    pos: (
        1660,
        1855,
    ),
    size: (
        24,
        12,
    ),
    rotated: false,
    offset: (
        -48.0,
        -54.0,
    ),
},1586 => SpriteInfo {
    pos: (
        2759,
        1930,
    ),
    size: (
        100,
        142,
    ),
    rotated: false,
    offset: (
        0.0,
        -21.0,
    ),
},396 => SpriteInfo {
    pos: (
        3788,
        207,
    ),
    size: (
        90,
        78,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1866 => SpriteInfo {
    pos: (
        2658,
        1483,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},659 => SpriteInfo {
    pos: (
        3537,
        1322,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4 => SpriteInfo {
    pos: (
        3854,
        1086,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1725 => SpriteInfo {
    pos: (
        2049,
        1433,
    ),
    size: (
        121,
        52,
    ),
    rotated: false,
    offset: (
        -0.5,
        0.0,
    ),
},1333 => SpriteInfo {
    pos: (
        1437,
        995,
    ),
    size: (
        126,
        126,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},137 => SpriteInfo {
    pos: (
        1419,
        2173,
    ),
    size: (
        319,
        318,
    ),
    rotated: false,
    offset: (
        -0.5,
        1.0,
    ),
},1619 => SpriteInfo {
    pos: (
        1015,
        881,
    ),
    size: (
        294,
        304,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1757 => SpriteInfo {
    pos: (
        2785,
        2200,
    ),
    size: (
        60,
        4,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1723 => SpriteInfo {
    pos: (
        2439,
        358,
    ),
    size: (
        144,
        143,
    ),
    rotated: false,
    offset: (
        -9.0,
        -8.5,
    ),
},1837 => SpriteInfo {
    pos: (
        462,
        281,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},409 => SpriteInfo {
    pos: (
        2091,
        265,
    ),
    size: (
        52,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},478 => SpriteInfo {
    pos: (
        2032,
        715,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},880 => SpriteInfo {
    pos: (
        1279,
        1337,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1871 => SpriteInfo {
    pos: (
        3853,
        1210,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},67 => SpriteInfo {
    pos: (
        1437,
        1125,
    ),
    size: (
        99,
        24,
    ),
    rotated: false,
    offset: (
        0.5,
        0.0,
    ),
},140 => SpriteInfo {
    pos: (
        1419,
        2150,
    ),
    size: (
        100,
        19,
    ),
    rotated: false,
    offset: (
        0.0,
        0.5,
    ),
},273 => SpriteInfo {
    pos: (
        2735,
        1006,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1603 => SpriteInfo {
    pos: (
        1059,
        0,
    ),
    size: (
        70,
        58,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1596 => SpriteInfo {
    pos: (
        553,
        212,
    ),
    size: (
        78,
        65,
    ),
    rotated: false,
    offset: (
        0.0,
        -0.5,
    ),
},695 => SpriteInfo {
    pos: (
        3165,
        1121,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1718 => SpriteInfo {
    pos: (
        3768,
        655,
    ),
    size: (
        247,
        151,
    ),
    rotated: false,
    offset: (
        -0.5,
        -10.5,
    ),
},643 => SpriteInfo {
    pos: (
        1313,
        965,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},734 => SpriteInfo {
    pos: (
        2256,
        1263,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},909 => SpriteInfo {
    pos: (
        3701,
        1521,
    ),
    size: (
        80,
        68,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1600 => SpriteInfo {
    pos: (
        3734,
        1022,
    ),
    size: (
        116,
        95,
    ),
    rotated: false,
    offset: (
        0.0,
        0.5,
    ),
},139 => SpriteInfo {
    pos: (
        2413,
        2149,
    ),
    size: (
        110,
        108,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},871 => SpriteInfo {
    pos: (
        1437,
        1153,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},242 => SpriteInfo {
    pos: (
        2199,
        165,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},647 => SpriteInfo {
    pos: (
        3610,
        950,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1607 => SpriteInfo {
    pos: (
        0,
        575,
    ),
    size: (
        42,
        62,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1874 => SpriteInfo {
    pos: (
        741,
        1809,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},20 => SpriteInfo {
    pos: (
        2323,
        238,
    ),
    size: (
        290,
        116,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1053 => SpriteInfo {
    pos: (
        4019,
        711,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1280 => SpriteInfo {
    pos: (
        2658,
        1607,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},642 => SpriteInfo {
    pos: (
        3899,
        1893,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1884 => SpriteInfo {
    pos: (
        1393,
        1667,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},10 => SpriteInfo {
    pos: (
        90,
        2043,
    ),
    size: (
        95,
        298,
    ),
    rotated: false,
    offset: (
        21.5,
        4.0,
    ),
},1883 => SpriteInfo {
    pos: (
        2796,
        792,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},228 => SpriteInfo {
    pos: (
        1215,
        1308,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1879 => SpriteInfo {
    pos: (
        1533,
        157,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},187 => SpriteInfo {
    pos: (
        811,
        145,
    ),
    size: (
        243,
        242,
    ),
    rotated: false,
    offset: (
        -0.5,
        0.0,
    ),
},881 => SpriteInfo {
    pos: (
        81,
        0,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},678 => SpriteInfo {
    pos: (
        2416,
        709,
    ),
    size: (
        315,
        317,
    ),
    rotated: false,
    offset: (
        -0.5,
        -0.5,
    ),
},411 => SpriteInfo {
    pos: (
        2735,
        916,
    ),
    size: (
        120,
        86,
    ),
    rotated: false,
    offset: (
        0.0,
        17.0,
    ),
},1892 => SpriteInfo {
    pos: (
        2859,
        1039,
    ),
    size: (
        50,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1722 => SpriteInfo {
    pos: (
        366,
        1294,
    ),
    size: (
        121,
        37,
    ),
    rotated: false,
    offset: (
        -0.5,
        0.5,
    ),
},51 => SpriteInfo {
    pos: (
        1117,
        1713,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},50 => SpriteInfo {
    pos: (
        1697,
        1433,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},694 => SpriteInfo {
    pos: (
        2782,
        1558,
    ),
    size: (
        240,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1618 => SpriteInfo {
    pos: (
        1641,
        1810,
    ),
    size: (
        40,
        41,
    ),
    rotated: false,
    offset: (
        0.0,
        0.5,
    ),
},1768 => SpriteInfo {
    pos: (
        1660,
        1871,
    ),
    size: (
        22,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},141 => SpriteInfo {
    pos: (
        0,
        1835,
    ),
    size: (
        121,
        120,
    ),
    rotated: false,
    offset: (
        -0.5,
        0.0,
    ),
},935 => SpriteInfo {
    pos: (
        655,
        466,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},413 => SpriteInfo {
    pos: (
        3672,
        1198,
    ),
    size: (
        52,
        86,
    ),
    rotated: false,
    offset: (
        0.0,
        17.0,
    ),
},890 => SpriteInfo {
    pos: (
        2829,
        0,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1890 => SpriteInfo {
    pos: (
        3728,
        1245,
    ),
    size: (
        120,
        54,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1864 => SpriteInfo {
    pos: (
        2289,
        2273,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},185 => SpriteInfo {
    pos: (
        189,
        2065,
    ),
    size: (
        38,
        160,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},395 => SpriteInfo {
    pos: (
        3372,
        518,
    ),
    size: (
        194,
        168,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1267 => SpriteInfo {
    pos: (
        3041,
        1121,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},931 => SpriteInfo {
    pos: (
        1455,
        2026,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1004 => SpriteInfo {
    pos: (
        2283,
        767,
    ),
    size: (
        120,
        8,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1845 => SpriteInfo {
    pos: (
        1117,
        1595,
    ),
    size: (
        154,
        114,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3 => SpriteInfo {
    pos: (
        2003,
        963,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1709 => SpriteInfo {
    pos: (
        2987,
        2054,
    ),
    size: (
        218,
        252,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1882 => SpriteInfo {
    pos: (
        2865,
        248,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1740 => SpriteInfo {
    pos: (
        3638,
        826,
    ),
    size: (
        80,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1003 => SpriteInfo {
    pos: (
        790,
        391,
    ),
    size: (
        120,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        28.0,
    ),
},1274 => SpriteInfo {
    pos: (
        1969,
        0,
    ),
    size: (
        34,
        78,
    ),
    rotated: false,
    offset: (
        23.0,
        1.0,
    ),
},1707 => SpriteInfo {
    pos: (
        1573,
        1022,
    ),
    size: (
        155,
        157,
    ),
    rotated: false,
    offset: (
        1.5,
        0.5,
    ),
},648 => SpriteInfo {
    pos: (
        244,
        1109,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},47 => SpriteInfo {
    pos: (
        1177,
        386,
    ),
    size: (
        132,
        338,
    ),
    rotated: false,
    offset: (
        23.0,
        1.0,
    ),
},1891 => SpriteInfo {
    pos: (
        4006,
        248,
    ),
    size: (
        80,
        76,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1872 => SpriteInfo {
    pos: (
        3570,
        524,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},35 => SpriteInfo {
    pos: (
        3524,
        1446,
    ),
    size: (
        99,
        16,
    ),
    rotated: false,
    offset: (
        0.5,
        0.0,
    ),
},45 => SpriteInfo {
    pos: (
        3071,
        0,
    ),
    size: (
        173,
        360,
    ),
    rotated: false,
    offset: (
        30.5,
        0.0,
    ),
},700 => SpriteInfo {
    pos: (
        1133,
        0,
    ),
    size: (
        220,
        110,
    ),
    rotated: false,
    offset: (
        10.0,
        5.0,
    ),
},870 => SpriteInfo {
    pos: (
        1038,
        391,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},928 => SpriteInfo {
    pos: (
        2431,
        505,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1206 => SpriteInfo {
    pos: (
        838,
        1933,
    ),
    size: (
        12,
        12,
    ),
    rotated: false,
    offset: (
        -54.0,
        -54.0,
    ),
},1058 => SpriteInfo {
    pos: (
        2975,
        621,
    ),
    size: (
        150,
        144,
    ),
    rotated: false,
    offset: (
        -5.0,
        -8.0,
    ),
},1001 => SpriteInfo {
    pos: (
        3650,
        1942,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},701 => SpriteInfo {
    pos: (
        3248,
        177,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},111 => SpriteInfo {
    pos: (
        1561,
        1326,
    ),
    size: (
        132,
        338,
    ),
    rotated: false,
    offset: (
        23.0,
        1.0,
    ),
},883 => SpriteInfo {
    pos: (
        3457,
        690,
    ),
    size: (
        86,
        120,
    ),
    rotated: false,
    offset: (
        -17.0,
        0.0,
    ),
},1698 => SpriteInfo {
    pos: (
        1263,
        114,
    ),
    size: (
        76,
        72,
    ),
    rotated: false,
    offset: (
        22.0,
        24.0,
    ),
},474 => SpriteInfo {
    pos: (
        648,
        121,
    ),
    size: (
        14,
        7,
    ),
    rotated: false,
    offset: (
        -53.0,
        -56.5,
    ),
},1022 => SpriteInfo {
    pos: (
        1517,
        1904,
    ),
    size: (
        118,
        118,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},18 => SpriteInfo {
    pos: (
        2380,
        1313,
    ),
    size: (
        510,
        166,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1281 => SpriteInfo {
    pos: (
        2555,
        555,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},644 => SpriteInfo {
    pos: (
        3245,
        1909,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1847 => SpriteInfo {
    pos: (
        2562,
        1953,
    ),
    size: (
        162,
        222,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},414 => SpriteInfo {
    pos: (
        2785,
        2208,
    ),
    size: (
        54,
        53,
    ),
    rotated: false,
    offset: (
        0.0,
        0.5,
    ),
},1584 => SpriteInfo {
    pos: (
        124,
        1319,
    ),
    size: (
        230,
        245,
    ),
    rotated: false,
    offset: (
        16.0,
        -35.5,
    ),
},1 => SpriteInfo {
    pos: (
        2835,
        496,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1210 => SpriteInfo {
    pos: (
        3413,
        1224,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1869 => SpriteInfo {
    pos: (
        3922,
        0,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},471 => SpriteInfo {
    pos: (
        1730,
        378,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},917 => SpriteInfo {
    pos: (
        4058,
        1734,
    ),
    size: (
        30,
        30,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},149 => SpriteInfo {
    pos: (
        1234,
        1461,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},394 => SpriteInfo {
    pos: (
        0,
        736,
    ),
    size: (
        321,
        279,
    ),
    rotated: false,
    offset: (
        -0.5,
        -0.5,
    ),
},1710 => SpriteInfo {
    pos: (
        486,
        724,
    ),
    size: (
        172,
        172,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1205 => SpriteInfo {
    pos: (
        1955,
        82,
    ),
    size: (
        12,
        12,
    ),
    rotated: false,
    offset: (
        -54.0,
        -54.0,
    ),
},693 => SpriteInfo {
    pos: (
        2183,
        488,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},927 => SpriteInfo {
    pos: (
        587,
        600,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},889 => SpriteInfo {
    pos: (
        2323,
        106,
    ),
    size: (
        240,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1717 => SpriteInfo {
    pos: (
        1821,
        54,
    ),
    size: (
        130,
        138,
    ),
    rotated: false,
    offset: (
        -1.0,
        -5.0,
    ),
},1620 => SpriteInfo {
    pos: (
        2975,
        769,
    ),
    size: (
        123,
        125,
    ),
    rotated: false,
    offset: (
        -36.5,
        -37.5,
    ),
},697 => SpriteInfo {
    pos: (
        0,
        1226,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},85 => SpriteInfo {
    pos: (
        3780,
        372,
    ),
    size: (
        279,
        279,
    ),
    rotated: false,
    offset: (
        -0.5,
        -0.5,
    ),
},46 => SpriteInfo {
    pos: (
        696,
        1445,
    ),
    size: (
        173,
        360,
    ),
    rotated: false,
    offset: (
        30.5,
        0.0,
    ),
},1721 => SpriteInfo {
    pos: (
        3644,
        455,
    ),
    size: (
        120,
        65,
    ),
    rotated: false,
    offset: (
        0.0,
        18.5,
    ),
},745 => SpriteInfo {
    pos: (
        1313,
        623,
    ),
    size: (
        132,
        338,
    ),
    rotated: false,
    offset: (
        23.0,
        1.0,
    ),
},217 => SpriteInfo {
    pos: (
        1781,
        196,
    ),
    size: (
        120,
        54,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1005 => SpriteInfo {
    pos: (
        68,
        347,
    ),
    size: (
        8,
        51,
    ),
    rotated: false,
    offset: (
        0.0,
        34.5,
    ),
},177 => SpriteInfo {
    pos: (
        1449,
        623,
    ),
    size: (
        121,
        120,
    ),
    rotated: false,
    offset: (
        -0.5,
        0.0,
    ),
},224 => SpriteInfo {
    pos: (
        1561,
        1183,
    ),
    size: (
        144,
        139,
    ),
    rotated: false,
    offset: (
        0.0,
        -4.5,
    ),
},1766 => SpriteInfo {
    pos: (
        3416,
        830,
    ),
    size: (
        32,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},259 => SpriteInfo {
    pos: (
        1295,
        2039,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},19 => SpriteInfo {
    pos: (
        1732,
        1107,
    ),
    size: (
        416,
        145,
    ),
    rotated: false,
    offset: (
        0.0,
        0.5,
    ),
},87 => SpriteInfo {
    pos: (
        1182,
        232,
    ),
    size: (
        150,
        150,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1715 => SpriteInfo {
    pos: (
        2634,
        1855,
    ),
    size: (
        121,
        93,
    ),
    rotated: false,
    offset: (
        -0.5,
        5.5,
    ),
},114 => SpriteInfo {
    pos: (
        81,
        124,
    ),
    size: (
        344,
        133,
    ),
    rotated: false,
    offset: (
        0.0,
        -3.5,
    ),
},1888 => SpriteInfo {
    pos: (
        2591,
        0,
    ),
    size: (
        234,
        234,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1020 => SpriteInfo {
    pos: (
        865,
        1997,
    ),
    size: (
        426,
        426,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},480 => SpriteInfo {
    pos: (
        325,
        921,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1273 => SpriteInfo {
    pos: (
        2859,
        957,
    ),
    size: (
        54,
        78,
    ),
    rotated: false,
    offset: (
        13.0,
        1.0,
    ),
},878 => SpriteInfo {
    pos: (
        914,
        515,
    ),
    size: (
        240,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1700 => SpriteInfo {
    pos: (
        2152,
        1126,
    ),
    size: (
        100,
        142,
    ),
    rotated: false,
    offset: (
        0.0,
        -21.0,
    ),
},1832 => SpriteInfo {
    pos: (
        3550,
        92,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        -30.0,
        -30.0,
    ),
},222 => SpriteInfo {
    pos: (
        81,
        261,
    ),
    size: (
        377,
        377,
    ),
    rotated: false,
    offset: (
        -0.5,
        -0.5,
    ),
},1202 => SpriteInfo {
    pos: (
        3333,
        690,
    ),
    size: (
        120,
        12,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},914 => SpriteInfo {
    pos: (
        3642,
        2066,
    ),
    size: (
        103,
        97,
    ),
    rotated: false,
    offset: (
        -0.5,
        10.5,
    ),
},877 => SpriteInfo {
    pos: (
        2782,
        1682,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1204 => SpriteInfo {
    pos: (
        140,
        1751,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1137 => SpriteInfo {
    pos: (
        3348,
        997,
    ),
    size: (
        91,
        74,
    ),
    rotated: false,
    offset: (
        0.5,
        0.0,
    ),
},216 => SpriteInfo {
    pos: (
        1331,
        1915,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},115 => SpriteInfo {
    pos: (
        3671,
        98,
    ),
    size: (
        222,
        105,
    ),
    rotated: false,
    offset: (
        0.0,
        -1.5,
    ),
},1203 => SpriteInfo {
    pos: (
        2894,
        1434,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1753 => SpriteInfo {
    pos: (
        2283,
        1007,
    ),
    size: (
        120,
        4,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1705 => SpriteInfo {
    pos: (
        358,
        1403,
    ),
    size: (
        334,
        328,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1597 => SpriteInfo {
    pos: (
        0,
        282,
    ),
    size: (
        72,
        61,
    ),
    rotated: false,
    offset: (
        0.0,
        -1.5,
    ),
},503 => SpriteInfo {
    pos: (
        1917,
        2081,
    ),
    size: (
        120,
        77,
    ),
    rotated: false,
    offset: (
        0.0,
        1.5,
    ),
},938 => SpriteInfo {
    pos: (
        3774,
        0,
    ),
    size: (
        144,
        94,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},660 => SpriteInfo {
    pos: (
        675,
        0,
    ),
    size: (
        132,
        338,
    ),
    rotated: false,
    offset: (
        23.0,
        1.0,
    ),
},1880 => SpriteInfo {
    pos: (
        790,
        459,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},154 => SpriteInfo {
    pos: (
        3785,
        1453,
    ),
    size: (
        277,
        277,
    ),
    rotated: false,
    offset: (
        -0.5,
        -0.5,
    ),
},646 => SpriteInfo {
    pos: (
        2307,
        488,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1844 => SpriteInfo {
    pos: (
        1821,
        1433,
    ),
    size: (
        224,
        114,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},476 => SpriteInfo {
    pos: (
        1295,
        2279,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1002 => SpriteInfo {
    pos: (
        0,
        347,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        28.0,
        28.0,
    ),
},155 => SpriteInfo {
    pos: (
        2007,
        0,
    ),
    size: (
        188,
        163,
    ),
    rotated: false,
    offset: (
        0.0,
        -26.5,
    ),
},83 => SpriteInfo {
    pos: (
        2679,
        585,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},907 => SpriteInfo {
    pos: (
        80,
        642,
    ),
    size: (
        278,
        90,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1885 => SpriteInfo {
    pos: (
        429,
        124,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1848 => SpriteInfo {
    pos: (
        641,
        946,
    ),
    size: (
        154,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        -3.0,
    ),
},179 => SpriteInfo {
    pos: (
        0,
        2121,
    ),
    size: (
        81,
        80,
    ),
    rotated: false,
    offset: (
        -0.5,
        0.0,
    ),
},1606 => SpriteInfo {
    pos: (
        214,
        1999,
    ),
    size: (
        28,
        62,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},410 => SpriteInfo {
    pos: (
        3976,
        1363,
    ),
    size: (
        86,
        86,
    ),
    rotated: false,
    offset: (
        17.0,
        17.0,
    ),
},1699 => SpriteInfo {
    pos: (
        1641,
        1742,
    ),
    size: (
        39,
        64,
    ),
    rotated: false,
    offset: (
        3.5,
        28.0,
    ),
},1050 => SpriteInfo {
    pos: (
        2894,
        1326,
    ),
    size: (
        120,
        104,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},12 => SpriteInfo {
    pos: (
        1955,
        167,
    ),
    size: (
        132,
        338,
    ),
    rotated: false,
    offset: (
        23.0,
        1.0,
    ),
},1754 => SpriteInfo {
    pos: (
        1846,
        0,
    ),
    size: (
        119,
        4,
    ),
    rotated: false,
    offset: (
        -0.5,
        0.0,
    ),
},1582 => SpriteInfo {
    pos: (
        2723,
        1130,
    ),
    size: (
        101,
        97,
    ),
    rotated: false,
    offset: (
        -0.5,
        1.5,
    ),
},1834 => SpriteInfo {
    pos: (
        3977,
        1299,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        -30.0,
        -30.0,
    ),
},1862 => SpriteInfo {
    pos: (
        3548,
        1198,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1831 => SpriteInfo {
    pos: (
        0,
        1474,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        -60.0,
        -60.0,
    ),
},916 => SpriteInfo {
    pos: (
        4021,
        124,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},136 => SpriteInfo {
    pos: (
        0,
        1643,
    ),
    size: (
        136,
        188,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1865 => SpriteInfo {
    pos: (
        3018,
        1369,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1060 => SpriteInfo {
    pos: (
        329,
        0,
    ),
    size: (
        97,
        96,
    ),
    rotated: false,
    offset: (
        -2.5,
        3.0,
    ),
},60 => SpriteInfo {
    pos: (
        1449,
        871,
    ),
    size: (
        110,
        120,
    ),
    rotated: false,
    offset: (
        -10.0,
        0.0,
    ),
},1601 => SpriteInfo {
    pos: (
        2147,
        265,
    ),
    size: (
        32,
        160,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1583 => SpriteInfo {
    pos: (
        3550,
        0,
    ),
    size: (
        117,
        88,
    ),
    rotated: false,
    offset: (
        -21.5,
        -1.0,
    ),
},1881 => SpriteInfo {
    pos: (
        3605,
        1074,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},143 => SpriteInfo {
    pos: (
        2165,
        2149,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1697 => SpriteInfo {
    pos: (
        2723,
        1231,
    ),
    size: (
        120,
        51,
    ),
    rotated: false,
    offset: (
        0.0,
        3.5,
    ),
},1758 => SpriteInfo {
    pos: (
        3248,
        0,
    ),
    size: (
        174,
        173,
    ),
    rotated: false,
    offset: (
        1.0,
        1.5,
    ),
},133 => SpriteInfo {
    pos: (
        368,
        1045,
    ),
    size: (
        56,
        186,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},680 => SpriteInfo {
    pos: (
        3333,
        2221,
    ),
    size: (
        144,
        144,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1277 => SpriteInfo {
    pos: (
        2829,
        124,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1873 => SpriteInfo {
    pos: (
        3882,
        248,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1730 => SpriteInfo {
    pos: (
        3369,
        2157,
    ),
    size: (
        121,
        60,
    ),
    rotated: false,
    offset: (
        -0.5,
        0.0,
    ),
},13 => SpriteInfo {
    pos: (
        711,
        604,
    ),
    size: (
        132,
        338,
    ),
    rotated: false,
    offset: (
        23.0,
        1.0,
    ),
},932 => SpriteInfo {
    pos: (
        3289,
        1121,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},107 => SpriteInfo {
    pos: (
        3221,
        673,
    ),
    size: (
        108,
        152,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1724 => SpriteInfo {
    pos: (
        2156,
        612,
    ),
    size: (
        256,
        151,
    ),
    rotated: false,
    offset: (
        -5.0,
        -9.5,
    ),
},1247 => SpriteInfo {
    pos: (
        811,
        0,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1720 => SpriteInfo {
    pos: (
        462,
        600,
    ),
    size: (
        121,
        69,
    ),
    rotated: false,
    offset: (
        -0.5,
        17.5,
    ),
},719 => SpriteInfo {
    pos: (
        2199,
        0,
    ),
    size: (
        120,
        37,
    ),
    rotated: false,
    offset: (
        0.0,
        -17.5,
    ),
},138 => SpriteInfo {
    pos: (
        1948,
        509,
    ),
    size: (
        204,
        202,
    ),
    rotated: false,
    offset: (
        0.0,
        1.0,
    ),
},16 => SpriteInfo {
    pos: (
        4068,
        839,
    ),
    size: (
        24,
        106,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},157 => SpriteInfo {
    pos: (
        2211,
        1387,
    ),
    size: (
        120,
        103,
    ),
    rotated: false,
    offset: (
        0.0,
        0.5,
    ),
},48 => SpriteInfo {
    pos: (
        1357,
        0,
    ),
    size: (
        460,
        153,
    ),
    rotated: false,
    offset: (
        0.0,
        -0.5,
    ),
},178 => SpriteInfo {
    pos: (
        1083,
        1837,
    ),
    size: (
        120,
        62,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},6 => SpriteInfo {
    pos: (
        3426,
        0,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1339 => SpriteInfo {
    pos: (
        2187,
        358,
    ),
    size: (
        248,
        126,
    ),
    rotated: false,
    offset: (
        3.0,
        1.0,
    ),
},1889 => SpriteInfo {
    pos: (
        3248,
        301,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},
        _ => return None,
    })
}
    


pub fn get_detail_sprite(id: u32) -> Option<SpriteInfo> {
    Some(match id {
        1002 => SpriteInfo {
    pos: (
        4063,
        328,
    ),
    size: (
        20,
        20,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1600 => SpriteInfo {
    pos: (
        1732,
        1022,
    ),
    size: (
        101,
        81,
    ),
    rotated: false,
    offset: (
        -3.5,
        -3.5,
    ),
},1698 => SpriteInfo {
    pos: (
        554,
        121,
    ),
    size: (
        90,
        87,
    ),
    rotated: false,
    offset: (
        15.0,
        16.5,
    ),
},877 => SpriteInfo {
    pos: (
        3701,
        1505,
    ),
    size: (
        63,
        12,
    ),
    rotated: false,
    offset: (
        24.5,
        12.0,
    ),
},1601 => SpriteInfo {
    pos: (
        1937,
        236,
    ),
    size: (
        7,
        150,
    ),
    rotated: false,
    offset: (
        -8.5,
        0.0,
    ),
},410 => SpriteInfo {
    pos: (
        3796,
        1303,
    ),
    size: (
        36,
        36,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1709 => SpriteInfo {
    pos: (
        0,
        2049,
    ),
    size: (
        68,
        68,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1875 => SpriteInfo {
    pos: (
        1058,
        232,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},481 => SpriteInfo {
    pos: (
        0,
        1143,
    ),
    size: (
        120,
        79,
    ),
    rotated: false,
    offset: (
        0.0,
        20.5,
    ),
},1247 => SpriteInfo {
    pos: (
        935,
        0,
    ),
    size: (
        120,
        17,
    ),
    rotated: false,
    offset: (
        0.0,
        -48.5,
    ),
},678 => SpriteInfo {
    pos: (
        1574,
        717,
    ),
    size: (
        301,
        301,
    ),
    rotated: false,
    offset: (
        -0.5,
        -0.5,
    ),
},1882 => SpriteInfo {
    pos: (
        1781,
        254,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1004 => SpriteInfo {
    pos: (
        847,
        648,
    ),
    size: (
        20,
        20,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},477 => SpriteInfo {
    pos: (
        3694,
        524,
    ),
    size: (
        38,
        38,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1884 => SpriteInfo {
    pos: (
        1517,
        1668,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1867 => SpriteInfo {
    pos: (
        1517,
        1792,
    ),
    size: (
        120,
        108,
    ),
    rotated: false,
    offset: (
        0.0,
        -3.0,
    ),
},647 => SpriteInfo {
    pos: (
        1879,
        963,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},478 => SpriteInfo {
    pos: (
        3694,
        566,
    ),
    size: (
        38,
        79,
    ),
    rotated: false,
    offset: (
        0.0,
        20.5,
    ),
},883 => SpriteInfo {
    pos: (
        1015,
        763,
    ),
    size: (
        80,
        114,
    ),
    rotated: false,
    offset: (
        -20.0,
        3.0,
    ),
},896 => SpriteInfo {
    pos: (
        3026,
        1595,
    ),
    size: (
        223,
        114,
    ),
    rotated: false,
    offset: (
        5.5,
        1.0,
    ),
},890 => SpriteInfo {
    pos: (
        2953,
        0,
    ),
    size: (
        114,
        114,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1865 => SpriteInfo {
    pos: (
        3142,
        1374,
    ),
    size: (
        120,
        108,
    ),
    rotated: false,
    offset: (
        0.0,
        1.0,
    ),
},216 => SpriteInfo {
    pos: (
        606,
        1923,
    ),
    size: (
        104,
        103,
    ),
    rotated: false,
    offset: (
        0.0,
        3.5,
    ),
},412 => SpriteInfo {
    pos: (
        847,
        672,
    ),
    size: (
        36,
        36,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},873 => SpriteInfo {
    pos: (
        3526,
        2066,
    ),
    size: (
        112,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},1583 => SpriteInfo {
    pos: (
        3671,
        0,
    ),
    size: (
        99,
        81,
    ),
    rotated: false,
    offset: (
        -16.5,
        -1.5,
    ),
},1279 => SpriteInfo {
    pos: (
        554,
        0,
    ),
    size: (
        117,
        117,
    ),
    rotated: false,
    offset: (
        1.5,
        1.5,
    ),
},482 => SpriteInfo {
    pos: (
        1908,
        715,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1872 => SpriteInfo {
    pos: (
        3221,
        549,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1277 => SpriteInfo {
    pos: (
        3426,
        124,
    ),
    size: (
        120,
        17,
    ),
    rotated: false,
    offset: (
        0.0,
        -48.5,
    ),
},1878 => SpriteInfo {
    pos: (
        3266,
        1432,
    ),
    size: (
        120,
        114,
    ),
    rotated: false,
    offset: (
        0.0,
        3.0,
    ),
},479 => SpriteInfo {
    pos: (
        1854,
        378,
    ),
    size: (
        79,
        79,
    ),
    rotated: false,
    offset: (
        20.5,
        20.5,
    ),
},1586 => SpriteInfo {
    pos: (
        714,
        1933,
    ),
    size: (
        92,
        92,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1734 => SpriteInfo {
    pos: (
        3390,
        1505,
    ),
    size: (
        307,
        309,
    ),
    rotated: false,
    offset: (
        -0.5,
        -0.5,
    ),
},882 => SpriteInfo {
    pos: (
        428,
        1176,
    ),
    size: (
        80,
        114,
    ),
    rotated: false,
    offset: (
        20.0,
        3.0,
    ),
},1877 => SpriteInfo {
    pos: (
        1024,
        639,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1003 => SpriteInfo {
    pos: (
        3372,
        218,
    ),
    size: (
        20,
        20,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},641 => SpriteInfo {
    pos: (
        482,
        1883,
    ),
    size: (
        120,
        90,
    ),
    rotated: false,
    offset: (
        0.0,
        15.0,
    ),
},1879 => SpriteInfo {
    pos: (
        1657,
        157,
    ),
    size: (
        120,
        114,
    ),
    rotated: false,
    offset: (
        0.0,
        3.0,
    ),
},644 => SpriteInfo {
    pos: (
        2562,
        1889,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},881 => SpriteInfo {
    pos: (
        205,
        0,
    ),
    size: (
        120,
        114,
    ),
    rotated: false,
    offset: (
        0.0,
        3.0,
    ),
},1873 => SpriteInfo {
    pos: (
        2741,
        248,
    ),
    size: (
        120,
        117,
    ),
    rotated: false,
    offset: (
        0.0,
        -1.5,
    ),
},409 => SpriteInfo {
    pos: (
        1905,
        196,
    ),
    size: (
        36,
        36,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},645 => SpriteInfo {
    pos: (
        590,
        2154,
    ),
    size: (
        60,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1266 => SpriteInfo {
    pos: (
        2953,
        118,
    ),
    size: (
        114,
        112,
    ),
    rotated: false,
    offset: (
        3.0,
        2.0,
    ),
},217 => SpriteInfo {
    pos: (
        2091,
        167,
    ),
    size: (
        92,
        42,
    ),
    rotated: false,
    offset: (
        0.0,
        1.0,
    ),
},888 => SpriteInfo {
    pos: (
        4007,
        2017,
    ),
    size: (
        80,
        112,
    ),
    rotated: false,
    offset: (
        20.0,
        4.0,
    ),
},411 => SpriteInfo {
    pos: (
        662,
        724,
    ),
    size: (
        36,
        36,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1881 => SpriteInfo {
    pos: (
        2917,
        1078,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1582 => SpriteInfo {
    pos: (
        2828,
        1130,
    ),
    size: (
        84,
        83,
    ),
    rotated: false,
    offset: (
        0.0,
        -0.5,
    ),
},1862 => SpriteInfo {
    pos: (
        2916,
        1202,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1699 => SpriteInfo {
    pos: (
        2562,
        1806,
    ),
    size: (
        68,
        79,
    ),
    rotated: false,
    offset: (
        4.0,
        20.5,
    ),
},1710 => SpriteInfo {
    pos: (
        847,
        604,
    ),
    size: (
        40,
        40,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1870 => SpriteInfo {
    pos: (
        2041,
        2205,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1880 => SpriteInfo {
    pos: (
        2711,
        461,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1700 => SpriteInfo {
    pos: (
        3452,
        1128,
    ),
    size: (
        92,
        92,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},878 => SpriteInfo {
    pos: (
        1664,
        502,
    ),
    size: (
        112,
        87,
    ),
    rotated: false,
    offset: (
        60.0,
        14.5,
    ),
},1868 => SpriteInfo {
    pos: (
        462,
        405,
    ),
    size: (
        120,
        110,
    ),
    rotated: false,
    offset: (
        0.0,
        3.0,
    ),
},1874 => SpriteInfo {
    pos: (
        3526,
        1818,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},646 => SpriteInfo {
    pos: (
        1854,
        461,
    ),
    size: (
        90,
        120,
    ),
    rotated: false,
    offset: (
        -15.0,
        0.0,
    ),
},889 => SpriteInfo {
    pos: (
        1059,
        114,
    ),
    size: (
        200,
        114,
    ),
    rotated: false,
    offset: (
        20.0,
        3.0,
    ),
},895 => SpriteInfo {
    pos: (
        1295,
        2163,
    ),
    size: (
        111,
        112,
    ),
    rotated: false,
    offset: (
        2.5,
        2.0,
    ),
},1620 => SpriteInfo {
    pos: (
        2735,
        709,
    ),
    size: (
        57,
        107,
    ),
    rotated: false,
    offset: (
        -13.5,
        -38.5,
    ),
},1871 => SpriteInfo {
    pos: (
        1297,
        1213,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1001 => SpriteInfo {
    pos: (
        4068,
        1185,
    ),
    size: (
        20,
        20,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1885 => SpriteInfo {
    pos: (
        3897,
        124,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},871 => SpriteInfo {
    pos: (
        124,
        1143,
    ),
    size: (
        114,
        112,
    ),
    rotated: false,
    offset: (
        3.0,
        2.0,
    ),
},1619 => SpriteInfo {
    pos: (
        449,
        900,
    ),
    size: (
        188,
        272,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1735 => SpriteInfo {
    pos: (
        512,
        1201,
    ),
    size: (
        195,
        198,
    ),
    rotated: false,
    offset: (
        -0.5,
        0.0,
    ),
},1054 => SpriteInfo {
    pos: (
        244,
        1045,
    ),
    size: (
        120,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1863 => SpriteInfo {
    pos: (
        590,
        2030,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},918 => SpriteInfo {
    pos: (
        3556,
        960,
    ),
    size: (
        45,
        43,
    ),
    rotated: false,
    offset: (
        -22.5,
        -77.5,
    ),
},1866 => SpriteInfo {
    pos: (
        3142,
        1486,
    ),
    size: (
        120,
        105,
    ),
    rotated: false,
    offset: (
        0.0,
        2.5,
    ),
},680 => SpriteInfo {
    pos: (
        3481,
        2221,
    ),
    size: (
        133,
        133,
    ),
    rotated: false,
    offset: (
        0.5,
        -0.5,
    ),
},939 => SpriteInfo {
    pos: (
        654,
        2154,
    ),
    size: (
        42,
        42,
    ),
    rotated: false,
    offset: (
        1.0,
        -14.0,
    ),
},218 => SpriteInfo {
    pos: (
        1215,
        1241,
    ),
    size: (
        66,
        63,
    ),
    rotated: false,
    offset: (
        0.0,
        2.5,
    ),
},1869 => SpriteInfo {
    pos: (
        935,
        21,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1708 => SpriteInfo {
    pos: (
        0,
        1959,
    ),
    size: (
        86,
        86,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1861 => SpriteInfo {
    pos: (
        891,
        763,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1053 => SpriteInfo {
    pos: (
        4019,
        775,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},480 => SpriteInfo {
    pos: (
        662,
        764,
    ),
    size: (
        38,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1864 => SpriteInfo {
    pos: (
        700,
        2277,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1883 => SpriteInfo {
    pos: (
        325,
        797,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1697 => SpriteInfo {
    pos: (
        242,
        1233,
    ),
    size: (
        120,
        82,
    ),
    rotated: false,
    offset: (
        0.0,
        4.0,
    ),
},1005 => SpriteInfo {
    pos: (
        4063,
        352,
    ),
    size: (
        20,
        20,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},880 => SpriteInfo {
    pos: (
        3400,
        1348,
    ),
    size: (
        120,
        114,
    ),
    rotated: false,
    offset: (
        0.0,
        3.0,
    ),
},642 => SpriteInfo {
    pos: (
        1083,
        1903,
    ),
    size: (
        90,
        90,
    ),
    rotated: false,
    offset: (
        15.0,
        15.0,
    ),
},413 => SpriteInfo {
    pos: (
        965,
        1063,
    ),
    size: (
        36,
        36,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},739 => SpriteInfo {
    pos: (
        3526,
        2122,
    ),
    size: (
        90,
        90,
    ),
    rotated: false,
    offset: (
        15.0,
        15.0,
    ),
},648 => SpriteInfo {
    pos: (
        3729,
        1121,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1267 => SpriteInfo {
    pos: (
        3978,
        1059,
    ),
    size: (
        114,
        112,
    ),
    rotated: false,
    offset: (
        -3.0,
        2.0,
    ),
},458 => SpriteInfo {
    pos: (
        2859,
        916,
    ),
    size: (
        38,
        37,
    ),
    rotated: false,
    offset: (
        0.0,
        2.5,
    ),
},869 => SpriteInfo {
    pos: (
        3774,
        1942,
    ),
    size: (
        120,
        112,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},643 => SpriteInfo {
    pos: (
        4002,
        965,
    ),
    size: (
        60,
        90,
    ),
    rotated: false,
    offset: (
        0.0,
        15.0,
    ),
},649 => SpriteInfo {
    pos: (
        3224,
        997,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1281 => SpriteInfo {
    pos: (
        779,
        583,
    ),
    size: (
        117,
        17,
    ),
    rotated: false,
    offset: (
        -1.5,
        -48.5,
    ),
},1843 => SpriteInfo {
    pos: (
        3898,
        2017,
    ),
    size: (
        105,
        26,
    ),
    rotated: false,
    offset: (
        -3.5,
        -3.0,
    ),
},1876 => SpriteInfo {
    pos: (
        3749,
        2122,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},870 => SpriteInfo {
    pos: (
        3396,
        394,
    ),
    size: (
        120,
        112,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},874 => SpriteInfo {
    pos: (
        4019,
        655,
    ),
    size: (
        52,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},679 => SpriteInfo {
    pos: (
        3701,
        1734,
    ),
    size: (
        194,
        204,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1736 => SpriteInfo {
    pos: (
        482,
        1735,
    ),
    size: (
        144,
        144,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},
        _ => return None,
    })
}
    

    