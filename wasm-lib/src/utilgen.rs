
use the_nexus::objects::{ObjectCategory::*, ObjectInfo};
use the_nexus::packing::SpriteInfo;


pub fn get_object_info(id: u32) -> Option<ObjectInfo> {
    Some(match id {
        47 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},874 => ObjectInfo {
    place_offset_x: -7.5,
    place_offset_y: -7.5,
    tintable: true,
    solid: false,
    category: Blocks,
},1715 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -12.5,
    tintable: true,
    solid: false,
    category: Spikes,
},1605 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},45 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},1868 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},468 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 14.25,
    tintable: true,
    solid: true,
    category: Outlines,
},41 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 20.0,
    tintable: true,
    solid: false,
    category: Deco,
},693 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},1889 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Spikes,
},680 => ObjectInfo {
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
},115 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -5.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},480 => ObjectInfo {
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
},1876 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},938 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1870 => ObjectInfo {
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
},227 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -4.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},928 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},224 => ObjectInfo {
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
},659 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},916 => ObjectInfo {
    place_offset_x: -7.5,
    place_offset_y: -7.5,
    tintable: true,
    solid: false,
    category: Deco,
},467 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},739 => ObjectInfo {
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
},1273 => ObjectInfo {
    place_offset_x: 5.0,
    place_offset_y: -5.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},927 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},54 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},86 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1004 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1888 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},1207 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},4 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Blocks,
},1847 => ObjectInfo {
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
},719 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -7.5,
    tintable: true,
    solid: false,
    category: Deco,
},177 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Spikes,
},1698 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Utilities,
},84 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},909 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -7.5,
    tintable: true,
    solid: false,
    category: Deco,
},931 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},83 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Blocks,
},266 => ObjectInfo {
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
},917 => ObjectInfo {
    place_offset_x: -11.25,
    place_offset_y: -11.25,
    tintable: true,
    solid: false,
    category: Deco,
},1729 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -7.5,
    tintable: true,
    solid: false,
    category: Spikes,
},1755 => ObjectInfo {
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
},700 => ObjectInfo {
    place_offset_x: 15.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},1019 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1845 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1837 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},35 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -13.0,
    tintable: false,
    solid: false,
    category: Utilities,
},1002 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1859 => ObjectInfo {
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
},1333 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},149 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},896 => ObjectInfo {
    place_offset_x: 15.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},473 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},471 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},1050 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -2.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},883 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},10 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},188 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},679 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},114 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -2.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},414 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -9.0,
    tintable: true,
    solid: false,
    category: Deco,
},702 => ObjectInfo {
    place_offset_x: 15.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},1728 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -7.5,
    tintable: true,
    solid: false,
    category: Spikes,
},1734 => ObjectInfo {
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
},85 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1054 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -7.5,
    tintable: true,
    solid: false,
    category: Utilities,
},1872 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1725 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -9.0,
    tintable: true,
    solid: false,
    category: Spikes,
},1880 => ObjectInfo {
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
},159 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -1.5,
    tintable: true,
    solid: false,
    category: GroundDeco,
},2 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Blocks,
},1710 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},137 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},919 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -10.0,
    tintable: true,
    solid: false,
    category: Utilities,
},1005 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},908 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -7.5,
    tintable: true,
    solid: false,
    category: Deco,
},929 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1604 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},97 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},458 => ObjectInfo {
    place_offset_x: -7.5,
    place_offset_y: -9.75,
    tintable: true,
    solid: false,
    category: Spikes,
},932 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},881 => ObjectInfo {
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
},1134 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},698 => ObjectInfo {
    place_offset_x: 15.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},222 => ObjectInfo {
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
},889 => ObjectInfo {
    place_offset_x: 15.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},259 => ObjectInfo {
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
},187 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Blocks,
},481 => ObjectInfo {
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
},650 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1596 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1602 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1864 => ObjectInfo {
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
},1606 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},482 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},745 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},1053 => ObjectInfo {
    place_offset_x: -7.5,
    place_offset_y: -7.5,
    tintable: true,
    solid: false,
    category: Utilities,
},1756 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1280 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},99 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},133 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},396 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},477 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},394 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1058 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},141 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},937 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1892 => ObjectInfo {
    place_offset_x: -7.5,
    place_offset_y: -9.75,
    tintable: true,
    solid: false,
    category: Spikes,
},49 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -2.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},395 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1757 => ObjectInfo {
    place_offset_x: -7.5,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},223 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},658 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1720 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -11.0,
    tintable: true,
    solid: false,
    category: Spikes,
},1766 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},470 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},154 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1731 => ObjectInfo {
    place_offset_x: -11.5,
    place_offset_y: -11.5,
    tintable: true,
    solid: false,
    category: Spikes,
},1697 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Utilities,
},1133 => ObjectInfo {
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
},695 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},11 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},20 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -2.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},1754 => ObjectInfo {
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
},646 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},503 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -5.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},1869 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1873 => ObjectInfo {
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
},1708 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1862 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1700 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Utilities,
},420 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -2.5,
    tintable: true,
    solid: false,
    category: GroundDeco,
},410 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1740 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},413 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1597 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},15 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 6.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},139 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1709 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},216 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Spikes,
},1210 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},1206 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},877 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},870 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},53 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},36 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},1600 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1866 => ObjectInfo {
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
},1830 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},478 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1724 => ObjectInfo {
    place_offset_x: 15.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Spikes,
},3 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Blocks,
},933 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},472 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},1582 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Utilities,
},1051 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -2.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},1020 => ObjectInfo {
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
},135 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -11.0,
    tintable: true,
    solid: false,
    category: Spikes,
},914 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1865 => ObjectInfo {
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
},18 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 4.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},7 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Blocks,
},1618 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Utilities,
},1274 => ObjectInfo {
    place_offset_x: 5.0,
    place_offset_y: -5.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},721 => ObjectInfo {
    place_offset_x: -11.5,
    place_offset_y: -11.5,
    tintable: true,
    solid: false,
    category: Deco,
},1867 => ObjectInfo {
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
},930 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1583 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Utilities,
},722 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},110 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 2.0,
    tintable: true,
    solid: false,
    category: Deco,
},1704 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},50 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},16 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -1.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},183 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1832 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},143 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},479 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1339 => ObjectInfo {
    place_offset_x: 15.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},1891 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -6.0,
    tintable: true,
    solid: false,
    category: Spikes,
},1247 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},60 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},1735 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1601 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},648 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},218 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -6.0,
    tintable: true,
    solid: false,
    category: Spikes,
},1331 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},918 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Utilities,
},1620 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},504 => ObjectInfo {
    place_offset_x: 5.0,
    place_offset_y: -5.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},469 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},140 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -13.0,
    tintable: false,
    solid: false,
    category: Utilities,
},1205 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},888 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},228 => ObjectInfo {
    place_offset_x: -7.5,
    place_offset_y: -7.5,
    tintable: true,
    solid: false,
    category: GroundDeco,
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
},1136 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1607 => ObjectInfo {
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
},6 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Blocks,
},12 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},101 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},1863 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},412 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},48 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 2.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},678 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1829 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},19 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 4.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},460 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},907 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -4.5,
    tintable: true,
    solid: false,
    category: Deco,
},1717 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Spikes,
},1835 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1758 => ObjectInfo {
    place_offset_x: -7.25,
    place_offset_y: 7.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},1768 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},699 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},878 => ObjectInfo {
    place_offset_x: 15.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},242 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},1052 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -2.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},1707 => ObjectInfo {
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
},155 => ObjectInfo {
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
},1699 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Utilities,
},1875 => ObjectInfo {
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
},1586 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Utilities,
},1706 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},185 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},641 => ObjectInfo {
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
},1705 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},87 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1753 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},474 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},934 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1277 => ObjectInfo {
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
},157 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -1.5,
    tintable: true,
    solid: false,
    category: GroundDeco,
},111 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},696 => ObjectInfo {
    place_offset_x: 15.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},643 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1844 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1721 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -11.0,
    tintable: true,
    solid: false,
    category: Spikes,
},1204 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},107 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 4.0,
    tintable: true,
    solid: false,
    category: Deco,
},1001 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},13 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},1603 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},419 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -2.5,
    tintable: true,
    solid: false,
    category: GroundDeco,
},1765 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},273 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1209 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},1519 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Utilities,
},1767 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},132 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},46 => ObjectInfo {
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
},1833 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1003 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},1741 => ObjectInfo {
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
},1883 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},697 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},1730 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -7.5,
    tintable: true,
    solid: false,
    category: Spikes,
},1135 => ObjectInfo {
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
},1060 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},51 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},408 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -12.5,
    tintable: true,
    solid: false,
    category: Deco,
},645 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},67 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -12.0,
    tintable: false,
    solid: false,
    category: Utilities,
},21 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -8.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},106 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 18.0,
    tintable: true,
    solid: false,
    category: Deco,
},882 => ObjectInfo {
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
},179 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -6.0,
    tintable: true,
    solid: false,
    category: Spikes,
},1722 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -11.0,
    tintable: true,
    solid: false,
    category: Spikes,
},701 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},647 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1022 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},158 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -1.5,
    tintable: true,
    solid: false,
    category: GroundDeco,
},1267 => ObjectInfo {
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
},694 => ObjectInfo {
    place_offset_x: 15.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Slopes,
},1332 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -12.5,
    tintable: false,
    solid: false,
    category: Utilities,
},52 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},184 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1021 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1203 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Outlines,
},217 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -9.0,
    tintable: true,
    solid: false,
    category: Spikes,
},186 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},660 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},411 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Deco,
},890 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1884 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1330 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: false,
    solid: false,
    category: Utilities,
},935 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},871 => ObjectInfo {
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
},1885 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1825 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1736 => ObjectInfo {
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
},1759 => ObjectInfo {
    place_offset_x: 10.5,
    place_offset_y: 9.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},649 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},494 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},1718 => ObjectInfo {
    place_offset_x: 15.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: true,
    category: Spikes,
},156 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},1266 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},406 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -8.0,
    tintable: true,
    solid: false,
    category: Deco,
},1878 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},1882 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Blocks,
},505 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: GroundDeco,
},1202 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 13.5,
    tintable: true,
    solid: true,
    category: Outlines,
},136 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},17 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: -8.0,
    tintable: true,
    solid: false,
    category: Pulsing,
},138 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    category: Saws,
},
        _ => return None,
    })
}
    


pub fn get_main_sprite(id: u32) -> Option<SpriteInfo> {
    Some(match id {
        21 => SpriteInfo {
    pos: (
        3680,
        1565,
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
},1584 => SpriteInfo {
    pos: (
        3662,
        391,
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
},154 => SpriteInfo {
    pos: (
        413,
        1468,
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
},1706 => SpriteInfo {
    pos: (
        3647,
        640,
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
},722 => SpriteInfo {
    pos: (
        887,
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
},1054 => SpriteInfo {
    pos: (
        1417,
        682,
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
},106 => SpriteInfo {
    pos: (
        261,
        742,
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
},1872 => SpriteInfo {
    pos: (
        1684,
        2191,
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
},473 => SpriteInfo {
    pos: (
        1801,
        1497,
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
},1620 => SpriteInfo {
    pos: (
        790,
        496,
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
},1879 => SpriteInfo {
    pos: (
        0,
        1214,
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
},159 => SpriteInfo {
    pos: (
        497,
        906,
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
},101 => SpriteInfo {
    pos: (
        3630,
        1085,
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
},494 => SpriteInfo {
    pos: (
        3115,
        1939,
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
},648 => SpriteInfo {
    pos: (
        1156,
        1781,
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
},211 => SpriteInfo {
    pos: (
        694,
        1468,
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
        2118,
        1614,
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
},273 => SpriteInfo {
    pos: (
        3086,
        837,
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
        1467,
        1000,
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
},143 => SpriteInfo {
    pos: (
        260,
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
},149 => SpriteInfo {
    pos: (
        540,
        2147,
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
},1022 => SpriteInfo {
    pos: (
        242,
        1826,
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
},158 => SpriteInfo {
    pos: (
        427,
        2017,
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
},135 => SpriteInfo {
    pos: (
        2981,
        554,
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
},1866 => SpriteInfo {
    pos: (
        0,
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
},698 => SpriteInfo {
    pos: (
        1579,
        670,
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
},1135 => SpriteInfo {
    pos: (
        3115,
        1761,
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
},1583 => SpriteInfo {
    pos: (
        3631,
        884,
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
},150 => SpriteInfo {
    pos: (
        81,
        742,
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
},1883 => SpriteInfo {
    pos: (
        438,
        2271,
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
        2734,
        260,
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
},907 => SpriteInfo {
    pos: (
        2356,
        1603,
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
},266 => SpriteInfo {
    pos: (
        1672,
        794,
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
},186 => SpriteInfo {
    pos: (
        3227,
        248,
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
},1060 => SpriteInfo {
    pos: (
        3115,
        1839,
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
},97 => SpriteInfo {
    pos: (
        3612,
        1771,
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
},1705 => SpriteInfo {
    pos: (
        818,
        1508,
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
},472 => SpriteInfo {
    pos: (
        1801,
        1565,
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
},185 => SpriteInfo {
    pos: (
        4015,
        822,
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
},10 => SpriteInfo {
    pos: (
        3256,
        1419,
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
},1730 => SpriteInfo {
    pos: (
        791,
        1840,
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
},645 => SpriteInfo {
    pos: (
        1135,
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
},184 => SpriteInfo {
    pos: (
        1886,
        1344,
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
},1867 => SpriteInfo {
    pos: (
        723,
        749,
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
        2593,
        1771,
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
},53 => SpriteInfo {
    pos: (
        1823,
        676,
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
},1755 => SpriteInfo {
    pos: (
        2675,
        1050,
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
        1548,
        794,
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
},1864 => SpriteInfo {
    pos: (
        1835,
        91,
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
        1814,
        1690,
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
},646 => SpriteInfo {
    pos: (
        2207,
        2104,
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
},1829 => SpriteInfo {
    pos: (
        2487,
        723,
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
},46 => SpriteInfo {
    pos: (
        0,
        1338,
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
},697 => SpriteInfo {
    pos: (
        1560,
        2079,
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
        3419,
        124,
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
},224 => SpriteInfo {
    pos: (
        1124,
        1052,
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
},1845 => SpriteInfo {
    pos: (
        3741,
        0,
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
},1731 => SpriteInfo {
    pos: (
        2593,
        1697,
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
},908 => SpriteInfo {
    pos: (
        694,
        1592,
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
},1278 => SpriteInfo {
    pos: (
        2693,
        926,
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
},1723 => SpriteInfo {
    pos: (
        1959,
        157,
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
},1756 => SpriteInfo {
    pos: (
        4074,
        833,
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
},1331 => SpriteInfo {
    pos: (
        2817,
        994,
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
},139 => SpriteInfo {
    pos: (
        1915,
        1114,
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
},1 => SpriteInfo {
    pos: (
        3161,
        713,
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
},470 => SpriteInfo {
    pos: (
        1684,
        2067,
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
},1882 => SpriteInfo {
    pos: (
        3855,
        887,
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
},1338 => SpriteInfo {
    pos: (
        2466,
        566,
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
},242 => SpriteInfo {
    pos: (
        2215,
        1980,
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
},1279 => SpriteInfo {
    pos: (
        2718,
        554,
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
},1888 => SpriteInfo {
    pos: (
        3836,
        1011,
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
},877 => SpriteInfo {
    pos: (
        3896,
        515,
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
},938 => SpriteInfo {
    pos: (
        575,
        701,
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
},227 => SpriteInfo {
    pos: (
        2609,
        255,
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
},1005 => SpriteInfo {
    pos: (
        1801,
        1510,
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
},1051 => SpriteInfo {
    pos: (
        1058,
        2247,
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
},115 => SpriteInfo {
    pos: (
        2130,
        1505,
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
},1058 => SpriteInfo {
    pos: (
        3804,
        1742,
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
},939 => SpriteInfo {
    pos: (
        612,
        308,
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
},138 => SpriteInfo {
    pos: (
        1211,
        2108,
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
},1205 => SpriteInfo {
    pos: (
        1798,
        1481,
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
},218 => SpriteInfo {
    pos: (
        1091,
        1423,
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
},680 => SpriteInfo {
    pos: (
        0,
        997,
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
},84 => SpriteInfo {
    pos: (
        2487,
        847,
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
},1020 => SpriteInfo {
    pos: (
        2797,
        0,
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
},481 => SpriteInfo {
    pos: (
        1459,
        437,
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
        869,
        1066,
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
        2107,
        281,
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
},36 => SpriteInfo {
    pos: (
        3896,
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
},937 => SpriteInfo {
    pos: (
        0,
        2120,
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
},1604 => SpriteInfo {
    pos: (
        1267,
        1957,
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
},1880 => SpriteInfo {
    pos: (
        1894,
        428,
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
},896 => SpriteInfo {
    pos: (
        1920,
        866,
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
},647 => SpriteInfo {
    pos: (
        1323,
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
},1619 => SpriteInfo {
    pos: (
        397,
        0,
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
},1832 => SpriteInfo {
    pos: (
        943,
        372,
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
},1021 => SpriteInfo {
    pos: (
        2858,
        2235,
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
},1767 => SpriteInfo {
    pos: (
        2155,
        1733,
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
},1892 => SpriteInfo {
    pos: (
        124,
        1267,
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
},702 => SpriteInfo {
    pos: (
        0,
        1702,
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
},1751 => SpriteInfo {
    pos: (
        2331,
        2197,
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
},882 => SpriteInfo {
    pos: (
        2918,
        1374,
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
},83 => SpriteInfo {
    pos: (
        1419,
        2154,
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
        3355,
        1444,
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
},880 => SpriteInfo {
    pos: (
        3132,
        1513,
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
        1507,
        1792,
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
},1004 => SpriteInfo {
    pos: (
        1560,
        2067,
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
},1877 => SpriteInfo {
    pos: (
        2171,
        2228,
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
},17 => SpriteInfo {
    pos: (
        4060,
        1440,
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
},11 => SpriteInfo {
    pos: (
        2242,
        1614,
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
},1715 => SpriteInfo {
    pos: (
        2961,
        779,
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
},1059 => SpriteInfo {
    pos: (
        869,
        952,
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
},476 => SpriteInfo {
    pos: (
        2175,
        1160,
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
        1770,
        215,
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
},408 => SpriteInfo {
    pos: (
        124,
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
},188 => SpriteInfo {
    pos: (
        3458,
        953,
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
},1332 => SpriteInfo {
    pos: (
        2175,
        1114,
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
},1210 => SpriteInfo {
    pos: (
        2351,
        442,
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
},15 => SpriteInfo {
    pos: (
        3601,
        484,
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
},1273 => SpriteInfo {
    pos: (
        148,
        1121,
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
},1721 => SpriteInfo {
    pos: (
        0,
        1145,
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
},1861 => SpriteInfo {
    pos: (
        480,
        523,
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
},414 => SpriteInfo {
    pos: (
        3680,
        1444,
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
},107 => SpriteInfo {
    pos: (
        177,
        1350,
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
},111 => SpriteInfo {
    pos: (
        1075,
        1905,
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
},35 => SpriteInfo {
    pos: (
        3612,
        1751,
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
},1280 => SpriteInfo {
    pos: (
        1762,
        1305,
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
},60 => SpriteInfo {
    pos: (
        3418,
        1125,
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
},874 => SpriteInfo {
    pos: (
        4020,
        502,
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
},148 => SpriteInfo {
    pos: (
        906,
        2073,
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
},1848 => SpriteInfo {
    pos: (
        2041,
        742,
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
},1710 => SpriteInfo {
    pos: (
        882,
        2187,
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
},871 => SpriteInfo {
    pos: (
        0,
        1826,
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
        3667,
        118,
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
},1724 => SpriteInfo {
    pos: (
        1490,
        1912,
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
},659 => SpriteInfo {
    pos: (
        2473,
        131,
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
},1698 => SpriteInfo {
    pos: (
        1835,
        0,
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
},155 => SpriteInfo {
    pos: (
        3227,
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
},458 => SpriteInfo {
    pos: (
        325,
        1554,
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
},1873 => SpriteInfo {
    pos: (
        1518,
        1621,
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
        887,
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
},1753 => SpriteInfo {
    pos: (
        1395,
        1369,
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
},50 => SpriteInfo {
    pos: (
        2465,
        1697,
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
        342,
        1143,
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
},641 => SpriteInfo {
    pos: (
        3294,
        1072,
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
        847,
        828,
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
        2487,
        1001,
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
},1605 => SpriteInfo {
    pos: (
        2611,
        929,
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
},1764 => SpriteInfo {
    pos: (
        2155,
        1749,
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
},133 => SpriteInfo {
    pos: (
        2155,
        1843,
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
},177 => SpriteInfo {
    pos: (
        887,
        124,
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
},86 => SpriteInfo {
    pos: (
        664,
        2152,
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
},1019 => SpriteInfo {
    pos: (
        3237,
        1877,
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
},889 => SpriteInfo {
    pos: (
        2385,
        1479,
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
},1050 => SpriteInfo {
    pos: (
        3899,
        0,
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
},1720 => SpriteInfo {
    pos: (
        2857,
        430,
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
},870 => SpriteInfo {
    pos: (
        416,
        2116,
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
        1091,
        1319,
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
},1844 => SpriteInfo {
    pos: (
        1707,
        552,
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
},396 => SpriteInfo {
    pos: (
        1427,
        1041,
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
},1708 => SpriteInfo {
    pos: (
        2774,
        1702,
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
},409 => SpriteInfo {
    pos: (
        148,
        997,
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
},85 => SpriteInfo {
    pos: (
        1007,
        769,
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
},932 => SpriteInfo {
    pos: (
        2609,
        403,
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
},41 => SpriteInfo {
    pos: (
        0,
        689,
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
},1833 => SpriteInfo {
    pos: (
        621,
        997,
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
},140 => SpriteInfo {
    pos: (
        2199,
        742,
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
},132 => SpriteInfo {
    pos: (
        2728,
        1333,
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
},1846 => SpriteInfo {
    pos: (
        2884,
        2045,
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
},504 => SpriteInfo {
    pos: (
        2611,
        847,
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
},1596 => SpriteInfo {
    pos: (
        1280,
        1539,
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
},1603 => SpriteInfo {
    pos: (
        3667,
        40,
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
},1740 => SpriteInfo {
    pos: (
        2611,
        723,
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
},1707 => SpriteInfo {
    pos: (
        916,
        1840,
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
},1704 => SpriteInfo {
    pos: (
        1695,
        0,
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
},929 => SpriteInfo {
    pos: (
        260,
        357,
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
},1247 => SpriteInfo {
    pos: (
        2103,
        442,
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
},87 => SpriteInfo {
    pos: (
        1241,
        1261,
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
},1003 => SpriteInfo {
    pos: (
        3477,
        885,
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
},259 => SpriteInfo {
    pos: (
        3477,
        761,
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
        2175,
        990,
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
        1998,
        304,
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
},1206 => SpriteInfo {
    pos: (
        1798,
        1465,
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
},1709 => SpriteInfo {
    pos: (
        1949,
        2033,
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
},187 => SpriteInfo {
    pos: (
        2339,
        1951,
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
},1735 => SpriteInfo {
    pos: (
        1280,
        1743,
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
},1136 => SpriteInfo {
    pos: (
        2599,
        527,
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
},1891 => SpriteInfo {
    pos: (
        3791,
        118,
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
},658 => SpriteInfo {
    pos: (
        551,
        2023,
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
},1754 => SpriteInfo {
    pos: (
        3612,
        1743,
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
},1834 => SpriteInfo {
    pos: (
        1319,
        1415,
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
},643 => SpriteInfo {
    pos: (
        108,
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
},482 => SpriteInfo {
    pos: (
        1195,
        1415,
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
        1012,
        124,
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
},51 => SpriteInfo {
    pos: (
        1874,
        304,
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
        3132,
        1419,
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
},1209 => SpriteInfo {
    pos: (
        0,
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
},1835 => SpriteInfo {
    pos: (
        1011,
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
},1718 => SpriteInfo {
    pos: (
        2059,
        587,
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
},136 => SpriteInfo {
    pos: (
        3754,
        1373,
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
        3240,
        1727,
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
},179 => SpriteInfo {
    pos: (
        2728,
        1174,
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
},413 => SpriteInfo {
    pos: (
        1211,
        1905,
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
},2 => SpriteInfo {
    pos: (
        3894,
        1373,
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
},919 => SpriteInfo {
    pos: (
        852,
        626,
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
},1871 => SpriteInfo {
    pos: (
        3334,
        948,
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
},467 => SpriteInfo {
    pos: (
        2713,
        678,
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
},1741 => SpriteInfo {
    pos: (
        3958,
        1801,
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
},411 => SpriteInfo {
    pos: (
        289,
        1464,
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
},1825 => SpriteInfo {
    pos: (
        244,
        1702,
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
},935 => SpriteInfo {
    pos: (
        2760,
        2045,
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
},1274 => SpriteInfo {
    pos: (
        4023,
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
},1001 => SpriteInfo {
    pos: (
        1796,
        800,
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
        675,
        2028,
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
},503 => SpriteInfo {
    pos: (
        1642,
        1621,
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
},13 => SpriteInfo {
    pos: (
        206,
        1008,
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
},1878 => SpriteInfo {
    pos: (
        2953,
        1250,
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
},52 => SpriteInfo {
    pos: (
        2107,
        157,
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
},869 => SpriteInfo {
    pos: (
        2710,
        2285,
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
},693 => SpriteInfo {
    pos: (
        3226,
        589,
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
        497,
        1007,
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
},719 => SpriteInfo {
    pos: (
        3046,
        961,
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
},1736 => SpriteInfo {
    pos: (
        2957,
        1543,
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
},217 => SpriteInfo {
    pos: (
        1646,
        163,
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
},1203 => SpriteInfo {
    pos: (
        373,
        771,
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
        1770,
        428,
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
},934 => SpriteInfo {
    pos: (
        2695,
        802,
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
        1464,
        746,
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
},881 => SpriteInfo {
    pos: (
        232,
        481,
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
        695,
        276,
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
},6 => SpriteInfo {
    pos: (
        3891,
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
},1597 => SpriteInfo {
    pos: (
        915,
        2001,
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
},1874 => SpriteInfo {
    pos: (
        3796,
        1249,
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
        0,
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
},1725 => SpriteInfo {
    pos: (
        2609,
        347,
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
},1267 => SpriteInfo {
    pos: (
        2586,
        2164,
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
        4020,
        754,
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
},1758 => SpriteInfo {
    pos: (
        612,
        400,
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
},395 => SpriteInfo {
    pos: (
        124,
        1948,
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
},1606 => SpriteInfo {
    pos: (
        4064,
        622,
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
},156 => SpriteInfo {
    pos: (
        2593,
        1903,
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
},696 => SpriteInfo {
    pos: (
        1290,
        876,
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
},734 => SpriteInfo {
    pos: (
        1571,
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
},1881 => SpriteInfo {
    pos: (
        2636,
        1939,
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
        3543,
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
},1134 => SpriteInfo {
    pos: (
        3630,
        976,
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
},1847 => SpriteInfo {
    pos: (
        3087,
        1005,
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
},480 => SpriteInfo {
    pos: (
        1860,
        1556,
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
        2597,
        131,
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
},890 => SpriteInfo {
    pos: (
        728,
        625,
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
},178 => SpriteInfo {
    pos: (
        364,
        1951,
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
},650 => SpriteInfo {
    pos: (
        3116,
        1637,
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
},1868 => SpriteInfo {
    pos: (
        1272,
        1137,
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
},1600 => SpriteInfo {
    pos: (
        1211,
        2009,
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
},1133 => SpriteInfo {
    pos: (
        567,
        1749,
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
},468 => SpriteInfo {
    pos: (
        1334,
        495,
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
},137 => SpriteInfo {
    pos: (
        1011,
        447,
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
},479 => SpriteInfo {
    pos: (
        2842,
        627,
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
},914 => SpriteInfo {
    pos: (
        799,
        2028,
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
},460 => SpriteInfo {
    pos: (
        3974,
        1621,
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
},12 => SpriteInfo {
    pos: (
        124,
        124,
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
},222 => SpriteInfo {
    pos: (
        1534,
        924,
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
},700 => SpriteInfo {
    pos: (
        567,
        1909,
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
},649 => SpriteInfo {
    pos: (
        149,
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
},1582 => SpriteInfo {
    pos: (
        2635,
        2063,
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
},1813 => SpriteInfo {
    pos: (
        1000,
        1052,
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
},1766 => SpriteInfo {
    pos: (
        2171,
        2033,
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
},1053 => SpriteInfo {
    pos: (
        124,
        1203,
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
},406 => SpriteInfo {
    pos: (
        2611,
        989,
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
},873 => SpriteInfo {
    pos: (
        2215,
        1916,
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
},660 => SpriteInfo {
    pos: (
        1813,
        2033,
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
},679 => SpriteInfo {
    pos: (
        2175,
        1284,
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
},1759 => SpriteInfo {
    pos: (
        1519,
        1429,
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
},1843 => SpriteInfo {
    pos: (
        3170,
        961,
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
},16 => SpriteInfo {
    pos: (
        2191,
        1733,
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
},1876 => SpriteInfo {
    pos: (
        1383,
        1495,
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
},419 => SpriteInfo {
    pos: (
        2953,
        1144,
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
},141 => SpriteInfo {
    pos: (
        2231,
        157,
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
},739 => SpriteInfo {
    pos: (
        852,
        704,
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
},1586 => SpriteInfo {
    pos: (
        384,
        308,
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
},114 => SpriteInfo {
    pos: (
        103,
        605,
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
},1722 => SpriteInfo {
    pos: (
        661,
        1170,
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
},216 => SpriteInfo {
    pos: (
        819,
        372,
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
},930 => SpriteInfo {
    pos: (
        2299,
        1059,
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
},18 => SpriteInfo {
    pos: (
        3077,
        1249,
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
},1885 => SpriteInfo {
    pos: (
        488,
        308,
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
},477 => SpriteInfo {
    pos: (
        993,
        1176,
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
        3896,
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
},1837 => SpriteInfo {
    pos: (
        791,
        1904,
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
},1052 => SpriteInfo {
    pos: (
        3477,
        589,
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
},1870 => SpriteInfo {
    pos: (
        1156,
        1657,
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
},745 => SpriteInfo {
    pos: (
        2039,
        990,
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
},157 => SpriteInfo {
    pos: (
        497,
        799,
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
},1768 => SpriteInfo {
    pos: (
        1798,
        1429,
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
},48 => SpriteInfo {
    pos: (
        2009,
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
},1330 => SpriteInfo {
    pos: (
        1984,
        1556,
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
},19 => SpriteInfo {
    pos: (
        3663,
        242,
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
},1765 => SpriteInfo {
    pos: (
        1750,
        1959,
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
},1890 => SpriteInfo {
    pos: (
        1335,
        437,
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
},1281 => SpriteInfo {
    pos: (
        2342,
        566,
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
},469 => SpriteInfo {
    pos: (
        604,
        577,
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
},1859 => SpriteInfo {
    pos: (
        1117,
        1195,
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
        2341,
        1821,
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
},228 => SpriteInfo {
    pos: (
        0,
        479,
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
},45 => SpriteInfo {
    pos: (
        2310,
        695,
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
},410 => SpriteInfo {
    pos: (
        2299,
        1183,
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
},1207 => SpriteInfo {
    pos: (
        2734,
        326,
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
},49 => SpriteInfo {
    pos: (
        2473,
        0,
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
},1333 => SpriteInfo {
    pos: (
        1334,
        746,
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
},1717 => SpriteInfo {
    pos: (
        2953,
        1002,
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
},883 => SpriteInfo {
    pos: (
        917,
        502,
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
},895 => SpriteInfo {
    pos: (
        2475,
        442,
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
},1863 => SpriteInfo {
    pos: (
        2857,
        503,
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
},7 => SpriteInfo {
    pos: (
        1935,
        552,
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
},695 => SpriteInfo {
    pos: (
        3419,
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
},1266 => SpriteInfo {
    pos: (
        1520,
        1305,
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
},916 => SpriteInfo {
    pos: (
        2155,
        1669,
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
},505 => SpriteInfo {
    pos: (
        3761,
        1890,
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
        893,
        1190,
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
},99 => SpriteInfo {
    pos: (
        3350,
        589,
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
},223 => SpriteInfo {
    pos: (
        661,
        1211,
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
},1757 => SpriteInfo {
    pos: (
        2611,
        981,
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
},47 => SpriteInfo {
    pos: (
        2638,
        1597,
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
},878 => SpriteInfo {
    pos: (
        2982,
        430,
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
        4020,
        622,
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
},1728 => SpriteInfo {
    pos: (
        81,
        922,
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
},917 => SpriteInfo {
    pos: (
        3707,
        0,
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
},471 => SpriteInfo {
    pos: (
        3488,
        1727,
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
        3761,
        2014,
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
},5 => SpriteInfo {
    pos: (
        2341,
        1697,
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
        943,
        436,
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
},67 => SpriteInfo {
    pos: (
        0,
        543,
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
},644 => SpriteInfo {
    pos: (
        1348,
        1957,
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
        4020,
        688,
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
},721 => SpriteInfo {
    pos: (
        2593,
        1731,
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
},1862 => SpriteInfo {
    pos: (
        1362,
        1619,
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
},1889 => SpriteInfo {
    pos: (
        3210,
        837,
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
},474 => SpriteInfo {
    pos: (
        4074,
        822,
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
},54 => SpriteInfo {
    pos: (
        2833,
        1498,
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
},1601 => SpriteInfo {
    pos: (
        1824,
        1429,
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
},110 => SpriteInfo {
    pos: (
        3566,
        344,
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
},1869 => SpriteInfo {
    pos: (
        2164,
        866,
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
},3 => SpriteInfo {
    pos: (
        1915,
        990,
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
},1202 => SpriteInfo {
    pos: (
        2175,
        1144,
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
},
        _ => return None,
    })
}
    


pub fn get_detail_sprite(id: u32) -> Option<SpriteInfo> {
    Some(match id {
        883 => SpriteInfo {
    pos: (
        0,
        571,
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
},1736 => SpriteInfo {
    pos: (
        177,
        1554,
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
},1005 => SpriteInfo {
    pos: (
        2593,
        1839,
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
},1582 => SpriteInfo {
    pos: (
        1472,
        2067,
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
},1281 => SpriteInfo {
    pos: (
        2085,
        566,
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
},869 => SpriteInfo {
    pos: (
        0,
        2287,
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
},642 => SpriteInfo {
    pos: (
        2199,
        765,
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
},1708 => SpriteInfo {
    pos: (
        1642,
        1702,
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
},918 => SpriteInfo {
    pos: (
        1750,
        1912,
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
},1881 => SpriteInfo {
    pos: (
        0,
        1950,
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
        4018,
        1564,
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
},1600 => SpriteInfo {
    pos: (
        322,
        2017,
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
},878 => SpriteInfo {
    pos: (
        488,
        432,
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
},648 => SpriteInfo {
    pos: (
        667,
        1785,
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
},1864 => SpriteInfo {
    pos: (
        3899,
        108,
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
},480 => SpriteInfo {
    pos: (
        4018,
        1440,
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
},1866 => SpriteInfo {
    pos: (
        260,
        248,
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
},1868 => SpriteInfo {
    pos: (
        1396,
        1137,
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
},1266 => SpriteInfo {
    pos: (
        1644,
        1305,
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
},481 => SpriteInfo {
    pos: (
        1583,
        441,
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
},1699 => SpriteInfo {
    pos: (
        3885,
        1890,
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
},739 => SpriteInfo {
    pos: (
        1947,
        676,
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
},479 => SpriteInfo {
    pos: (
        1334,
        629,
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
},409 => SpriteInfo {
    pos: (
        1427,
        1000,
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
},870 => SpriteInfo {
    pos: (
        3042,
        2119,
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
},896 => SpriteInfo {
    pos: (
        2819,
        876,
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
},1004 => SpriteInfo {
    pos: (
        3207,
        1939,
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
},1875 => SpriteInfo {
    pos: (
        451,
        647,
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
},1279 => SpriteInfo {
    pos: (
        1458,
        561,
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
},890 => SpriteInfo {
    pos: (
        2595,
        605,
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
},1698 => SpriteInfo {
    pos: (
        1915,
        0,
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
},477 => SpriteInfo {
    pos: (
        3754,
        1085,
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
},1620 => SpriteInfo {
    pos: (
        4020,
        391,
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
},478 => SpriteInfo {
    pos: (
        2721,
        131,
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
},1885 => SpriteInfo {
    pos: (
        1646,
        317,
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
},1863 => SpriteInfo {
    pos: (
        1334,
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
},1876 => SpriteInfo {
    pos: (
        3894,
        1497,
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
},882 => SpriteInfo {
    pos: (
        1395,
        1377,
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
},679 => SpriteInfo {
    pos: (
        893,
        1300,
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
},871 => SpriteInfo {
    pos: (
        124,
        1826,
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
},1054 => SpriteInfo {
    pos: (
        3477,
        697,
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
},680 => SpriteInfo {
    pos: (
        1290,
        1000,
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
},880 => SpriteInfo {
    pos: (
        1156,
        1539,
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
},1874 => SpriteInfo {
    pos: (
        3920,
        1249,
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
        3667,
        0,
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
        991,
        2001,
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
},873 => SpriteInfo {
    pos: (
        3957,
        1925,
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
},881 => SpriteInfo {
    pos: (
        356,
        481,
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
},1879 => SpriteInfo {
    pos: (
        1915,
        1226,
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
},1583 => SpriteInfo {
    pos: (
        3752,
        884,
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
},646 => SpriteInfo {
    pos: (
        322,
        2102,
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
},1865 => SpriteInfo {
    pos: (
        3364,
        1727,
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
},647 => SpriteInfo {
    pos: (
        1447,
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
},1878 => SpriteInfo {
    pos: (
        1396,
        1251,
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
},1586 => SpriteInfo {
    pos: (
        3566,
        248,
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
},217 => SpriteInfo {
    pos: (
        3227,
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
},1003 => SpriteInfo {
    pos: (
        3601,
        656,
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
},1880 => SpriteInfo {
    pos: (
        2733,
        430,
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
        342,
        1019,
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
},1735 => SpriteInfo {
    pos: (
        368,
        1749,
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
},1877 => SpriteInfo {
    pos: (
        314,
        2240,
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
        1419,
        2278,
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
},482 => SpriteInfo {
    pos: (
        3008,
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
},1619 => SpriteInfo {
    pos: (
        695,
        0,
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
},1002 => SpriteInfo {
    pos: (
        976,
        650,
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
},1697 => SpriteInfo {
    pos: (
        2774,
        1616,
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
},678 => SpriteInfo {
    pos: (
        2423,
        1174,
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
},1001 => SpriteInfo {
    pos: (
        976,
        626,
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
},1869 => SpriteInfo {
    pos: (
        621,
        873,
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
        3752,
        969,
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
},1872 => SpriteInfo {
    pos: (
        1543,
        2203,
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
},939 => SpriteInfo {
    pos: (
        2734,
        214,
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
},1862 => SpriteInfo {
    pos: (
        3680,
        1619,
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
        2593,
        1863,
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
},889 => SpriteInfo {
    pos: (
        2629,
        1479,
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
},649 => SpriteInfo {
    pos: (
        273,
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
},874 => SpriteInfo {
    pos: (
        4020,
        566,
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
},643 => SpriteInfo {
    pos: (
        2018,
        379,
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
},1053 => SpriteInfo {
    pos: (
        2728,
        1258,
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
},645 => SpriteInfo {
    pos: (
        1259,
        0,
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
},641 => SpriteInfo {
    pos: (
        745,
        1076,
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
},1882 => SpriteInfo {
    pos: (
        373,
        895,
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
        1646,
        142,
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
},1867 => SpriteInfo {
    pos: (
        2837,
        751,
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
},877 => SpriteInfo {
    pos: (
        2018,
        473,
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
},1267 => SpriteInfo {
    pos: (
        2740,
        2169,
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
},411 => SpriteInfo {
    pos: (
        4044,
        1249,
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
},1861 => SpriteInfo {
    pos: (
        1583,
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
},1247 => SpriteInfo {
    pos: (
        2227,
        442,
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
},1700 => SpriteInfo {
    pos: (
        1646,
        221,
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
},216 => SpriteInfo {
    pos: (
        0,
        372,
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
},218 => SpriteInfo {
    pos: (
        4018,
        1373,
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
},895 => SpriteInfo {
    pos: (
        2227,
        463,
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
},1870 => SpriteInfo {
    pos: (
        694,
        1661,
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
},410 => SpriteInfo {
    pos: (
        3796,
        1085,
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
},1843 => SpriteInfo {
    pos: (
        2487,
        971,
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
},1871 => SpriteInfo {
    pos: (
        745,
        952,
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
        488,
        1951,
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
},1601 => SpriteInfo {
    pos: (
        1507,
        1377,
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
},1873 => SpriteInfo {
    pos: (
        3850,
        1621,
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
},1734 => SpriteInfo {
    pos: (
        1335,
        124,
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
},1710 => SpriteInfo {
    pos: (
        3166,
        2119,
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
},
        _ => return None,
    })
}
    

    