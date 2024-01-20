
use the_nexus::objects::{ObjectCategory::*, ObjectInfo};
use the_nexus::packing::SpriteInfo;


pub fn get_object_info(id: u32) -> Option<ObjectInfo> {
    Some(match id {
        2557 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},190 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2654 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},201 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},4227 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 30.0,
    category: Pixel,
},2385 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3372 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3000 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},4001 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4208 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4000 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},111 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},3848 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3439 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4006 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2480 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2545 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4007 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1863 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},1867 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},8 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Spikes,
},3576 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},200 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},1763 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1846 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1587 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},2475 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2944 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3997 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2638 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2361 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},35 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},4004 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3478 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2419 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 3.333,
    category: Pixel,
},3547 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.5,
    category: Pixel,
},2656 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},472 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},67 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},3931 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},216 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Spikes,
},3444 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.5,
    category: Pixel,
},1600 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},4323 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3531 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2577 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},10 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},4177 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2548 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2645 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},696 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},211 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3808 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},18 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1706 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},4095 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2229 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},130 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3222 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1734 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},1705 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},3856 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Pixel,
},86 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},2946 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3996 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1766 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2510 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3983 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3803 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1765 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},99 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},2692 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},1330 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},1210 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},2554 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2644 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},6 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},504 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1834 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},1859 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},1849 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},152 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},940 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2063 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},2543 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2572 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3536 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4087 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3944 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3622 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3088 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1758 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},695 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3635 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3425 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2521 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.25,
    category: Pixel,
},3368 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1292 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1224 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},234 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1703 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},2024 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},1836 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2587 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4173 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2667 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3809 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},41 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3549 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.5,
    category: Pixel,
},414 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1697 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},11 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},2862 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3312 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3129 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2483 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3535 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4356 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2632 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3601 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},4230 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3577 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4187 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3552 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.5,
    category: Pixel,
},4148 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1862 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},1866 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},1864 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},663 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},3939 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},394 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},2651 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},1833 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},4150 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2668 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3054 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3001 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},1884 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3480 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3621 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3481 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3554 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.5,
    category: Pixel,
},266 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},1339 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},3857 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Pixel,
},3955 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},4185 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3227 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3087 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},908 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2313 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2518 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.25,
    category: Pixel,
},1605 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3402 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3089 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2464 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4377 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1011 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1845 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2665 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},4134 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2542 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3058 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},471 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},2478 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3977 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3308 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2698 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3999 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3520 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1328 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},2540 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},259 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},1583 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},473 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},3861 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Pixel,
},2491 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2585 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3935 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3639 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1589 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},1754 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2224 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3329 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3311 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2522 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.25,
    category: Pixel,
},1698 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},2553 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4142 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2652 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3855 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Pixel,
},1222 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},3362 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2517 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.25,
    category: Pixel,
},3969 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},218 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Spikes,
},3907 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3046 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},1345 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},699 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3814 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1332 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},4189 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3427 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3941 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},407 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2473 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2028 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},3633 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3122 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2546 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2567 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3221 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2576 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1340 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},4143 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2310 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 3.333,
    category: Pixel,
},1751 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},2589 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2693 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1206 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},3375 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},39 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Spikes,
},1711 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Spikes,
},3503 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.308,
    category: Pixel,
},2948 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},1712 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Spikes,
},2490 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3860 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Pixel,
},2471 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2026 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},2499 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2360 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2658 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},468 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},2564 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3228 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1714 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Spikes,
},2631 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3995 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3652 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1009 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3145 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2393 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3230 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},5 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},1054 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},3631 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3637 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2418 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},4068 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2373 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3321 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3553 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.5,
    category: Pixel,
},2512 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.5,
    category: Pixel,
},1333 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},3862 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Pixel,
},1598 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},1209 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},1220 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},2994 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},662 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},85 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},2694 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 0.909,
    category: OrbsAndGlorbs,
},3318 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3151 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2560 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3548 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.5,
    category: Pixel,
},4372 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2550 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1338 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},3309 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1753 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1837 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1757 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1597 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2679 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3123 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3534 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2472 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1838 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2886 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},2686 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},4371 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},187 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},3144 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2603 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2367 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4300 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.5,
    category: OrbsAndGlorbs,
},4066 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2987 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},2414 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3636 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3551 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.5,
    category: Pixel,
},2674 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2477 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3450 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.5,
    category: Pixel,
},907 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},4368 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3443 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.5,
    category: Pixel,
},2479 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1584 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},2588 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3852 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Pixel,
},702 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},2672 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2515 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.25,
    category: Pixel,
},2549 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 30.02,
    category: Pixel,
},3323 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2962 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3962 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3812 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2563 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2422 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 3.333,
    category: Pixel,
},3945 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},137 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},4072 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2684 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2462 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2500 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3625 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2681 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2368 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2677 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2592 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1342 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},2468 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1223 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},3634 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3465 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2961 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},4190 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2503 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2671 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3220 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2566 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2559 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2857 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},2867 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},4357 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},470 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},411 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2362 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2354 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},1732 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Spikes,
},2945 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3847 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},4172 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2947 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},2565 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2012 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},409 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},918 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},939 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},188 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},3324 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},410 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3627 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2032 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},1053 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},2889 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},4188 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2649 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2650 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2227 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3446 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.5,
    category: Pixel,
},2523 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.25,
    category: Pixel,
},4164 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2990 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},1726 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Spikes,
},3897 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3650 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2538 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2507 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3149 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2467 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3988 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1830 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},106 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3501 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.308,
    category: Pixel,
},1852 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},2675 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},4005 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2556 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2583 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3863 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Pixel,
},3479 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3853 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Pixel,
},3247 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2689 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},930 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3807 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3804 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3578 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3366 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3647 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1327 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},4369 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3629 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},19 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3801 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1759 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3811 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2226 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4375 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1205 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},2249 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},103 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Spikes,
},1708 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},4065 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3993 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},153 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2627 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},1462 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},505 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},697 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3332 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},87 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},3150 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4374 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1607 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3365 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1331 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},1835 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2506 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1461 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},458 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Spikes,
},2520 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.25,
    category: Pixel,
},2278 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2568 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2416 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2413 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},1221 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},3946 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},131 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2488 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4382 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2891 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},745 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},3233 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4098 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2513 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.5,
    category: Pixel,
},1203 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},2584 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3373 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},469 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},3653 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3296 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3440 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1012 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},928 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},1831 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},4071 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1847 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2682 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3521 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2476 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1716 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Spikes,
},3424 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3850 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3376 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3998 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1601 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2575 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4169 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4064 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2154 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.765,
    category: Pixel,
},4325 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3234 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2647 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},4324 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1227 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},2574 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1713 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Spikes,
},4076 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3229 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1735 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},2988 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},217 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Spikes,
},4174 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3854 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Pixel,
},2531 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.5,
    category: Pixel,
},891 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},4 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},1344 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},3555 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.5,
    category: Pixel,
},474 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},3648 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},4228 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3530 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3322 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3319 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1764 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2688 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2666 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2673 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2498 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1768 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3477 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3518 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3295 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2474 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3371 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2676 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2420 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 3.333,
    category: Pixel,
},3447 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.5,
    category: Pixel,
},1844 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2643 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2505 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2555 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},20 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1334 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},3523 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},203 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},3516 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2640 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3426 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3519 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4085 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3152 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},927 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},4097 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3146 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1885 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},2963 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},929 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},392 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Spikes,
},1829 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},3042 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},4178 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},934 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},2469 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3086 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},661 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},3942 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},4370 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},110 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3367 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2579 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},273 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},138 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},3582 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2547 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},129 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2697 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},413 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2642 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3002 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},3464 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4229 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2470 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2680 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3630 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},4135 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4175 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3526 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4376 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3226 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2466 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2481 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2637 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2573 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3056 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3057 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3994 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3626 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3965 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2561 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2949 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3984 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2502 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1865 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3527 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2591 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},942 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2552 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},83 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3975 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1058 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},3879 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2661 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3529 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4165 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3796 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3502 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.308,
    category: Pixel,
},3232 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3361 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1582 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},2023 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},4070 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},467 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},2551 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3901 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1709 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},1832 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},2544 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2648 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},1861 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},2027 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},3310 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2696 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2660 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2580 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1707 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},143 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3810 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3370 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},12 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},877 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},2417 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},693 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},503 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3628 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},933 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},4231 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2664 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3455 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.5,
    category: Pixel,
},2866 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},3851 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},36 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},3795 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4191 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2570 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1720 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Spikes,
},1275 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},2514 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.25,
    category: Pixel,
},3528 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2465 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2653 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},664 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},2225 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3544 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.5,
    category: Pixel,
},2558 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2519 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.25,
    category: Pixel,
},2699 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3331 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},151 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2657 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},4233 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},84 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},3360 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3525 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2605 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: OrbsAndGlorbs,
},2894 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},4003 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2353 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3546 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.5,
    category: Pixel,
},4002 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1730 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Spikes,
},3858 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Pixel,
},1614 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},700 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},2655 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3374 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3651 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Pixel,
},1341 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},3124 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4170 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2646 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3933 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3910 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1596 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3147 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3859 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Pixel,
},3325 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2511 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.5,
    category: Pixel,
},2363 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3231 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1204 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},2055 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},7 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},694 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},1225 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},1704 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},1604 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2691 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2482 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1464 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3624 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1702 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},4379 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3369 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2366 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},13 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},2639 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},4381 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2669 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2571 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3794 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3135 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2703 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3320 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1813 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},1343 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},1767 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3330 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2943 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},235 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2508 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},910 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1059 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},698 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},878 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3476 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2663 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},1463 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},2662 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},4353 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.5,
    category: Pixel,
},2659 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2993 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3131 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2991 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3902 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},4378 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3979 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3817 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},412 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2582 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3537 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},906 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1602 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2581 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},932 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3125 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2489 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2598 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4168 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1728 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Spikes,
},3314 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2251 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2586 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2250 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3623 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3524 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2641 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},107 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2463 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},140 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},2578 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4226 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3130 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1701 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},1 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},1606 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},1519 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},3522 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},202 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},3005 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},2501 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3943 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},395 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},3294 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3448 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.5,
    category: Pixel,
},3004 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},2670 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3646 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2683 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2704 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3849 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3638 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3806 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},47 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},3805 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},134 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3654 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2700 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},1022 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},2541 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2355 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},701 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},2690 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2516 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.25,
    category: Pixel,
},2926 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},2887 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},2300 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},4380 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2301 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},2279 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2590 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3632 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},3138 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2964 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},1699 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},2412 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3136 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3403 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1868 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3517 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3545 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.5,
    category: Pixel,
},2504 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2421 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 3.333,
    category: Pixel,
},2228 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1207 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Outlines,
},2569 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1722 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Spikes,
},1721 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Spikes,
},3550 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.5,
    category: Pixel,
},1591 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},2562 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3423 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4086 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2996 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},3802 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},4069 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2312 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},186 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},1752 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},4067 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1736 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Saws,
},3363 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3137 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4096 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3533 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},1729 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Spikes,
},3055 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},1725 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Spikes,
},2509 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2687 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3148 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2685 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3532 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4149 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2858 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Blocks,
},1888 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2411 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},4373 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},101 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},141 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: OrbsAndGlorbs,
},4151 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},4171 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3313 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},2678 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},3404 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.875,
    category: Pixel,
},3364 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 30.0,
    category: Pixel,
},1762 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 1.0,
    category: Deco,
},2311 => ObjectInfo {
    place_offset_x: 0.0,
    place_offset_y: 0.0,
    tintable: true,
    solid: false,
    builtin_scale: 2.0,
    category: Pixel,
},
        _ => return None,
    })
}
    


pub fn get_main_sprite(id: u32) -> Option<SpriteInfo> {
    Some(match id {
        1702 => SpriteInfo {
    pos: (
        1889,
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
},41 => SpriteInfo {
    pos: (
        2523,
        1514,
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
},3808 => SpriteInfo {
    pos: (
        3743,
        256,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},86 => SpriteInfo {
    pos: (
        443,
        2036,
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
},3374 => SpriteInfo {
    pos: (
        2229,
        2235,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2023 => SpriteInfo {
    pos: (
        1603,
        1091,
    ),
    size: (
        209,
        61,
    ),
    rotated: false,
    offset: (
        -1.5,
        2.5,
    ),
},2561 => SpriteInfo {
    pos: (
        449,
        132,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3977 => SpriteInfo {
    pos: (
        659,
        355,
    ),
    size: (
        126,
        127,
    ),
    rotated: false,
    offset: (
        -1.0,
        -0.5,
    ),
},3296 => SpriteInfo {
    pos: (
        1575,
        828,
    ),
    size: (
        24,
        20,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1708 => SpriteInfo {
    pos: (
        1089,
        148,
    ),
    size: (
        338,
        340,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3650 => SpriteInfo {
    pos: (
        1853,
        2100,
    ),
    size: (
        64,
        58,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1022 => SpriteInfo {
    pos: (
        1345,
        1930,
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
},933 => SpriteInfo {
    pos: (
        1853,
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
},700 => SpriteInfo {
    pos: (
        1582,
        1156,
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
},3230 => SpriteInfo {
    pos: (
        1395,
        2360,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2592 => SpriteInfo {
    pos: (
        2229,
        2303,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1862 => SpriteInfo {
    pos: (
        1471,
        1599,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4325 => SpriteInfo {
    pos: (
        1611,
        375,
    ),
    size: (
        64,
        44,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2310 => SpriteInfo {
    pos: (
        2260,
        1957,
    ),
    size: (
        36,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1224 => SpriteInfo {
    pos: (
        3707,
        172,
    ),
    size: (
        24,
        24,
    ),
    rotated: false,
    offset: (
        -48.0,
        -48.0,
    ),
},2664 => SpriteInfo {
    pos: (
        2411,
        1902,
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
},3056 => SpriteInfo {
    pos: (
        3947,
        1566,
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
},2564 => SpriteInfo {
    pos: (
        1475,
        936,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2519 => SpriteInfo {
    pos: (
        287,
        916,
    ),
    size: (
        8,
        92,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},2414 => SpriteInfo {
    pos: (
        1263,
        492,
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
},1867 => SpriteInfo {
    pos: (
        3039,
        1158,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2229 => SpriteInfo {
    pos: (
        1142,
        2253,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3149 => SpriteInfo {
    pos: (
        413,
        1442,
    ),
    size: (
        52,
        64,
    ),
    rotated: false,
    offset: (
        6.0,
        0.0,
    ),
},2688 => SpriteInfo {
    pos: (
        1786,
        2612,
    ),
    size: (
        136,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3544 => SpriteInfo {
    pos: (
        3215,
        1453,
    ),
    size: (
        80,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        12.0,
    ),
},1832 => SpriteInfo {
    pos: (
        2325,
        0,
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
},2154 => SpriteInfo {
    pos: (
        2826,
        1372,
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
},1721 => SpriteInfo {
    pos: (
        3939,
        1630,
    ),
    size: (
        121,
        66,
    ),
    rotated: false,
    offset: (
        -0.5,
        19.0,
    ),
},1844 => SpriteInfo {
    pos: (
        2800,
        2142,
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
},4370 => SpriteInfo {
    pos: (
        3342,
        1940,
    ),
    size: (
        56,
        8,
    ),
    rotated: false,
    offset: (
        0.0,
        -24.0,
    ),
},3623 => SpriteInfo {
    pos: (
        2226,
        1613,
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
},235 => SpriteInfo {
    pos: (
        504,
        1307,
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
},2362 => SpriteInfo {
    pos: (
        3092,
        2142,
    ),
    size: (
        48,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2024 => SpriteInfo {
    pos: (
        0,
        1282,
    ),
    size: (
        169,
        156,
    ),
    rotated: false,
    offset: (
        -4.5,
        -9.0,
    ),
},3322 => SpriteInfo {
    pos: (
        2107,
        839,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3984 => SpriteInfo {
    pos: (
        1209,
        0,
    ),
    size: (
        125,
        124,
    ),
    rotated: false,
    offset: (
        1.5,
        2.0,
    ),
},1601 => SpriteInfo {
    pos: (
        2685,
        855,
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
},3148 => SpriteInfo {
    pos: (
        1779,
        128,
    ),
    size: (
        44,
        36,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3628 => SpriteInfo {
    pos: (
        2437,
        204,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4353 => SpriteInfo {
    pos: (
        1497,
        0,
    ),
    size: (
        108,
        108,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1220 => SpriteInfo {
    pos: (
        2747,
        759,
    ),
    size: (
        120,
        24,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3465 => SpriteInfo {
    pos: (
        1137,
        1283,
    ),
    size: (
        48,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1762 => SpriteInfo {
    pos: (
        694,
        1981,
    ),
    size: (
        203,
        203,
    ),
    rotated: false,
    offset: (
        0.5,
        0.5,
    ),
},1596 => SpriteInfo {
    pos: (
        1762,
        236,
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
},3247 => SpriteInfo {
    pos: (
        1395,
        2220,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2591 => SpriteInfo {
    pos: (
        188,
        1512,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2361 => SpriteInfo {
    pos: (
        3402,
        2280,
    ),
    size: (
        56,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3147 => SpriteInfo {
    pos: (
        408,
        1788,
    ),
    size: (
        44,
        64,
    ),
    rotated: false,
    offset: (
        -10.0,
        0.0,
    ),
},1275 => SpriteInfo {
    pos: (
        2949,
        0,
    ),
    size: (
        120,
        84,
    ),
    rotated: false,
    offset: (
        0.0,
        7.0,
    ),
},3638 => SpriteInfo {
    pos: (
        3217,
        2410,
    ),
    size: (
        160,
        160,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2466 => SpriteInfo {
    pos: (
        3633,
        2489,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1751 => SpriteInfo {
    pos: (
        0,
        2257,
    ),
    size: (
        135,
        138,
    ),
    rotated: false,
    offset: (
        9.5,
        0.0,
    ),
},2669 => SpriteInfo {
    pos: (
        3901,
        1902,
    ),
    size: (
        60,
        4,
    ),
    rotated: false,
    offset: (
        0.0,
        24.0,
    ),
},3125 => SpriteInfo {
    pos: (
        564,
        604,
    ),
    size: (
        52,
        64,
    ),
    rotated: false,
    offset: (
        6.0,
        0.0,
    ),
},698 => SpriteInfo {
    pos: (
        3517,
        1751,
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
},1765 => SpriteInfo {
    pos: (
        3707,
        136,
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
},2471 => SpriteInfo {
    pos: (
        1797,
        0,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3131 => SpriteInfo {
    pos: (
        3302,
        1389,
    ),
    size: (
        40,
        48,
    ),
    rotated: false,
    offset: (
        -8.0,
        0.0,
    ),
},2569 => SpriteInfo {
    pos: (
        4087,
        1090,
    ),
    size: (
        4,
        4,
    ),
    rotated: false,
    offset: (
        -30.0,
        -26.0,
    ),
},4373 => SpriteInfo {
    pos: (
        2427,
        864,
    ),
    size: (
        56,
        60,
    ),
    rotated: false,
    offset: (
        4.0,
        -2.0,
    ),
},2886 => SpriteInfo {
    pos: (
        2970,
        1417,
    ),
    size: (
        241,
        39,
    ),
    rotated: false,
    offset: (
        1.5,
        12.5,
    ),
},1831 => SpriteInfo {
    pos: (
        3854,
        396,
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
},504 => SpriteInfo {
    pos: (
        208,
        276,
    ),
    size: (
        79,
        79,
    ),
    rotated: false,
    offset: (
        0.5,
        0.5,
    ),
},3365 => SpriteInfo {
    pos: (
        1744,
        1521,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3999 => SpriteInfo {
    pos: (
        212,
        1077,
    ),
    size: (
        64,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},702 => SpriteInfo {
    pos: (
        824,
        930,
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
},3517 => SpriteInfo {
    pos: (
        2271,
        500,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3546 => SpriteInfo {
    pos: (
        2547,
        2332,
    ),
    size: (
        76,
        56,
    ),
    rotated: false,
    offset: (
        2.0,
        12.0,
    ),
},3464 => SpriteInfo {
    pos: (
        3302,
        1441,
    ),
    size: (
        32,
        40,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2032 => SpriteInfo {
    pos: (
        344,
        2056,
    ),
    size: (
        92,
        98,
    ),
    rotated: false,
    offset: (
        1.0,
        0.0,
    ),
},4007 => SpriteInfo {
    pos: (
        2035,
        1243,
    ),
    size: (
        60,
        20,
    ),
    rotated: false,
    offset: (
        -2.0,
        -18.0,
    ),
},503 => SpriteInfo {
    pos: (
        3753,
        2217,
    ),
    size: (
        120,
        78,
    ),
    rotated: false,
    offset: (
        0.0,
        1.0,
    ),
},1463 => SpriteInfo {
    pos: (
        1587,
        2177,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4164 => SpriteInfo {
    pos: (
        2597,
        2525,
    ),
    size: (
        68,
        68,
    ),
    rotated: false,
    offset: (
        -50.0,
        0.0,
    ),
},2225 => SpriteInfo {
    pos: (
        392,
        1680,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2655 => SpriteInfo {
    pos: (
        1435,
        1895,
    ),
    size: (
        52,
        20,
    ),
    rotated: false,
    offset: (
        0.0,
        -16.0,
    ),
},1766 => SpriteInfo {
    pos: (
        3655,
        1438,
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
},3653 => SpriteInfo {
    pos: (
        783,
        0,
    ),
    size: (
        64,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2659 => SpriteInfo {
    pos: (
        1847,
        68,
    ),
    size: (
        4,
        52,
    ),
    rotated: false,
    offset: (
        -24.0,
        -4.0,
    ),
},3794 => SpriteInfo {
    pos: (
        0,
        1007,
    ),
    size: (
        60,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3370 => SpriteInfo {
    pos: (
        60,
        1214,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1462 => SpriteInfo {
    pos: (
        2339,
        448,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4148 => SpriteInfo {
    pos: (
        789,
        398,
    ),
    size: (
        80,
        36,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4068 => SpriteInfo {
    pos: (
        3995,
        128,
    ),
    size: (
        64,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        6.0,
    ),
},3532 => SpriteInfo {
    pos: (
        2342,
        1344,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1331 => SpriteInfo {
    pos: (
        3494,
        512,
    ),
    size: (
        132,
        337,
    ),
    rotated: false,
    offset: (
        23.0,
        0.5,
    ),
},3548 => SpriteInfo {
    pos: (
        3381,
        2536,
    ),
    size: (
        80,
        80,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3088 => SpriteInfo {
    pos: (
        3483,
        1173,
    ),
    size: (
        116,
        79,
    ),
    rotated: false,
    offset: (
        0.0,
        0.5,
    ),
},2511 => SpriteInfo {
    pos: (
        1743,
        313,
    ),
    size: (
        100,
        80,
    ),
    rotated: false,
    offset: (
        2.0,
        0.0,
    ),
},2498 => SpriteInfo {
    pos: (
        1073,
        1391,
    ),
    size: (
        56,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        8.0,
    ),
},4095 => SpriteInfo {
    pos: (
        973,
        554,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3151 => SpriteInfo {
    pos: (
        315,
        2551,
    ),
    size: (
        64,
        40,
    ),
    rotated: false,
    offset: (
        0.0,
        12.0,
    ),
},2420 => SpriteInfo {
    pos: (
        0,
        2058,
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
},1713 => SpriteInfo {
    pos: (
        3475,
        2352,
    ),
    size: (
        154,
        198,
    ),
    rotated: false,
    offset: (
        -7.0,
        -47.0,
    ),
},4067 => SpriteInfo {
    pos: (
        2547,
        2392,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1205 => SpriteInfo {
    pos: (
        2721,
        601,
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
},3939 => SpriteInfo {
    pos: (
        1105,
        2481,
    ),
    size: (
        56,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1768 => SpriteInfo {
    pos: (
        2721,
        657,
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
},3601 => SpriteInfo {
    pos: (
        256,
        1538,
    ),
    size: (
        104,
        104,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},2491 => SpriteInfo {
    pos: (
        3163,
        1150,
    ),
    size: (
        64,
        40,
    ),
    rotated: false,
    offset: (
        0.0,
        -12.0,
    ),
},2996 => SpriteInfo {
    pos: (
        1786,
        2422,
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
},4071 => SpriteInfo {
    pos: (
        173,
        1293,
    ),
    size: (
        40,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2656 => SpriteInfo {
    pos: (
        3486,
        941,
    ),
    size: (
        4,
        52,
    ),
    rotated: false,
    offset: (
        -24.0,
        4.0,
    ),
},930 => SpriteInfo {
    pos: (
        248,
        448,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4189 => SpriteInfo {
    pos: (
        1853,
        675,
    ),
    size: (
        40,
        44,
    ),
    rotated: false,
    offset: (
        0.0,
        -10.0,
    ),
},2251 => SpriteInfo {
    pos: (
        1133,
        1335,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3639 => SpriteInfo {
    pos: (
        1474,
        112,
    ),
    size: (
        160,
        160,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2681 => SpriteInfo {
    pos: (
        1816,
        1135,
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
},4086 => SpriteInfo {
    pos: (
        861,
        2663,
    ),
    size: (
        36,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3234 => SpriteInfo {
    pos: (
        1641,
        1889,
    ),
    size: (
        72,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2507 => SpriteInfo {
    pos: (
        793,
        2663,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3227 => SpriteInfo {
    pos: (
        0,
        1067,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4379 => SpriteInfo {
    pos: (
        3449,
        2410,
    ),
    size: (
        20,
        12,
    ),
    rotated: false,
    offset: (
        -22.0,
        -26.0,
    ),
},2416 => SpriteInfo {
    pos: (
        2278,
        1356,
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
},140 => SpriteInfo {
    pos: (
        1921,
        2133,
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
},2631 => SpriteInfo {
    pos: (
        291,
        302,
    ),
    size: (
        60,
        64,
    ),
    rotated: false,
    offset: (
        2.0,
        0.0,
    ),
},3002 => SpriteInfo {
    pos: (
        304,
        0,
    ),
    size: (
        120,
        33,
    ),
    rotated: false,
    offset: (
        0.0,
        -3.5,
    ),
},2676 => SpriteInfo {
    pos: (
        139,
        2382,
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
},1 => SpriteInfo {
    pos: (
        2399,
        1514,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2538 => SpriteInfo {
    pos: (
        2299,
        952,
    ),
    size: (
        60,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},8 => SpriteInfo {
    pos: (
        1998,
        1267,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2510 => SpriteInfo {
    pos: (
        3055,
        910,
    ),
    size: (
        64,
        36,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},1703 => SpriteInfo {
    pos: (
        1873,
        1913,
    ),
    size: (
        98,
        85,
    ),
    rotated: false,
    offset: (
        0.0,
        -6.5,
    ),
},3897 => SpriteInfo {
    pos: (
        3662,
        2228,
    ),
    size: (
        62,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3528 => SpriteInfo {
    pos: (
        496,
        596,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2693 => SpriteInfo {
    pos: (
        3085,
        1062,
    ),
    size: (
        108,
        28,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4380 => SpriteInfo {
    pos: (
        1153,
        0,
    ),
    size: (
        52,
        12,
    ),
    rotated: false,
    offset: (
        -2.0,
        -22.0,
    ),
},4227 => SpriteInfo {
    pos: (
        3253,
        887,
    ),
    size: (
        4,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2465 => SpriteInfo {
    pos: (
        2073,
        0,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},202 => SpriteInfo {
    pos: (
        687,
        1054,
    ),
    size: (
        202,
        226,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2598 => SpriteInfo {
    pos: (
        2433,
        764,
    ),
    size: (
        56,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},2482 => SpriteInfo {
    pos: (
        2226,
        1717,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3057 => SpriteInfo {
    pos: (
        3212,
        2205,
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
},469 => SpriteInfo {
    pos: (
        957,
        1444,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2501 => SpriteInfo {
    pos: (
        1859,
        472,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3998 => SpriteInfo {
    pos: (
        3865,
        962,
    ),
    size: (
        64,
        54,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3550 => SpriteInfo {
    pos: (
        3274,
        448,
    ),
    size: (
        44,
        60,
    ),
    rotated: false,
    offset: (
        2.0,
        -10.0,
    ),
},3814 => SpriteInfo {
    pos: (
        2695,
        409,
    ),
    size: (
        56,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},908 => SpriteInfo {
    pos: (
        694,
        1912,
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
},2663 => SpriteInfo {
    pos: (
        2433,
        44,
    ),
    size: (
        8,
        4,
    ),
    rotated: false,
    offset: (
        -26.0,
        24.0,
    ),
},3862 => SpriteInfo {
    pos: (
        285,
        1390,
    ),
    size: (
        60,
        50,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},234 => SpriteInfo {
    pos: (
        462,
        2702,
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
},2704 => SpriteInfo {
    pos: (
        1611,
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
},4233 => SpriteInfo {
    pos: (
        3706,
        1405,
    ),
    size: (
        64,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        -4.0,
    ),
},3425 => SpriteInfo {
    pos: (
        3545,
        1324,
    ),
    size: (
        40,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},218 => SpriteInfo {
    pos: (
        1985,
        2323,
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
},1604 => SpriteInfo {
    pos: (
        1698,
        128,
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
},939 => SpriteInfo {
    pos: (
        3633,
        2360,
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
},2649 => SpriteInfo {
    pos: (
        1017,
        1099,
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
},891 => SpriteInfo {
    pos: (
        3718,
        600,
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
},2580 => SpriteInfo {
    pos: (
        1887,
        752,
    ),
    size: (
        56,
        56,
    ),
    rotated: false,
    offset: (
        4.0,
        4.0,
    ),
},468 => SpriteInfo {
    pos: (
        248,
        438,
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
},3817 => SpriteInfo {
    pos: (
        1570,
        1393,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2632 => SpriteInfo {
    pos: (
        2101,
        2191,
    ),
    size: (
        52,
        36,
    ),
    rotated: false,
    offset: (
        6.0,
        -14.0,
    ),
},1757 => SpriteInfo {
    pos: (
        2600,
        1880,
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
},3324 => SpriteInfo {
    pos: (
        3630,
        512,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3530 => SpriteInfo {
    pos: (
        1737,
        1913,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2889 => SpriteInfo {
    pos: (
        3233,
        2349,
    ),
    size: (
        238,
        57,
    ),
    rotated: false,
    offset: (
        1.0,
        15.5,
    ),
},1861 => SpriteInfo {
    pos: (
        841,
        670,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},151 => SpriteInfo {
    pos: (
        3539,
        1392,
    ),
    size: (
        48,
        159,
    ),
    rotated: false,
    offset: (
        0.0,
        0.5,
    ),
},934 => SpriteInfo {
    pos: (
        3965,
        1902,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3122 => SpriteInfo {
    pos: (
        2474,
        2176,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1849 => SpriteInfo {
    pos: (
        2068,
        1807,
    ),
    size: (
        143,
        64,
    ),
    rotated: false,
    offset: (
        -18.5,
        -8.0,
    ),
},2481 => SpriteInfo {
    pos: (
        0,
        731,
    ),
    size: (
        60,
        44,
    ),
    rotated: false,
    offset: (
        -2.0,
        -6.0,
    ),
},3549 => SpriteInfo {
    pos: (
        1521,
        1801,
    ),
    size: (
        44,
        80,
    ),
    rotated: false,
    offset: (
        2.0,
        0.0,
    ),
},2567 => SpriteInfo {
    pos: (
        3842,
        644,
    ),
    size: (
        108,
        56,
    ),
    rotated: false,
    offset: (
        10.0,
        0.0,
    ),
},3652 => SpriteInfo {
    pos: (
        3633,
        2399,
    ),
    size: (
        64,
        50,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3625 => SpriteInfo {
    pos: (
        517,
        124,
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
},4005 => SpriteInfo {
    pos: (
        3863,
        304,
    ),
    size: (
        64,
        20,
    ),
    rotated: false,
    offset: (
        0.0,
        -18.0,
    ),
},3226 => SpriteInfo {
    pos: (
        2675,
        1045,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},273 => SpriteInfo {
    pos: (
        3761,
        1849,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1711 => SpriteInfo {
    pos: (
        3905,
        1700,
    ),
    size: (
        185,
        198,
    ),
    rotated: false,
    offset: (
        -6.5,
        -47.0,
    ),
},2553 => SpriteInfo {
    pos: (
        1903,
        2354,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2063 => SpriteInfo {
    pos: (
        3089,
        1460,
    ),
    size: (
        78,
        126,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3130 => SpriteInfo {
    pos: (
        1679,
        363,
    ),
    size: (
        60,
        64,
    ),
    rotated: false,
    offset: (
        -2.0,
        0.0,
    ),
},2690 => SpriteInfo {
    pos: (
        4071,
        0,
    ),
    size: (
        4,
        4,
    ),
    rotated: false,
    offset: (
        -58.0,
        0.0,
    ),
},2684 => SpriteInfo {
    pos: (
        1735,
        823,
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
},2686 => SpriteInfo {
    pos: (
        4063,
        0,
    ),
    size: (
        4,
        44,
    ),
    rotated: false,
    offset: (
        -24.0,
        0.0,
    ),
},2516 => SpriteInfo {
    pos: (
        3993,
        282,
    ),
    size: (
        96,
        44,
    ),
    rotated: false,
    offset: (
        0.0,
        -26.0,
    ),
},2573 => SpriteInfo {
    pos: (
        3003,
        693,
    ),
    size: (
        64,
        8,
    ),
    rotated: false,
    offset: (
        0.0,
        -24.0,
    ),
},1584 => SpriteInfo {
    pos: (
        2515,
        619,
    ),
    size: (
        202,
        212,
    ),
    rotated: false,
    offset: (
        17.0,
        -36.0,
    ),
},2943 => SpriteInfo {
    pos: (
        387,
        2441,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2520 => SpriteInfo {
    pos: (
        2869,
        2292,
    ),
    size: (
        52,
        60,
    ),
    rotated: false,
    offset: (
        22.0,
        18.0,
    ),
},1292 => SpriteInfo {
    pos: (
        3621,
        0,
    ),
    size: (
        60,
        39,
    ),
    rotated: false,
    offset: (
        0.0,
        0.5,
    ),
},2581 => SpriteInfo {
    pos: (
        3481,
        1256,
    ),
    size: (
        64,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        6.0,
    ),
},2542 => SpriteInfo {
    pos: (
        2930,
        956,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2700 => SpriteInfo {
    pos: (
        1735,
        1027,
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
},3931 => SpriteInfo {
    pos: (
        1519,
        2435,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4134 => SpriteInfo {
    pos: (
        2183,
        276,
    ),
    size: (
        40,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2867 => SpriteInfo {
    pos: (
        1321,
        1583,
    ),
    size: (
        146,
        95,
    ),
    rotated: false,
    offset: (
        7.0,
        15.5,
    ),
},2549 => SpriteInfo {
    pos: (
        1939,
        812,
    ),
    size: (
        4,
        4,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3318 => SpriteInfo {
    pos: (
        3742,
        324,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3313 => SpriteInfo {
    pos: (
        2193,
        0,
    ),
    size: (
        64,
        40,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2279 => SpriteInfo {
    pos: (
        1986,
        2612,
    ),
    size: (
        36,
        116,
    ),
    rotated: false,
    offset: (
        -18.0,
        6.0,
    ),
},85 => SpriteInfo {
    pos: (
        1987,
        396,
    ),
    size: (
        280,
        280,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2417 => SpriteInfo {
    pos: (
        1141,
        16,
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
},696 => SpriteInfo {
    pos: (
        648,
        2432,
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
},2551 => SpriteInfo {
    pos: (
        1427,
        591,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2687 => SpriteInfo {
    pos: (
        783,
        66,
    ),
    size: (
        128,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},1606 => SpriteInfo {
    pos: (
        3461,
        1756,
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
},1582 => SpriteInfo {
    pos: (
        3601,
        1337,
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
},2643 => SpriteInfo {
    pos: (
        4085,
        2110,
    ),
    size: (
        4,
        60,
    ),
    rotated: false,
    offset: (
        -24.0,
        0.0,
    ),
},2644 => SpriteInfo {
    pos: (
        2215,
        1865,
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
},927 => SpriteInfo {
    pos: (
        1996,
        1391,
    ),
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
        1605,
        1765,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2500 => SpriteInfo {
    pos: (
        389,
        1510,
    ),
    size: (
        56,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2857 => SpriteInfo {
    pos: (
        3830,
        1399,
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
},2360 => SpriteInfo {
    pos: (
        1083,
        1961,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},940 => SpriteInfo {
    pos: (
        2661,
        64,
    ),
    size: (
        98,
        121,
    ),
    rotated: false,
    offset: (
        0.0,
        1.5,
    ),
},3519 => SpriteInfo {
    pos: (
        449,
        64,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3545 => SpriteInfo {
    pos: (
        1902,
        588,
    ),
    size: (
        60,
        56,
    ),
    rotated: false,
    offset: (
        10.0,
        12.0,
    ),
},4187 => SpriteInfo {
    pos: (
        2294,
        1717,
    ),
    size: (
        40,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2946 => SpriteInfo {
    pos: (
        0,
        1442,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2990 => SpriteInfo {
    pos: (
        139,
        2260,
    ),
    size: (
        120,
        118,
    ),
    rotated: false,
    offset: (
        0.0,
        -1.0,
    ),
},3001 => SpriteInfo {
    pos: (
        3517,
        1706,
    ),
    size: (
        120,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        -4.0,
    ),
},2694 => SpriteInfo {
    pos: (
        68,
        1094,
    ),
    size: (
        132,
        116,
    ),
    rotated: false,
    offset: (
        0.0,
        8.0,
    ),
},1767 => SpriteInfo {
    pos: (
        387,
        2362,
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
},470 => SpriteInfo {
    pos: (
        2766,
        1115,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3150 => SpriteInfo {
    pos: (
        2210,
        2477,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1598 => SpriteInfo {
    pos: (
        3742,
        488,
    ),
    size: (
        104,
        108,
    ),
    rotated: false,
    offset: (
        0.0,
        6.0,
    ),
},2964 => SpriteInfo {
    pos: (
        3289,
        689,
    ),
    size: (
        116,
        107,
    ),
    rotated: false,
    offset: (
        0.0,
        -6.5,
    ),
},47 => SpriteInfo {
    pos: (
        2664,
        1880,
    ),
    size: (
        132,
        339,
    ),
    rotated: false,
    offset: (
        23.0,
        1.5,
    ),
},3228 => SpriteInfo {
    pos: (
        953,
        474,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2413 => SpriteInfo {
    pos: (
        0,
        1566,
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
},3478 => SpriteInfo {
    pos: (
        3150,
        1995,
    ),
    size: (
        60,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3863 => SpriteInfo {
    pos: (
        3533,
        853,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1726 => SpriteInfo {
    pos: (
        2146,
        1023,
    ),
    size: (
        121,
        40,
    ),
    rotated: false,
    offset: (
        -0.5,
        6.0,
    ),
},2557 => SpriteInfo {
    pos: (
        2068,
        1614,
    ),
    size: (
        28,
        8,
    ),
    rotated: false,
    offset: (
        18.0,
        -24.0,
    ),
},1864 => SpriteInfo {
    pos: (
        3195,
        2682,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3502 => SpriteInfo {
    pos: (
        2350,
        1706,
    ),
    size: (
        168,
        124,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2653 => SpriteInfo {
    pos: (
        1847,
        236,
    ),
    size: (
        4,
        52,
    ),
    rotated: false,
    offset: (
        -24.0,
        4.0,
    ),
},2570 => SpriteInfo {
    pos: (
        3342,
        272,
    ),
    size: (
        64,
        40,
    ),
    rotated: false,
    offset: (
        0.0,
        -12.0,
    ),
},3042 => SpriteInfo {
    pos: (
        1327,
        492,
    ),
    size: (
        96,
        96,
    ),
    rotated: false,
    offset: (
        0.0,
        1.0,
    ),
},1865 => SpriteInfo {
    pos: (
        3850,
        520,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2651 => SpriteInfo {
    pos: (
        3187,
        0,
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
},2565 => SpriteInfo {
    pos: (
        664,
        1143,
    ),
    size: (
        12,
        8,
    ),
    rotated: false,
    offset: (
        -26.0,
        -28.0,
    ),
},3309 => SpriteInfo {
    pos: (
        379,
        1856,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3993 => SpriteInfo {
    pos: (
        1411,
        659,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3371 => SpriteInfo {
    pos: (
        2869,
        44,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1698 => SpriteInfo {
    pos: (
        3406,
        2120,
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
},2637 => SpriteInfo {
    pos: (
        1411,
        839,
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
},3 => SpriteInfo {
    pos: (
        1786,
        2676,
    ),
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
        1717,
        1981,
    ),
    size: (
        132,
        339,
    ),
    rotated: false,
    offset: (
        23.0,
        1.5,
    ),
},2641 => SpriteInfo {
    pos: (
        2347,
        1902,
    ),
    size: (
        60,
        20,
    ),
    rotated: false,
    offset: (
        0.0,
        -16.0,
    ),
},3803 => SpriteInfo {
    pos: (
        3973,
        2781,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3804 => SpriteInfo {
    pos: (
        2158,
        2609,
    ),
    size: (
        64,
        61,
    ),
    rotated: false,
    offset: (
        0.0,
        0.5,
    ),
},4149 => SpriteInfo {
    pos: (
        1876,
        1525,
    ),
    size: (
        76,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4002 => SpriteInfo {
    pos: (
        1519,
        2515,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3943 => SpriteInfo {
    pos: (
        124,
        1553,
    ),
    size: (
        56,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3332 => SpriteInfo {
    pos: (
        2424,
        2722,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3852 => SpriteInfo {
    pos: (
        893,
        1178,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3331 => SpriteInfo {
    pos: (
        2245,
        84,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3367 => SpriteInfo {
    pos: (
        3994,
        1399,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1734 => SpriteInfo {
    pos: (
        1744,
        1589,
    ),
    size: (
        320,
        320,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3902 => SpriteInfo {
    pos: (
        1893,
        2032,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3855 => SpriteInfo {
    pos: (
        3092,
        2251,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1706 => SpriteInfo {
    pos: (
        661,
        2188,
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
},413 => SpriteInfo {
    pos: (
        627,
        1006,
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
},3152 => SpriteInfo {
    pos: (
        2059,
        1095,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4085 => SpriteInfo {
    pos: (
        3894,
        898,
    ),
    size: (
        64,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3320 => SpriteInfo {
    pos: (
        3088,
        1590,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2658 => SpriteInfo {
    pos: (
        1395,
        2352,
    ),
    size: (
        52,
        4,
    ),
    rotated: false,
    offset: (
        0.0,
        24.0,
    ),
},2947 => SpriteInfo {
    pos: (
        3749,
        132,
    ),
    size: (
        116,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},266 => SpriteInfo {
    pos: (
        3962,
        830,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2543 => SpriteInfo {
    pos: (
        3815,
        1529,
    ),
    size: (
        60,
        12,
    ),
    rotated: false,
    offset: (
        2.0,
        -22.0,
    ),
},1340 => SpriteInfo {
    pos: (
        659,
        336,
    ),
    size: (
        106,
        6,
    ),
    rotated: false,
    offset: (
        -8.0,
        0.0,
    ),
},4368 => SpriteInfo {
    pos: (
        2781,
        1015,
    ),
    size: (
        60,
        8,
    ),
    rotated: false,
    offset: (
        2.0,
        -24.0,
    ),
},663 => SpriteInfo {
    pos: (
        1711,
        2324,
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
},3427 => SpriteInfo {
    pos: (
        191,
        2446,
    ),
    size: (
        44,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3795 => SpriteInfo {
    pos: (
        1735,
        779,
    ),
    size: (
        64,
        40,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1605 => SpriteInfo {
    pos: (
        3327,
        970,
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
},1846 => SpriteInfo {
    pos: (
        1255,
        1682,
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
},4208 => SpriteInfo {
    pos: (
        308,
        370,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1332 => SpriteInfo {
    pos: (
        1853,
        2002,
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
},505 => SpriteInfo {
    pos: (
        2068,
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
},3979 => SpriteInfo {
    pos: (
        737,
        126,
    ),
    size: (
        121,
        121,
    ),
    rotated: false,
    offset: (
        -1.5,
        -2.5,
    ),
},12 => SpriteInfo {
    pos: (
        688,
        702,
    ),
    size: (
        132,
        339,
    ),
    rotated: false,
    offset: (
        23.0,
        1.5,
    ),
},3965 => SpriteInfo {
    pos: (
        2890,
        1172,
    ),
    size: (
        128,
        128,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1725 => SpriteInfo {
    pos: (
        3319,
        2784,
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
},2574 => SpriteInfo {
    pos: (
        2408,
        2448,
    ),
    size: (
        64,
        40,
    ),
    rotated: false,
    offset: (
        0.0,
        -12.0,
    ),
},137 => SpriteInfo {
    pos: (
        303,
        864,
    ),
    size: (
        320,
        318,
    ),
    rotated: false,
    offset: (
        0.0,
        1.0,
    ),
},2517 => SpriteInfo {
    pos: (
        3726,
        2319,
    ),
    size: (
        96,
        96,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3879 => SpriteInfo {
    pos: (
        1495,
        2583,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3123 => SpriteInfo {
    pos: (
        292,
        2274,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2550 => SpriteInfo {
    pos: (
        1147,
        1893,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2249 => SpriteInfo {
    pos: (
        2278,
        1140,
    ),
    size: (
        60,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1602 => SpriteInfo {
    pos: (
        851,
        0,
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
},3631 => SpriteInfo {
    pos: (
        3877,
        0,
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
},2228 => SpriteInfo {
    pos: (
        1871,
        812,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1758 => SpriteInfo {
    pos: (
        0,
        2539,
    ),
    size: (
        175,
        174,
    ),
    rotated: false,
    offset: (
        0.5,
        1.0,
    ),
},2546 => SpriteInfo {
    pos: (
        3185,
        899,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2945 => SpriteInfo {
    pos: (
        3337,
        2620,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2672 => SpriteInfo {
    pos: (
        1689,
        0,
    ),
    size: (
        12,
        20,
    ),
    rotated: false,
    offset: (
        24.0,
        -16.0,
    ),
},2662 => SpriteInfo {
    pos: (
        379,
        1924,
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
},3129 => SpriteInfo {
    pos: (
        2000,
        126,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        -2.0,
        2.0,
    ),
},4191 => SpriteInfo {
    pos: (
        564,
        2289,
    ),
    size: (
        48,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2504 => SpriteInfo {
    pos: (
        3004,
        468,
    ),
    size: (
        64,
        44,
    ),
    rotated: false,
    offset: (
        0.0,
        10.0,
    ),
},3321 => SpriteInfo {
    pos: (
        2340,
        2402,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3576 => SpriteInfo {
    pos: (
        889,
        374,
    ),
    size: (
        128,
        96,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},141 => SpriteInfo {
    pos: (
        3656,
        1591,
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
},2026 => SpriteInfo {
    pos: (
        0,
        2399,
    ),
    size: (
        97,
        136,
    ),
    rotated: false,
    offset: (
        6.5,
        -15.0,
    ),
},83 => SpriteInfo {
    pos: (
        1234,
        1111,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},13 => SpriteInfo {
    pos: (
        2142,
        1067,
    ),
    size: (
        132,
        339,
    ),
    rotated: false,
    offset: (
        23.0,
        1.5,
    ),
},3402 => SpriteInfo {
    pos: (
        2492,
        2706,
    ),
    size: (
        20,
        24,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2680 => SpriteInfo {
    pos: (
        3322,
        448,
    ),
    size: (
        56,
        20,
    ),
    rotated: false,
    offset: (
        2.0,
        -16.0,
    ),
},3646 => SpriteInfo {
    pos: (
        3681,
        68,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2988 => SpriteInfo {
    pos: (
        172,
        2084,
    ),
    size: (
        120,
        108,
    ),
    rotated: false,
    offset: (
        0.0,
        -6.0,
    ),
},2468 => SpriteInfo {
    pos: (
        3003,
        625,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3941 => SpriteInfo {
    pos: (
        2342,
        1208,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1885 => SpriteInfo {
    pos: (
        2587,
        493,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2313 => SpriteInfo {
    pos: (
        943,
        1306,
    ),
    size: (
        8,
        16,
    ),
    rotated: false,
    offset: (
        -2.0,
        -2.0,
    ),
},474 => SpriteInfo {
    pos: (
        1513,
        2711,
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
},2368 => SpriteInfo {
    pos: (
        179,
        2551,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4076 => SpriteInfo {
    pos: (
        555,
        2595,
    ),
    size: (
        76,
        92,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4 => SpriteInfo {
    pos: (
        2596,
        2597,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2544 => SpriteInfo {
    pos: (
        3685,
        0,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3501 => SpriteInfo {
    pos: (
        3274,
        324,
    ),
    size: (
        156,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4382 => SpriteInfo {
    pos: (
        3459,
        2104,
    ),
    size: (
        60,
        12,
    ),
    rotated: false,
    offset: (
        2.0,
        -22.0,
    ),
},2605 => SpriteInfo {
    pos: (
        880,
        1831,
    ),
    size: (
        56,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2926 => SpriteInfo {
    pos: (
        3249,
        1084,
    ),
    size: (
        94,
        301,
    ),
    rotated: false,
    offset: (
        21.0,
        4.5,
    ),
},131 => SpriteInfo {
    pos: (
        1467,
        2047,
    ),
    size: (
        152,
        72,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2652 => SpriteInfo {
    pos: (
        1109,
        2549,
    ),
    size: (
        52,
        4,
    ),
    rotated: false,
    offset: (
        0.0,
        -24.0,
    ),
},216 => SpriteInfo {
    pos: (
        3071,
        2580,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4356 => SpriteInfo {
    pos: (
        2895,
        799,
    ),
    size: (
        20,
        44,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2512 => SpriteInfo {
    pos: (
        2611,
        409,
    ),
    size: (
        80,
        80,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4004 => SpriteInfo {
    pos: (
        1041,
        2461,
    ),
    size: (
        60,
        20,
    ),
    rotated: false,
    offset: (
        2.0,
        -18.0,
    ),
},2300 => SpriteInfo {
    pos: (
        2604,
        1514,
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
},1866 => SpriteInfo {
    pos: (
        2028,
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
},3851 => SpriteInfo {
    pos: (
        3815,
        1467,
    ),
    size: (
        64,
        58,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3361 => SpriteInfo {
    pos: (
        152,
        1881,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4064 => SpriteInfo {
    pos: (
        3894,
        762,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2627 => SpriteInfo {
    pos: (
        3974,
        634,
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
},3975 => SpriteInfo {
    pos: (
        630,
        2781,
    ),
    size: (
        128,
        115,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.5,
    ),
},2515 => SpriteInfo {
    pos: (
        1083,
        2029,
    ),
    size: (
        96,
        96,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4190 => SpriteInfo {
    pos: (
        4071,
        8,
    ),
    size: (
        16,
        40,
    ),
    rotated: false,
    offset: (
        0.0,
        12.0,
    ),
},2689 => SpriteInfo {
    pos: (
        3749,
        124,
    ),
    size: (
        112,
        4,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2654 => SpriteInfo {
    pos: (
        2082,
        963,
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
},2638 => SpriteInfo {
    pos: (
        3185,
        875,
    ),
    size: (
        56,
        20,
    ),
    rotated: false,
    offset: (
        2.0,
        -16.0,
    ),
},3308 => SpriteInfo {
    pos: (
        1125,
        80,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2674 => SpriteInfo {
    pos: (
        3758,
        2017,
    ),
    size: (
        60,
        4,
    ),
    rotated: false,
    offset: (
        0.0,
        24.0,
    ),
},3622 => SpriteInfo {
    pos: (
        68,
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
},2566 => SpriteInfo {
    pos: (
        100,
        994,
    ),
    size: (
        128,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1884 => SpriteInfo {
    pos: (
        3251,
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
},3848 => SpriteInfo {
    pos: (
        987,
        238,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2894 => SpriteInfo {
    pos: (
        1606,
        675,
    ),
    size: (
        243,
        42,
    ),
    rotated: false,
    offset: (
        0.5,
        3.0,
    ),
},2858 => SpriteInfo {
    pos: (
        1073,
        1283,
    ),
    size: (
        60,
        8,
    ),
    rotated: false,
    offset: (
        0.0,
        -26.0,
    ),
},3521 => SpriteInfo {
    pos: (
        841,
        542,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3634 => SpriteInfo {
    pos: (
        701,
        510,
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
},1204 => SpriteInfo {
    pos: (
        299,
        178,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1722 => SpriteInfo {
    pos: (
        862,
        225,
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
},4142 => SpriteInfo {
    pos: (
        449,
        36,
    ),
    size: (
        84,
        24,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1709 => SpriteInfo {
    pos: (
        3171,
        1579,
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
},4226 => SpriteInfo {
    pos: (
        188,
        1225,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2603 => SpriteInfo {
    pos: (
        3946,
        1407,
    ),
    size: (
        44,
        28,
    ),
    rotated: false,
    offset: (
        -10.0,
        18.0,
    ),
},2583 => SpriteInfo {
    pos: (
        2082,
        1027,
    ),
    size: (
        56,
        64,
    ),
    rotated: false,
    offset: (
        4.0,
        0.0,
    ),
},2312 => SpriteInfo {
    pos: (
        3978,
        368,
    ),
    size: (
        8,
        56,
    ),
    rotated: false,
    offset: (
        -2.0,
        -2.0,
    ),
},3058 => SpriteInfo {
    pos: (
        349,
        1390,
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
},3480 => SpriteInfo {
    pos: (
        3461,
        2742,
    ),
    size: (
        60,
        44,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2012 => SpriteInfo {
    pos: (
        304,
        37,
    ),
    size: (
        141,
        137,
    ),
    rotated: false,
    offset: (
        0.5,
        -3.5,
    ),
},1338 => SpriteInfo {
    pos: (
        2029,
        2609,
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
},259 => SpriteInfo {
    pos: (
        1263,
        684,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4150 => SpriteInfo {
    pos: (
        1089,
        0,
    ),
    size: (
        48,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2554 => SpriteInfo {
    pos: (
        452,
        796,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1852 => SpriteInfo {
    pos: (
        199,
        104,
    ),
    size: (
        80,
        100,
    ),
    rotated: false,
    offset: (
        3.0,
        -30.0,
    ),
},2590 => SpriteInfo {
    pos: (
        3591,
        1502,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1207 => SpriteInfo {
    pos: (
        1769,
        0,
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
},1838 => SpriteInfo {
    pos: (
        383,
        2565,
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
},1834 => SpriteInfo {
    pos: (
        2522,
        1796,
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
},143 => SpriteInfo {
    pos: (
        893,
        1054,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3232 => SpriteInfo {
    pos: (
        0,
        1203,
    ),
    size: (
        56,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},67 => SpriteInfo {
    pos: (
        1257,
        1519,
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
},2227 => SpriteInfo {
    pos: (
        2279,
        1865,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2558 => SpriteInfo {
    pos: (
        2843,
        847,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},107 => SpriteInfo {
    pos: (
        340,
        698,
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
},2555 => SpriteInfo {
    pos: (
        4063,
        52,
    ),
    size: (
        28,
        8,
    ),
    rotated: false,
    offset: (
        18.0,
        -24.0,
    ),
},3231 => SpriteInfo {
    pos: (
        2025,
        2191,
    ),
    size: (
        72,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3046 => SpriteInfo {
    pos: (
        296,
        2158,
    ),
    size: (
        112,
        112,
    ),
    rotated: false,
    offset: (
        0.0,
        -4.0,
    ),
},203 => SpriteInfo {
    pos: (
        3630,
        711,
    ),
    size: (
        260,
        226,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1712 => SpriteInfo {
    pos: (
        2668,
        1560,
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
},3311 => SpriteInfo {
    pos: (
        237,
        1444,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3145 => SpriteInfo {
    pos: (
        3581,
        2625,
    ),
    size: (
        64,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},1845 => SpriteInfo {
    pos: (
        635,
        2663,
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
},2027 => SpriteInfo {
    pos: (
        2543,
        2048,
    ),
    size: (
        86,
        220,
    ),
    rotated: false,
    offset: (
        -1.0,
        -8.0,
    ),
},3578 => SpriteInfo {
    pos: (
        1431,
        136,
    ),
    size: (
        16,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1764 => SpriteInfo {
    pos: (
        3711,
        2104,
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
},2226 => SpriteInfo {
    pos: (
        3469,
        1630,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2419 => SpriteInfo {
    pos: (
        1480,
        1519,
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
},2514 => SpriteInfo {
    pos: (
        0,
        839,
    ),
    size: (
        56,
        96,
    ),
    rotated: false,
    offset: (
        -20.0,
        0.0,
    ),
},3651 => SpriteInfo {
    pos: (
        3381,
        2410,
    ),
    size: (
        64,
        62,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2464 => SpriteInfo {
    pos: (
        0,
        939,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1054 => SpriteInfo {
    pos: (
        44,
        2193,
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
},3907 => SpriteInfo {
    pos: (
        3214,
        2143,
    ),
    size: (
        64,
        58,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1736 => SpriteInfo {
    pos: (
        3263,
        803,
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
},1009 => SpriteInfo {
    pos: (
        1487,
        456,
    ),
    size: (
        39,
        39,
    ),
    rotated: false,
    offset: (
        0.5,
        0.5,
    ),
},2578 => SpriteInfo {
    pos: (
        1623,
        2109,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2518 => SpriteInfo {
    pos: (
        3621,
        43,
    ),
    size: (
        56,
        96,
    ),
    rotated: false,
    offset: (
        20.0,
        0.0,
    ),
},458 => SpriteInfo {
    pos: (
        2429,
        2308,
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
},3630 => SpriteInfo {
    pos: (
        1806,
        1319,
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
},3983 => SpriteInfo {
    pos: (
        3825,
        2488,
    ),
    size: (
        116,
        118,
    ),
    rotated: false,
    offset: (
        -1.0,
        1.0,
    ),
},1720 => SpriteInfo {
    pos: (
        3071,
        2507,
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
},1735 => SpriteInfo {
    pos: (
        2169,
        2021,
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
},2575 => SpriteInfo {
    pos: (
        192,
        28,
    ),
    size: (
        52,
        8,
    ),
    rotated: false,
    offset: (
        -6.0,
        -24.0,
    ),
},1343 => SpriteInfo {
    pos: (
        2649,
        2223,
    ),
    size: (
        98,
        12,
    ),
    rotated: false,
    offset: (
        -12.0,
        0.0,
    ),
},2993 => SpriteInfo {
    pos: (
        68,
        991,
    ),
    size: (
        28,
        92,
    ),
    rotated: false,
    offset: (
        2.0,
        -7.0,
    ),
},3854 => SpriteInfo {
    pos: (
        529,
        1563,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3859 => SpriteInfo {
    pos: (
        3338,
        2069,
    ),
    size: (
        64,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2991 => SpriteInfo {
    pos: (
        680,
        0,
    ),
    size: (
        99,
        120,
    ),
    rotated: false,
    offset: (
        -1.5,
        8.0,
    ),
},2862 => SpriteInfo {
    pos: (
        1847,
        172,
    ),
    size: (
        8,
        8,
    ),
    rotated: false,
    offset: (
        26.0,
        -26.0,
    ),
},3536 => SpriteInfo {
    pos: (
        40,
        1646,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},138 => SpriteInfo {
    pos: (
        1169,
        2470,
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
},87 => SpriteInfo {
    pos: (
        2763,
        112,
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
},409 => SpriteInfo {
    pos: (
        1972,
        2009,
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
},2675 => SpriteInfo {
    pos: (
        1729,
        1657,
    ),
    size: (
        4,
        12,
    ),
    rotated: false,
    offset: (
        -24.0,
        -24.0,
    ),
},3522 => SpriteInfo {
    pos: (
        3877,
        2296,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2463 => SpriteInfo {
    pos: (
        3837,
        2740,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4143 => SpriteInfo {
    pos: (
        1711,
        2388,
    ),
    size: (
        72,
        24,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2477 => SpriteInfo {
    pos: (
        1394,
        963,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2568 => SpriteInfo {
    pos: (
        1633,
        24,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3861 => SpriteInfo {
    pos: (
        2427,
        1000,
    ),
    size: (
        60,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        -1.0,
    ),
},3477 => SpriteInfo {
    pos: (
        2068,
        1570,
    ),
    size: (
        28,
        40,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2665 => SpriteInfo {
    pos: (
        3481,
        1352,
    ),
    size: (
        60,
        20,
    ),
    rotated: false,
    offset: (
        0.0,
        -16.0,
    ),
},3802 => SpriteInfo {
    pos: (
        2096,
        328,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4377 => SpriteInfo {
    pos: (
        1068,
        826,
    ),
    size: (
        8,
        8,
    ),
    rotated: false,
    offset: (
        -28.0,
        -28.0,
    ),
},2552 => SpriteInfo {
    pos: (
        965,
        670,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2522 => SpriteInfo {
    pos: (
        1041,
        594,
    ),
    size: (
        40,
        84,
    ),
    rotated: false,
    offset: (
        24.0,
        6.0,
    ),
},1707 => SpriteInfo {
    pos: (
        2755,
        468,
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
},4065 => SpriteInfo {
    pos: (
        1975,
        1913,
    ),
    size: (
        64,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        4.0,
    ),
},3373 => SpriteInfo {
    pos: (
        2101,
        1999,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4378 => SpriteInfo {
    pos: (
        440,
        2257,
    ),
    size: (
        120,
        60,
    ),
    rotated: false,
    offset: (
        4.0,
        2.0,
    ),
},2639 => SpriteInfo {
    pos: (
        4080,
        2628,
    ),
    size: (
        4,
        52,
    ),
    rotated: false,
    offset: (
        -24.0,
        4.0,
    ),
},2250 => SpriteInfo {
    pos: (
        3473,
        2554,
    ),
    size: (
        112,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4170 => SpriteInfo {
    pos: (
        789,
        438,
    ),
    size: (
        92,
        68,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3910 => SpriteInfo {
    pos: (
        3706,
        1337,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3403 => SpriteInfo {
    pos: (
        3447,
        484,
    ),
    size: (
        28,
        20,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2640 => SpriteInfo {
    pos: (
        3088,
        1746,
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
},1206 => SpriteInfo {
    pos: (
        283,
        60,
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
},3329 => SpriteInfo {
    pos: (
        1406,
        68,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2685 => SpriteInfo {
    pos: (
        1927,
        512,
    ),
    size: (
        52,
        4,
    ),
    rotated: false,
    offset: (
        0.0,
        -24.0,
    ),
},1759 => SpriteInfo {
    pos: (
        2120,
        1420,
    ),
    size: (
        275,
        189,
    ),
    rotated: false,
    offset: (
        0.5,
        0.5,
    ),
},4228 => SpriteInfo {
    pos: (
        3161,
        596,
    ),
    size: (
        56,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2647 => SpriteInfo {
    pos: (
        552,
        0,
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
},3809 => SpriteInfo {
    pos: (
        232,
        1012,
    ),
    size: (
        64,
        61,
    ),
    rotated: false,
    offset: (
        0.0,
        0.5,
    ),
},3089 => SpriteInfo {
    pos: (
        1491,
        1953,
    ),
    size: (
        128,
        90,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},187 => SpriteInfo {
    pos: (
        3495,
        266,
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
},3996 => SpriteInfo {
    pos: (
        1835,
        2350,
    ),
    size: (
        64,
        58,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3962 => SpriteInfo {
    pos: (
        1144,
        943,
    ),
    size: (
        92,
        128,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1597 => SpriteInfo {
    pos: (
        3393,
        1630,
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
},2508 => SpriteInfo {
    pos: (
        2227,
        276,
    ),
    size: (
        64,
        36,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},3220 => SpriteInfo {
    pos: (
        3274,
        256,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1222 => SpriteInfo {
    pos: (
        2476,
        1860,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},662 => SpriteInfo {
    pos: (
        1475,
        872,
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
},2866 => SpriteInfo {
    pos: (
        524,
        2357,
    ),
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
        423,
        242,
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
},2301 => SpriteInfo {
    pos: (
        4008,
        2228,
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
},3806 => SpriteInfo {
    pos: (
        3274,
        1835,
    ),
    size: (
        64,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},695 => SpriteInfo {
    pos: (
        3538,
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
},3450 => SpriteInfo {
    pos: (
        3282,
        2143,
    ),
    size: (
        32,
        36,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},3997 => SpriteInfo {
    pos: (
        1133,
        1463,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1704 => SpriteInfo {
    pos: (
        60,
        124,
    ),
    size: (
        135,
        138,
    ),
    rotated: false,
    offset: (
        9.5,
        0.0,
    ),
},4376 => SpriteInfo {
    pos: (
        3150,
        2055,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        2.0,
        2.0,
    ),
},3000 => SpriteInfo {
    pos: (
        3150,
        1835,
    ),
    size: (
        120,
        28,
    ),
    rotated: false,
    offset: (
        0.0,
        -5.0,
    ),
},201 => SpriteInfo {
    pos: (
        1865,
        118,
    ),
    size: (
        131,
        226,
    ),
    rotated: false,
    offset: (
        -0.5,
        0.0,
    ),
},3632 => SpriteInfo {
    pos: (
        248,
        572,
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
},3801 => SpriteInfo {
    pos: (
        217,
        1321,
    ),
    size: (
        64,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2657 => SpriteInfo {
    pos: (
        3028,
        2202,
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
},942 => SpriteInfo {
    pos: (
        1462,
        0,
    ),
    size: (
        31,
        42,
    ),
    rotated: false,
    offset: (
        -0.5,
        1.0,
    ),
},2683 => SpriteInfo {
    pos: (
        1855,
        68,
    ),
    size: (
        4,
        44,
    ),
    rotated: false,
    offset: (
        -24.0,
        0.0,
    ),
},3138 => SpriteInfo {
    pos: (
        68,
        703,
    ),
    size: (
        44,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3857 => SpriteInfo {
    pos: (
        1903,
        1457,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1754 => SpriteInfo {
    pos: (
        3289,
        681,
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
},2648 => SpriteInfo {
    pos: (
        2871,
        759,
    ),
    size: (
        60,
        4,
    ),
    rotated: false,
    offset: (
        0.0,
        24.0,
    ),
},2668 => SpriteInfo {
    pos: (
        1823,
        723,
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
},36 => SpriteInfo {
    pos: (
        3701,
        2419,
    ),
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
        915,
        150,
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
},2563 => SpriteInfo {
    pos: (
        3161,
        468,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        2.0,
        2.0,
    ),
},2028 => SpriteInfo {
    pos: (
        3072,
        468,
    ),
    size: (
        85,
        189,
    ),
    rotated: false,
    offset: (
        4.5,
        -1.5,
    ),
},3369 => SpriteInfo {
    pos: (
        2356,
        2666,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1600 => SpriteInfo {
    pos: (
        3954,
        1467,
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
},3518 => SpriteInfo {
    pos: (
        3465,
        2606,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2587 => SpriteInfo {
    pos: (
        584,
        758,
    ),
    size: (
        64,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        -4.0,
    ),
},2650 => SpriteInfo {
    pos: (
        3253,
        823,
    ),
    size: (
        4,
        60,
    ),
    rotated: false,
    offset: (
        -24.0,
        0.0,
    ),
},4357 => SpriteInfo {
    pos: (
        2769,
        0,
    ),
    size: (
        28,
        40,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3087 => SpriteInfo {
    pos: (
        2937,
        88,
    ),
    size: (
        123,
        107,
    ),
    rotated: false,
    offset: (
        0.5,
        2.5,
    ),
},2490 => SpriteInfo {
    pos: (
        1507,
        828,
    ),
    size: (
        64,
        40,
    ),
    rotated: false,
    offset: (
        0.0,
        -12.0,
    ),
},4069 => SpriteInfo {
    pos: (
        1323,
        1858,
    ),
    size: (
        64,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        4.0,
    ),
},1210 => SpriteInfo {
    pos: (
        1863,
        348,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3443 => SpriteInfo {
    pos: (
        1977,
        2236,
    ),
    size: (
        24,
        28,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},407 => SpriteInfo {
    pos: (
        204,
        1141,
    ),
    size: (
        44,
        34,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2697 => SpriteInfo {
    pos: (
        1853,
        2286,
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
},4096 => SpriteInfo {
    pos: (
        953,
        306,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3554 => SpriteInfo {
    pos: (
        288,
        1936,
    ),
    size: (
        80,
        8,
    ),
    rotated: false,
    offset: (
        0.0,
        8.0,
    ),
},3531 => SpriteInfo {
    pos: (
        1623,
        1957,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1012 => SpriteInfo {
    pos: (
        3499,
        0,
    ),
    size: (
        118,
        118,
    ),
    rotated: false,
    offset: (
        1.0,
        1.0,
    ),
},2548 => SpriteInfo {
    pos: (
        2617,
        835,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2579 => SpriteInfo {
    pos: (
        64,
        543,
    ),
    size: (
        48,
        12,
    ),
    rotated: false,
    offset: (
        -4.0,
        -22.0,
    ),
},1863 => SpriteInfo {
    pos: (
        3950,
        2432,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4000 => SpriteInfo {
    pos: (
        1917,
        2286,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4098 => SpriteInfo {
    pos: (
        3537,
        1638,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3055 => SpriteInfo {
    pos: (
        1041,
        2397,
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
},3648 => SpriteInfo {
    pos: (
        1902,
        648,
    ),
    size: (
        60,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2 => SpriteInfo {
    pos: (
        3218,
        1895,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4300 => SpriteInfo {
    pos: (
        2212,
        1933,
    ),
    size: (
        96,
        20,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2559 => SpriteInfo {
    pos: (
        2115,
        192,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3054 => SpriteInfo {
    pos: (
        2017,
        2259,
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
},35 => SpriteInfo {
    pos: (
        3728,
        2299,
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
},661 => SpriteInfo {
    pos: (
        3662,
        2296,
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
},2373 => SpriteInfo {
    pos: (
        3591,
        1438,
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
},1225 => SpriteInfo {
    pos: (
        60,
        843,
    ),
    size: (
        48,
        24,
    ),
    rotated: false,
    offset: (
        -36.0,
        -48.0,
    ),
},3533 => SpriteInfo {
    pos: (
        2271,
        364,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1464 => SpriteInfo {
    pos: (
        2184,
        152,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3534 => SpriteInfo {
    pos: (
        1173,
        875,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3005 => SpriteInfo {
    pos: (
        3690,
        664,
    ),
    size: (
        114,
        43,
    ),
    rotated: false,
    offset: (
        0.0,
        -8.5,
    ),
},2411 => SpriteInfo {
    pos: (
        2437,
        2244,
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
},200 => SpriteInfo {
    pos: (
        940,
        1893,
    ),
    size: (
        139,
        174,
    ),
    rotated: false,
    offset: (
        -0.5,
        0.0,
    ),
},3626 => SpriteInfo {
    pos: (
        580,
        1912,
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
},1730 => SpriteInfo {
    pos: (
        1027,
        2757,
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
},1836 => SpriteInfo {
    pos: (
        2278,
        1292,
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
},3368 => SpriteInfo {
    pos: (
        3422,
        803,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3537 => SpriteInfo {
    pos: (
        3163,
        1194,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2571 => SpriteInfo {
    pos: (
        3630,
        648,
    ),
    size: (
        56,
        8,
    ),
    rotated: false,
    offset: (
        4.0,
        -24.0,
    ),
},4178 => SpriteInfo {
    pos: (
        3196,
        2497,
    ),
    size: (
        8,
        12,
    ),
    rotated: false,
    offset: (
        -6.0,
        0.0,
    ),
},3229 => SpriteInfo {
    pos: (
        3409,
        1001,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2412 => SpriteInfo {
    pos: (
        2404,
        1966,
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
},2469 => SpriteInfo {
    pos: (
        0,
        611,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1328 => SpriteInfo {
    pos: (
        2913,
        1024,
    ),
    size: (
        122,
        144,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1209 => SpriteInfo {
    pos: (
        3587,
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
},3853 => SpriteInfo {
    pos: (
        311,
        1868,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3527 => SpriteInfo {
    pos: (
        504,
        1375,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2891 => SpriteInfo {
    pos: (
        1786,
        2546,
    ),
    size: (
        239,
        62,
    ),
    rotated: false,
    offset: (
        -1.5,
        0.0,
    ),
},2474 => SpriteInfo {
    pos: (
        2721,
        973,
    ),
    size: (
        40,
        20,
    ),
    rotated: false,
    offset: (
        -12.0,
        -22.0,
    ),
},3553 => SpriteInfo {
    pos: (
        208,
        2072,
    ),
    size: (
        72,
        8,
    ),
    rotated: false,
    offset: (
        0.0,
        8.0,
    ),
},1333 => SpriteInfo {
    pos: (
        1215,
        1926,
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
},1728 => SpriteInfo {
    pos: (
        3322,
        484,
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
},2499 => SpriteInfo {
    pos: (
        2862,
        1304,
    ),
    size: (
        56,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3847 => SpriteInfo {
    pos: (
        1257,
        1547,
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
},1752 => SpriteInfo {
    pos: (
        917,
        0,
    ),
    size: (
        168,
        146,
    ),
    rotated: false,
    offset: (
        19.0,
        16.0,
    ),
},4169 => SpriteInfo {
    pos: (
        2157,
        2235,
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
},699 => SpriteInfo {
    pos: (
        2633,
        2239,
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
},4188 => SpriteInfo {
    pos: (
        2350,
        1613,
    ),
    size: (
        40,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},84 => SpriteInfo {
    pos: (
        903,
        2666,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3375 => SpriteInfo {
    pos: (
        3758,
        2025,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2224 => SpriteInfo {
    pos: (
        3653,
        2740,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3447 => SpriteInfo {
    pos: (
        3698,
        512,
    ),
    size: (
        28,
        12,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2961 => SpriteInfo {
    pos: (
        1587,
        2301,
    ),
    size: (
        120,
        107,
    ),
    rotated: false,
    offset: (
        0.0,
        -6.5,
    ),
},4165 => SpriteInfo {
    pos: (
        1735,
        895,
    ),
    size: (
        128,
        60,
    ),
    rotated: false,
    offset: (
        20.0,
        0.0,
    ),
},3366 => SpriteInfo {
    pos: (
        2271,
        432,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2887 => SpriteInfo {
    pos: (
        2030,
        2545,
    ),
    size: (
        230,
        60,
    ),
    rotated: false,
    offset: (
        -5.0,
        3.0,
    ),
},4375 => SpriteInfo {
    pos: (
        3381,
        2476,
    ),
    size: (
        56,
        56,
    ),
    rotated: false,
    offset: (
        -4.0,
        0.0,
    ),
},211 => SpriteInfo {
    pos: (
        411,
        336,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1011 => SpriteInfo {
    pos: (
        3022,
        1282,
    ),
    size: (
        120,
        118,
    ),
    rotated: false,
    offset: (
        0.0,
        1.0,
    ),
},3312 => SpriteInfo {
    pos: (
        1675,
        295,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1763 => SpriteInfo {
    pos: (
        3461,
        1875,
    ),
    size: (
        293,
        225,
    ),
    rotated: false,
    offset: (
        0.5,
        0.5,
    ),
},2962 => SpriteInfo {
    pos: (
        3921,
        1022,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2367 => SpriteInfo {
    pos: (
        1530,
        1016,
    ),
    size: (
        64,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2588 => SpriteInfo {
    pos: (
        1338,
        0,
    ),
    size: (
        56,
        52,
    ),
    rotated: false,
    offset: (
        -4.0,
        -6.0,
    ),
},1835 => SpriteInfo {
    pos: (
        1563,
        2692,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2476 => SpriteInfo {
    pos: (
        3234,
        1389,
    ),
    size: (
        64,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},19 => SpriteInfo {
    pos: (
        460,
        1763,
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
},877 => SpriteInfo {
    pos: (
        200,
        1646,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3811 => SpriteInfo {
    pos: (
        3160,
        2251,
    ),
    size: (
        41,
        64,
    ),
    rotated: false,
    offset: (
        -0.5,
        0.0,
    ),
},1868 => SpriteInfo {
    pos: (
        220,
        1948,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2472 => SpriteInfo {
    pos: (
        344,
        1948,
    ),
    size: (
        24,
        24,
    ),
    rotated: false,
    offset: (
        -20.0,
        -20.0,
    ),
},2473 => SpriteInfo {
    pos: (
        2369,
        696,
    ),
    size: (
        128,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3945 => SpriteInfo {
    pos: (
        669,
        124,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3637 => SpriteInfo {
    pos: (
        300,
        1186,
    ),
    size: (
        200,
        200,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},20 => SpriteInfo {
    pos: (
        1606,
        555,
    ),
    size: (
        292,
        116,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3969 => SpriteInfo {
    pos: (
        3239,
        124,
    ),
    size: (
        128,
        128,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3221 => SpriteInfo {
    pos: (
        621,
        526,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3935 => SpriteInfo {
    pos: (
        563,
        2481,
    ),
    size: (
        64,
        54,
    ),
    rotated: false,
    offset: (
        0.0,
        -5.0,
    ),
},2421 => SpriteInfo {
    pos: (
        1413,
        1742,
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
},3629 => SpriteInfo {
    pos: (
        3213,
        2574,
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
},3086 => SpriteInfo {
    pos: (
        2000,
        192,
    ),
    size: (
        111,
        95,
    ),
    rotated: false,
    offset: (
        0.5,
        2.5,
    ),
},3363 => SpriteInfo {
    pos: (
        1041,
        2329,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},188 => SpriteInfo {
    pos: (
        0,
        266,
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
},745 => SpriteInfo {
    pos: (
        905,
        2323,
    ),
    size: (
        132,
        339,
    ),
    rotated: false,
    offset: (
        23.0,
        1.5,
    ),
},3849 => SpriteInfo {
    pos: (
        584,
        690,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2531 => SpriteInfo {
    pos: (
        3327,
        962,
    ),
    size: (
        80,
        4,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2671 => SpriteInfo {
    pos: (
        895,
        2432,
    ),
    size: (
        4,
        60,
    ),
    rotated: false,
    offset: (
        -24.0,
        0.0,
    ),
},3805 => SpriteInfo {
    pos: (
        2312,
        1950,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2703 => SpriteInfo {
    pos: (
        2299,
        1016,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2673 => SpriteInfo {
    pos: (
        3150,
        2119,
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
},4171 => SpriteInfo {
    pos: (
        2346,
        2478,
    ),
    size: (
        56,
        72,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1053 => SpriteInfo {
    pos: (
        3055,
        950,
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
},2521 => SpriteInfo {
    pos: (
        3894,
        1399,
    ),
    size: (
        96,
        4,
    ),
    rotated: false,
    offset: (
        0.0,
        -10.0,
    ),
},2502 => SpriteInfo {
    pos: (
        3781,
        1613,
    ),
    size: (
        64,
        24,
    ),
    rotated: false,
    offset: (
        0.0,
        -20.0,
    ),
},2660 => SpriteInfo {
    pos: (
        0,
        543,
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
},3362 => SpriteInfo {
    pos: (
        1735,
        959,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3330 => SpriteInfo {
    pos: (
        1537,
        1733,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3633 => SpriteInfo {
    pos: (
        469,
        1563,
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
},3547 => SpriteInfo {
    pos: (
        324,
        1748,
    ),
    size: (
        80,
        80,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1589 => SpriteInfo {
    pos: (
        1136,
        1075,
    ),
    size: (
        94,
        92,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},3810 => SpriteInfo {
    pos: (
        1531,
        2711,
    ),
    size: (
        26,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2661 => SpriteInfo {
    pos: (
        4087,
        1018,
    ),
    size: (
        4,
        8,
    ),
    rotated: false,
    offset: (
        -24.0,
        -26.0,
    ),
},2478 => SpriteInfo {
    pos: (
        2475,
        2052,
    ),
    size: (
        64,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},3516 => SpriteInfo {
    pos: (
        1530,
        456,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4151 => SpriteInfo {
    pos: (
        1085,
        875,
    ),
    size: (
        84,
        44,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3807 => SpriteInfo {
    pos: (
        0,
        0,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2666 => SpriteInfo {
    pos: (
        1803,
        959,
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
},2645 => SpriteInfo {
    pos: (
        4087,
        1030,
    ),
    size: (
        4,
        56,
    ),
    rotated: false,
    offset: (
        -24.0,
        -2.0,
    ),
},3503 => SpriteInfo {
    pos: (
        1447,
        787,
    ),
    size: (
        56,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4097 => SpriteInfo {
    pos: (
        3163,
        1262,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1614 => SpriteInfo {
    pos: (
        533,
        508,
    ),
    size: (
        84,
        84,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},697 => SpriteInfo {
    pos: (
        2313,
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
},2545 => SpriteInfo {
    pos: (
        1338,
        68,
    ),
    size: (
        64,
        8,
    ),
    rotated: false,
    offset: (
        0.0,
        -24.0,
    ),
},4070 => SpriteInfo {
    pos: (
        2192,
        1957,
    ),
    size: (
        64,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},1330 => SpriteInfo {
    pos: (
        3347,
        1221,
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
},2577 => SpriteInfo {
    pos: (
        2030,
        2477,
    ),
    size: (
        44,
        8,
    ),
    rotated: false,
    offset: (
        -2.0,
        -24.0,
    ),
},3995 => SpriteInfo {
    pos: (
        3863,
        328,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2488 => SpriteInfo {
    pos: (
        2817,
        0,
    ),
    size: (
        64,
        40,
    ),
    rotated: false,
    offset: (
        0.0,
        -12.0,
    ),
},3144 => SpriteInfo {
    pos: (
        793,
        2731,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2523 => SpriteInfo {
    pos: (
        2649,
        0,
    ),
    size: (
        8,
        92,
    ),
    rotated: false,
    offset: (
        40.0,
        2.0,
    ),
},1327 => SpriteInfo {
    pos: (
        0,
        438,
    ),
    size: (
        120,
        101,
    ),
    rotated: false,
    offset: (
        0.0,
        0.5,
    ),
},932 => SpriteInfo {
    pos: (
        3758,
        2093,
    ),
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
        311,
        2659,
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
},928 => SpriteInfo {
    pos: (
        2026,
        2738,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},217 => SpriteInfo {
    pos: (
        3371,
        214,
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
},414 => SpriteInfo {
    pos: (
        3950,
        2228,
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
},3135 => SpriteInfo {
    pos: (
        64,
        559,
    ),
    size: (
        48,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2355 => SpriteInfo {
    pos: (
        3441,
        627,
    ),
    size: (
        48,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1058 => SpriteInfo {
    pos: (
        1240,
        963,
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
},1701 => SpriteInfo {
    pos: (
        2797,
        1444,
    ),
    size: (
        156,
        112,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2562 => SpriteInfo {
    pos: (
        2078,
        2477,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1519 => SpriteInfo {
    pos: (
        4045,
        1058,
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
},3988 => SpriteInfo {
    pos: (
        2100,
        1613,
    ),
    size: (
        122,
        122,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2584 => SpriteInfo {
    pos: (
        3205,
        2269,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2586 => SpriteInfo {
    pos: (
        1451,
        727,
    ),
    size: (
        52,
        56,
    ),
    rotated: false,
    offset: (
        6.0,
        -4.0,
    ),
},411 => SpriteInfo {
    pos: (
        1394,
        1036,
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
},1699 => SpriteInfo {
    pos: (
        1530,
        1160,
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
},3850 => SpriteInfo {
    pos: (
        3221,
        628,
    ),
    size: (
        64,
        63,
    ),
    rotated: false,
    offset: (
        0.0,
        0.5,
    ),
},7 => SpriteInfo {
    pos: (
        3781,
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
},3423 => SpriteInfo {
    pos: (
        3434,
        428,
    ),
    size: (
        36,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4066 => SpriteInfo {
    pos: (
        3273,
        2281,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3222 => SpriteInfo {
    pos: (
        3995,
        0,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2505 => SpriteInfo {
    pos: (
        1089,
        52,
    ),
    size: (
        32,
        48,
    ),
    rotated: false,
    offset: (
        16.0,
        8.0,
    ),
},2963 => SpriteInfo {
    pos: (
        504,
        1186,
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
},3551 => SpriteInfo {
    pos: (
        252,
        1141,
    ),
    size: (
        36,
        80,
    ),
    rotated: false,
    offset: (
        -2.0,
        0.0,
    ),
},3426 => SpriteInfo {
    pos: (
        3549,
        1256,
    ),
    size: (
        48,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1833 => SpriteInfo {
    pos: (
        1935,
        1049,
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
},4003 => SpriteInfo {
    pos: (
        3588,
        1570,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2467 => SpriteInfo {
    pos: (
        705,
        251,
    ),
    size: (
        60,
        64,
    ),
    rotated: false,
    offset: (
        -2.0,
        0.0,
    ),
},4323 => SpriteInfo {
    pos: (
        1395,
        2104,
    ),
    size: (
        64,
        44,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},39 => SpriteInfo {
    pos: (
        3393,
        1698,
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
},3323 => SpriteInfo {
    pos: (
        3894,
        830,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4371 => SpriteInfo {
    pos: (
        2974,
        1340,
    ),
    size: (
        24,
        60,
    ),
    rotated: false,
    offset: (
        -16.0,
        -2.0,
    ),
},3439 => SpriteInfo {
    pos: (
        2801,
        2260,
    ),
    size: (
        64,
        44,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3535 => SpriteInfo {
    pos: (
        2408,
        1834,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},6 => SpriteInfo {
    pos: (
        3214,
        2019,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4173 => SpriteInfo {
    pos: (
        3197,
        1062,
    ),
    size: (
        48,
        80,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1714 => SpriteInfo {
    pos: (
        3603,
        1173,
    ),
    size: (
        148,
        160,
    ),
    rotated: false,
    offset: (
        -6.0,
        -28.0,
    ),
},2366 => SpriteInfo {
    pos: (
        2617,
        903,
    ),
    size: (
        56,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},103 => SpriteInfo {
    pos: (
        2220,
        1785,
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
},394 => SpriteInfo {
    pos: (
        1205,
        1235,
    ),
    size: (
        321,
        280,
    ),
    rotated: false,
    offset: (
        -0.5,
        0.0,
    ),
},1341 => SpriteInfo {
    pos: (
        3342,
        1952,
    ),
    size: (
        113,
        113,
    ),
    rotated: false,
    offset: (
        1.5,
        1.5,
    ),
},3647 => SpriteInfo {
    pos: (
        1068,
        987,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1344 => SpriteInfo {
    pos: (
        1377,
        2559,
    ),
    size: (
        114,
        114,
    ),
    rotated: false,
    offset: (
        -3.0,
        11.0,
    ),
},1059 => SpriteInfo {
    pos: (
        2615,
        2411,
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
},1697 => SpriteInfo {
    pos: (
        1971,
        1515,
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
},2483 => SpriteInfo {
    pos: (
        3239,
        1513,
    ),
    size: (
        64,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},2422 => SpriteInfo {
    pos: (
        1041,
        554,
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
},3858 => SpriteInfo {
    pos: (
        2035,
        1999,
    ),
    size: (
        62,
        57,
    ),
    rotated: false,
    offset: (
        0.0,
        0.5,
    ),
},1830 => SpriteInfo {
    pos: (
        3261,
        962,
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
},471 => SpriteInfo {
    pos: (
        3071,
        2704,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3233 => SpriteInfo {
    pos: (
        0,
        1135,
    ),
    size: (
        56,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4072 => SpriteInfo {
    pos: (
        3799,
        1545,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4174 => SpriteInfo {
    pos: (
        3797,
        941,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3444 => SpriteInfo {
    pos: (
        3382,
        448,
    ),
    size: (
        24,
        28,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1342 => SpriteInfo {
    pos: (
        1573,
        1270,
    ),
    size: (
        229,
        119,
    ),
    rotated: false,
    offset: (
        2.5,
        2.5,
    ),
},186 => SpriteInfo {
    pos: (
        2731,
        2447,
    ),
    size: (
        336,
        338,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},106 => SpriteInfo {
    pos: (
        961,
        1178,
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
},3364 => SpriteInfo {
    pos: (
        3494,
        941,
    ),
    size: (
        4,
        4,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4135 => SpriteInfo {
    pos: (
        2484,
        2666,
    ),
    size: (
        40,
        36,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},472 => SpriteInfo {
    pos: (
        1097,
        2621,
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
},2572 => SpriteInfo {
    pos: (
        1017,
        1055,
    ),
    size: (
        64,
        40,
    ),
    rotated: false,
    offset: (
        0.0,
        -12.0,
    ),
},3404 => SpriteInfo {
    pos: (
        68,
        611,
    ),
    size: (
        36,
        24,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3424 => SpriteInfo {
    pos: (
        3885,
        1857,
    ),
    size: (
        12,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2055 => SpriteInfo {
    pos: (
        1939,
        931,
    ),
    size: (
        139,
        114,
    ),
    rotated: false,
    offset: (
        -1.5,
        -8.0,
    ),
},190 => SpriteInfo {
    pos: (
        1109,
        2557,
    ),
    size: (
        56,
        196,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3137 => SpriteInfo {
    pos: (
        68,
        639,
    ),
    size: (
        36,
        28,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2541 => SpriteInfo {
    pos: (
        1358,
        1126,
    ),
    size: (
        64,
        8,
    ),
    rotated: false,
    offset: (
        0.0,
        -24.0,
    ),
},3446 => SpriteInfo {
    pos: (
        3755,
        1173,
    ),
    size: (
        24,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3636 => SpriteInfo {
    pos: (
        2445,
        0,
    ),
    size: (
        200,
        200,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2556 => SpriteInfo {
    pos: (
        2107,
        771,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3621 => SpriteInfo {
    pos: (
        1463,
        2247,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3955 => SpriteInfo {
    pos: (
        821,
        266,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2354 => SpriteInfo {
    pos: (
        2128,
        128,
    ),
    size: (
        52,
        56,
    ),
    rotated: false,
    offset: (
        -4.0,
        -2.0,
    ),
},3933 => SpriteInfo {
    pos: (
        3829,
        1781,
    ),
    size: (
        60,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1732 => SpriteInfo {
    pos: (
        1638,
        1393,
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
},3440 => SpriteInfo {
    pos: (
        2308,
        248,
    ),
    size: (
        64,
        44,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1339 => SpriteInfo {
    pos: (
        2751,
        629,
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
},10 => SpriteInfo {
    pos: (
        1507,
        524,
    ),
    size: (
        95,
        300,
    ),
    rotated: false,
    offset: (
        21.5,
        4.0,
    ),
},3524 => SpriteInfo {
    pos: (
        1453,
        1767,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},906 => SpriteInfo {
    pos: (
        3977,
        2696,
    ),
    size: (
        99,
        81,
    ),
    rotated: false,
    offset: (
        -0.5,
        0.5,
    ),
},4381 => SpriteInfo {
    pos: (
        303,
        2667,
    ),
    size: (
        56,
        12,
    ),
    rotated: false,
    offset: (
        -4.0,
        -22.0,
    ),
},2692 => SpriteInfo {
    pos: (
        1763,
        1457,
    ),
    size: (
        136,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2503 => SpriteInfo {
    pos: (
        2711,
        477,
    ),
    size: (
        28,
        44,
    ),
    rotated: false,
    offset: (
        18.0,
        -10.0,
    ),
},3481 => SpriteInfo {
    pos: (
        240,
        370,
    ),
    size: (
        64,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2699 => SpriteInfo {
    pos: (
        3195,
        695,
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
},3627 => SpriteInfo {
    pos: (
        2255,
        696,
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
},395 => SpriteInfo {
    pos: (
        2561,
        237,
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
},2513 => SpriteInfo {
    pos: (
        3318,
        2197,
    ),
    size: (
        80,
        80,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},664 => SpriteInfo {
    pos: (
        3691,
        1467,
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
},2691 => SpriteInfo {
    pos: (
        2929,
        2266,
    ),
    size: (
        128,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},918 => SpriteInfo {
    pos: (
        3797,
        1146,
    ),
    size: (
        286,
        249,
    ),
    rotated: false,
    offset: (
        1.0,
        0.5,
    ),
},2480 => SpriteInfo {
    pos: (
        315,
        2595,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        2.0,
        2.0,
    ),
},3376 => SpriteInfo {
    pos: (
        3461,
        2674,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},18 => SpriteInfo {
    pos: (
        2759,
        298,
    ),
    size: (
        511,
        166,
    ),
    rotated: false,
    offset: (
        -0.5,
        0.0,
    ),
},3624 => SpriteInfo {
    pos: (
        3826,
        2364,
    ),
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
        3950,
        2292,
    ),
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
        2271,
        572,
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
},4087 => SpriteInfo {
    pos: (
        247,
        908,
    ),
    size: (
        36,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3944 => SpriteInfo {
    pos: (
        2342,
        1140,
    ),
    size: (
        52,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},878 => SpriteInfo {
    pos: (
        824,
        806,
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
},2585 => SpriteInfo {
    pos: (
        3721,
        2740,
    ),
    size: (
        56,
        64,
    ),
    rotated: false,
    offset: (
        -4.0,
        0.0,
    ),
},2560 => SpriteInfo {
    pos: (
        3088,
        1658,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1461 => SpriteInfo {
    pos: (
        3869,
        180,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2540 => SpriteInfo {
    pos: (
        3382,
        1872,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2682 => SpriteInfo {
    pos: (
        1633,
        0,
    ),
    size: (
        52,
        20,
    ),
    rotated: false,
    offset: (
        0.0,
        -16.0,
    ),
},2311 => SpriteInfo {
    pos: (
        2068,
        1626,
    ),
    size: (
        12,
        40,
    ),
    rotated: false,
    offset: (
        -4.0,
        -2.0,
    ),
},929 => SpriteInfo {
    pos: (
        2303,
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
},3860 => SpriteInfo {
    pos: (
        1867,
        948,
    ),
    size: (
        64,
        59,
    ),
    rotated: false,
    offset: (
        0.0,
        -1.5,
    ),
},134 => SpriteInfo {
    pos: (
        3003,
        705,
    ),
    size: (
        56,
        123,
    ),
    rotated: false,
    offset: (
        0.0,
        0.5,
    ),
},2642 => SpriteInfo {
    pos: (
        1395,
        2288,
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
},3901 => SpriteInfo {
    pos: (
        2627,
        2343,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2987 => SpriteInfo {
    pos: (
        3753,
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
},3520 => SpriteInfo {
    pos: (
        3630,
        580,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},5 => SpriteInfo {
    pos: (
        2175,
        899,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4324 => SpriteInfo {
    pos: (
        3393,
        1824,
    ),
    size: (
        64,
        44,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2678 => SpriteInfo {
    pos: (
        3885,
        1849,
    ),
    size: (
        12,
        4,
    ),
    rotated: false,
    offset: (
        24.0,
        24.0,
    ),
},2353 => SpriteInfo {
    pos: (
        1687,
        2692,
    ),
    size: (
        60,
        44,
    ),
    rotated: false,
    offset: (
        0.0,
        -8.0,
    ),
},2944 => SpriteInfo {
    pos: (
        2226,
        2666,
    ),
    size: (
        61,
        120,
    ),
    rotated: false,
    offset: (
        -29.5,
        0.0,
    ),
},3325 => SpriteInfo {
    pos: (
        1803,
        787,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2547 => SpriteInfo {
    pos: (
        2347,
        1926,
    ),
    size: (
        52,
        8,
    ),
    rotated: false,
    offset: (
        -6.0,
        -24.0,
    ),
},4172 => SpriteInfo {
    pos: (
        199,
        208,
    ),
    size: (
        92,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3994 => SpriteInfo {
    pos: (
        3530,
        1555,
    ),
    size: (
        54,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4230 => SpriteInfo {
    pos: (
        2615,
        1023,
    ),
    size: (
        56,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},910 => SpriteInfo {
    pos: (
        520,
        796,
    ),
    size: (
        58,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3582 => SpriteInfo {
    pos: (
        164,
        1770,
    ),
    size: (
        156,
        40,
    ),
    rotated: false,
    offset: (
        0.0,
        -24.0,
    ),
},3812 => SpriteInfo {
    pos: (
        3413,
        681,
    ),
    size: (
        64,
        54,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3294 => SpriteInfo {
    pos: (
        3481,
        1312,
    ),
    size: (
        60,
        36,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2506 => SpriteInfo {
    pos: (
        1779,
        168,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4372 => SpriteInfo {
    pos: (
        905,
        2015,
    ),
    size: (
        16,
        60,
    ),
    rotated: false,
    offset: (
        16.0,
        -2.0,
    ),
},4175 => SpriteInfo {
    pos: (
        1033,
        682,
    ),
    size: (
        40,
        112,
    ),
    rotated: false,
    offset: (
        0.0,
        -36.0,
    ),
},2479 => SpriteInfo {
    pos: (
        821,
        334,
    ),
    size: (
        64,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},2696 => SpriteInfo {
    pos: (
        1360,
        1519,
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
},3479 => SpriteInfo {
    pos: (
        2425,
        2360,
    ),
    size: (
        52,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2576 => SpriteInfo {
    pos: (
        3758,
        1973,
    ),
    size: (
        64,
        40,
    ),
    rotated: false,
    offset: (
        0.0,
        -12.0,
    ),
},4006 => SpriteInfo {
    pos: (
        462,
        2766,
    ),
    size: (
        64,
        20,
    ),
    rotated: false,
    offset: (
        0.0,
        -18.0,
    ),
},101 => SpriteInfo {
    pos: (
        1606,
        721,
    ),
    size: (
        125,
        356,
    ),
    rotated: false,
    offset: (
        24.5,
        1.0,
    ),
},1888 => SpriteInfo {
    pos: (
        2848,
        1560,
    ),
    size: (
        236,
        236,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1345 => SpriteInfo {
    pos: (
        2925,
        2326,
    ),
    size: (
        220,
        117,
    ),
    rotated: false,
    offset: (
        -8.0,
        10.5,
    ),
},3310 => SpriteInfo {
    pos: (
        3962,
        954,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3942 => SpriteInfo {
    pos: (
        1867,
        1011,
    ),
    size: (
        64,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1813 => SpriteInfo {
    pos: (
        3347,
        1355,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2646 => SpriteInfo {
    pos: (
        2389,
        0,
    ),
    size: (
        52,
        4,
    ),
    rotated: false,
    offset: (
        4.0,
        24.0,
    ),
},3654 => SpriteInfo {
    pos: (
        1021,
        370,
    ),
    size: (
        57,
        60,
    ),
    rotated: false,
    offset: (
        0.5,
        -1.0,
    ),
},4374 => SpriteInfo {
    pos: (
        1611,
        363,
    ),
    size: (
        64,
        8,
    ),
    rotated: false,
    offset: (
        0.0,
        24.0,
    ),
},4168 => SpriteInfo {
    pos: (
        1449,
        2751,
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
},3796 => SpriteInfo {
    pos: (
        1109,
        2329,
    ),
    size: (
        28,
        28,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1334 => SpriteInfo {
    pos: (
        1169,
        2677,
    ),
    size: (
        276,
        226,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2679 => SpriteInfo {
    pos: (
        2361,
        2338,
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
},2489 => SpriteInfo {
    pos: (
        2035,
        1199,
    ),
    size: (
        64,
        40,
    ),
    rotated: false,
    offset: (
        0.0,
        -12.0,
    ),
},1203 => SpriteInfo {
    pos: (
        456,
        1912,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1221 => SpriteInfo {
    pos: (
        3071,
        664,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},392 => SpriteInfo {
    pos: (
        3434,
        376,
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
},2670 => SpriteInfo {
    pos: (
        1463,
        2371,
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
},3635 => SpriteInfo {
    pos: (
        1431,
        276,
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
},1223 => SpriteInfo {
    pos: (
        901,
        1963,
    ),
    size: (
        24,
        24,
    ),
    rotated: false,
    offset: (
        -48.0,
        -48.0,
    ),
},2475 => SpriteInfo {
    pos: (
        889,
        266,
    ),
    size: (
        60,
        32,
    ),
    rotated: false,
    offset: (
        -2.0,
        16.0,
    ),
},3455 => SpriteInfo {
    pos: (
        3774,
        1399,
    ),
    size: (
        52,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3476 => SpriteInfo {
    pos: (
        661,
        2036,
    ),
    size: (
        28,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1583 => SpriteInfo {
    pos: (
        2476,
        2464,
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
},3448 => SpriteInfo {
    pos: (
        124,
        454,
    ),
    size: (
        28,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3523 => SpriteInfo {
    pos: (
        132,
        1632,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1607 => SpriteInfo {
    pos: (
        3039,
        1054,
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
},1837 => SpriteInfo {
    pos: (
        2493,
        835,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},152 => SpriteInfo {
    pos: (
        2528,
        2684,
    ),
    size: (
        62,
        119,
    ),
    rotated: false,
    offset: (
        0.0,
        0.5,
    ),
},3372 => SpriteInfo {
    pos: (
        3393,
        1756,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1716 => SpriteInfo {
    pos: (
        408,
        460,
    ),
    size: (
        121,
        58,
    ),
    rotated: false,
    offset: (
        -0.5,
        -24.0,
    ),
},3124 => SpriteInfo {
    pos: (
        1041,
        2485,
    ),
    size: (
        60,
        64,
    ),
    rotated: false,
    offset: (
        -2.0,
        0.0,
    ),
},412 => SpriteInfo {
    pos: (
        1053,
        2129,
    ),
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
        101,
        2446,
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
},2667 => SpriteInfo {
    pos: (
        1495,
        591,
    ),
    size: (
        4,
        60,
    ),
    rotated: false,
    offset: (
        -24.0,
        0.0,
    ),
},4231 => SpriteInfo {
    pos: (
        3225,
        536,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1705 => SpriteInfo {
    pos: (
        2812,
        1810,
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
},2363 => SpriteInfo {
    pos: (
        3085,
        1094,
    ),
    size: (
        48,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},3552 => SpriteInfo {
    pos: (
        3342,
        1835,
    ),
    size: (
        36,
        60,
    ),
    rotated: false,
    offset: (
        -2.0,
        -10.0,
    ),
},3946 => SpriteInfo {
    pos: (
        1006,
        2253,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},99 => SpriteInfo {
    pos: (
        116,
        634,
    ),
    size: (
        127,
        356,
    ),
    rotated: false,
    offset: (
        24.5,
        1.0,
    ),
},2698 => SpriteInfo {
    pos: (
        889,
        302,
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
},3146 => SpriteInfo {
    pos: (
        3150,
        1927,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},129 => SpriteInfo {
    pos: (
        3657,
        2610,
    ),
    size: (
        316,
        126,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4177 => SpriteInfo {
    pos: (
        3123,
        910,
    ),
    size: (
        44,
        84,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2582 => SpriteInfo {
    pos: (
        2543,
        1984,
    ),
    size: (
        52,
        48,
    ),
    rotated: false,
    offset: (
        -6.0,
        8.0,
    ),
},3360 => SpriteInfo {
    pos: (
        2068,
        1739,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2948 => SpriteInfo {
    pos: (
        2721,
        787,
    ),
    size: (
        118,
        118,
    ),
    rotated: false,
    offset: (
        -1.0,
        -1.0,
    ),
},4185 => SpriteInfo {
    pos: (
        3346,
        1479,
    ),
    size: (
        84,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2994 => SpriteInfo {
    pos: (
        2491,
        959,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2393 => SpriteInfo {
    pos: (
        1074,
        2253,
    ),
    size: (
        64,
        72,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4369 => SpriteInfo {
    pos: (
        1519,
        2503,
    ),
    size: (
        60,
        8,
    ),
    rotated: false,
    offset: (
        2.0,
        -24.0,
    ),
},1829 => SpriteInfo {
    pos: (
        2673,
        1429,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2509 => SpriteInfo {
    pos: (
        3319,
        2744,
    ),
    size: (
        64,
        36,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},11 => SpriteInfo {
    pos: (
        3991,
        330,
    ),
    size: (
        95,
        300,
    ),
    rotated: false,
    offset: (
        21.5,
        4.0,
    ),
},2385 => SpriteInfo {
    pos: (
        3731,
        1531,
    ),
    size: (
        64,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        -4.0,
    ),
},3526 => SpriteInfo {
    pos: (
        2845,
        915,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1227 => SpriteInfo {
    pos: (
        1463,
        2123,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3319 => SpriteInfo {
    pos: (
        2475,
        1984,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2462 => SpriteInfo {
    pos: (
        4022,
        762,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},130 => SpriteInfo {
    pos: (
        0,
        2751,
    ),
    size: (
        458,
        176,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2589 => SpriteInfo {
    pos: (
        3589,
        2557,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2949 => SpriteInfo {
    pos: (
        3317,
        559,
    ),
    size: (
        120,
        118,
    ),
    rotated: false,
    offset: (
        0.0,
        1.0,
    ),
},2278 => SpriteInfo {
    pos: (
        1926,
        2612,
    ),
    size: (
        56,
        128,
    ),
    rotated: false,
    offset: (
        -28.0,
        0.0,
    ),
},2677 => SpriteInfo {
    pos: (
        4085,
        2026,
    ),
    size: (
        4,
        60,
    ),
    rotated: false,
    offset: (
        -24.0,
        0.0,
    ),
},701 => SpriteInfo {
    pos: (
        263,
        2342,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3525 => SpriteInfo {
    pos: (
        3338,
        2129,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4001 => SpriteInfo {
    pos: (
        1505,
        1885,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},110 => SpriteInfo {
    pos: (
        925,
        2183,
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
},1591 => SpriteInfo {
    pos: (
        3950,
        2416,
    ),
    size: (
        120,
        12,
    ),
    rotated: false,
    offset: (
        0.0,
        1.0,
    ),
},3004 => SpriteInfo {
    pos: (
        1947,
        771,
    ),
    size: (
        156,
        156,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1847 => SpriteInfo {
    pos: (
        3073,
        65,
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
},3295 => SpriteInfo {
    pos: (
        627,
        962,
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
},2470 => SpriteInfo {
    pos: (
        965,
        738,
    ),
    size: (
        60,
        64,
    ),
    rotated: false,
    offset: (
        -2.0,
        0.0,
    ),
},3856 => SpriteInfo {
    pos: (
        324,
        1680,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3577 => SpriteInfo {
    pos: (
        769,
        251,
    ),
    size: (
        48,
        100,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1587 => SpriteInfo {
    pos: (
        3993,
        184,
    ),
    size: (
        94,
        94,
    ),
    rotated: false,
    offset: (
        0.0,
        3.0,
    ),
},467 => SpriteInfo {
    pos: (
        2737,
        2312,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2418 => SpriteInfo {
    pos: (
        1105,
        2417,
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
},3555 => SpriteInfo {
    pos: (
        667,
        690,
    ),
    size: (
        8,
        56,
    ),
    rotated: false,
    offset: (
        -36.0,
        12.0,
    ),
},153 => SpriteInfo {
    pos: (
        1977,
        2156,
    ),
    size: (
        36,
        76,
    ),
    rotated: false,
    offset: (
        0.0,
        -4.0,
    ),
},3136 => SpriteInfo {
    pos: (
        2192,
        1875,
    ),
    size: (
        16,
        28,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3529 => SpriteInfo {
    pos: (
        2594,
        2721,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},473 => SpriteInfo {
    pos: (
        3978,
        448,
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
},4229 => SpriteInfo {
    pos: (
        192,
        0,
    ),
    size: (
        56,
        4,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3314 => SpriteInfo {
    pos: (
        1469,
        1723,
    ),
    size: (
        64,
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
    


pub fn get_detail_sprite(id: u32) -> Option<SpriteInfo> {
    Some(match id {
        3135 => SpriteInfo {
    pos: (
        3698,
        528,
    ),
    size: (
        16,
        28,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3370 => SpriteInfo {
    pos: (
        128,
        1214,
    ),
    size: (
        56,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},2947 => SpriteInfo {
    pos: (
        3583,
        143,
    ),
    size: (
        120,
        118,
    ),
    rotated: false,
    offset: (
        0.0,
        1.0,
    ),
},3371 => SpriteInfo {
    pos: (
        192,
        40,
    ),
    size: (
        56,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},2474 => SpriteInfo {
    pos: (
        1543,
        996,
    ),
    size: (
        32,
        16,
    ),
    rotated: false,
    offset: (
        -16.0,
        -24.0,
    ),
},4185 => SpriteInfo {
    pos: (
        3434,
        1490,
    ),
    size: (
        92,
        40,
    ),
    rotated: false,
    offset: (
        18.0,
        -4.0,
    ),
},2491 => SpriteInfo {
    pos: (
        2059,
        1163,
    ),
    size: (
        64,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        -12.0,
    ),
},4208 => SpriteInfo {
    pos: (
        2507,
        328,
    ),
    size: (
        24,
        20,
    ),
    rotated: false,
    offset: (
        0.0,
        22.0,
    ),
},3294 => SpriteInfo {
    pos: (
        2974,
        1304,
    ),
    size: (
        40,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        -4.0,
    ),
},3537 => SpriteInfo {
    pos: (
        1806,
        1199,
    ),
    size: (
        64,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2598 => SpriteInfo {
    pos: (
        64,
        739,
    ),
    size: (
        48,
        44,
    ),
    rotated: false,
    offset: (
        0.0,
        10.0,
    ),
},3576 => SpriteInfo {
    pos: (
        3742,
        392,
    ),
    size: (
        108,
        92,
    ),
    rotated: false,
    offset: (
        -6.0,
        -2.0,
    ),
},3554 => SpriteInfo {
    pos: (
        0,
        1938,
    ),
    size: (
        80,
        24,
    ),
    rotated: false,
    offset: (
        0.0,
        -8.0,
    ),
},4172 => SpriteInfo {
    pos: (
        3495,
        226,
    ),
    size: (
        60,
        36,
    ),
    rotated: false,
    offset: (
        -12.0,
        10.0,
    ),
},2566 => SpriteInfo {
    pos: (
        3119,
        998,
    ),
    size: (
        120,
        60,
    ),
    rotated: false,
    offset: (
        4.0,
        2.0,
    ),
},2549 => SpriteInfo {
    pos: (
        60,
        871,
    ),
    size: (
        40,
        60,
    ),
    rotated: false,
    offset: (
        8.0,
        -2.0,
    ),
},4300 => SpriteInfo {
    pos: (
        2312,
        1938,
    ),
    size: (
        88,
        8,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},4095 => SpriteInfo {
    pos: (
        2515,
        555,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        2.0,
        2.0,
    ),
},4072 => SpriteInfo {
    pos: (
        595,
        1479,
    ),
    size: (
        28,
        60,
    ),
    rotated: false,
    offset: (
        18.0,
        2.0,
    ),
},3330 => SpriteInfo {
    pos: (
        2604,
        1707,
    ),
    size: (
        56,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2500 => SpriteInfo {
    pos: (
        1081,
        1511,
    ),
    size: (
        48,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3376 => SpriteInfo {
    pos: (
        2158,
        2674,
    ),
    size: (
        60,
        52,
    ),
    rotated: false,
    offset: (
        -2.0,
        -2.0,
    ),
},2670 => SpriteInfo {
    pos: (
        1527,
        2371,
    ),
    size: (
        56,
        60,
    ),
    rotated: false,
    offset: (
        2.0,
        0.0,
    ),
},3231 => SpriteInfo {
    pos: (
        1395,
        2152,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2462 => SpriteInfo {
    pos: (
        2369,
        764,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        2.0,
        2.0,
    ),
},2504 => SpriteInfo {
    pos: (
        601,
        460,
    ),
    size: (
        28,
        44,
    ),
    rotated: false,
    offset: (
        18.0,
        10.0,
    ),
},4233 => SpriteInfo {
    pos: (
        3894,
        1407,
    ),
    size: (
        48,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},2505 => SpriteInfo {
    pos: (
        2071,
        68,
    ),
    size: (
        64,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3528 => SpriteInfo {
    pos: (
        3441,
        559,
    ),
    size: (
        48,
        64,
    ),
    rotated: false,
    offset: (
        8.0,
        0.0,
    ),
},3222 => SpriteInfo {
    pos: (
        2389,
        8,
    ),
    size: (
        40,
        56,
    ),
    rotated: false,
    offset: (
        -12.0,
        -4.0,
    ),
},2523 => SpriteInfo {
    pos: (
        2661,
        0,
    ),
    size: (
        104,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},3056 => SpriteInfo {
    pos: (
        4011,
        1566,
    ),
    size: (
        54,
        48,
    ),
    rotated: false,
    offset: (
        3.0,
        0.0,
    ),
},3444 => SpriteInfo {
    pos: (
        3474,
        428,
    ),
    size: (
        16,
        24,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},3545 => SpriteInfo {
    pos: (
        1327,
        592,
    ),
    size: (
        80,
        80,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2648 => SpriteInfo {
    pos: (
        2935,
        759,
    ),
    size: (
        52,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2548 => SpriteInfo {
    pos: (
        652,
        700,
    ),
    size: (
        8,
        4,
    ),
    rotated: false,
    offset: (
        -20.0,
        18.0,
    ),
},2028 => SpriteInfo {
    pos: (
        1088,
        492,
    ),
    size: (
        171,
        379,
    ),
    rotated: false,
    offset: (
        8.5,
        -3.5,
    ),
},2224 => SpriteInfo {
    pos: (
        1687,
        2740,
    ),
    size: (
        64,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},3427 => SpriteInfo {
    pos: (
        3071,
        2447,
    ),
    size: (
        36,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1852 => SpriteInfo {
    pos: (
        3499,
        122,
    ),
    size: (
        80,
        100,
    ),
    rotated: false,
    offset: (
        3.0,
        -30.0,
    ),
},3502 => SpriteInfo {
    pos: (
        0,
        1714,
    ),
    size: (
        160,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},2699 => SpriteInfo {
    pos: (
        3894,
        704,
    ),
    size: (
        56,
        52,
    ),
    rotated: false,
    offset: (
        -2.0,
        0.0,
    ),
},3375 => SpriteInfo {
    pos: (
        1623,
        2025,
    ),
    size: (
        64,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},3369 => SpriteInfo {
    pos: (
        2424,
        2666,
    ),
    size: (
        56,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},2693 => SpriteInfo {
    pos: (
        100,
        1062,
    ),
    size: (
        108,
        28,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2948 => SpriteInfo {
    pos: (
        3063,
        788,
    ),
    size: (
        118,
        118,
    ),
    rotated: false,
    offset: (
        1.0,
        1.0,
    ),
},2589 => SpriteInfo {
    pos: (
        2532,
        2556,
    ),
    size: (
        60,
        64,
    ),
    rotated: false,
    offset: (
        -2.0,
        0.0,
    ),
},3309 => SpriteInfo {
    pos: (
        1255,
        1858,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4143 => SpriteInfo {
    pos: (
        2695,
        2343,
    ),
    size: (
        32,
        16,
    ),
    rotated: false,
    offset: (
        -12.0,
        0.0,
    ),
},3001 => SpriteInfo {
    pos: (
        3641,
        1715,
    ),
    size: (
        120,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        4.0,
    ),
},2469 => SpriteInfo {
    pos: (
        959,
        622,
    ),
    size: (
        64,
        44,
    ),
    rotated: false,
    offset: (
        0.0,
        6.0,
    ),
},3372 => SpriteInfo {
    pos: (
        880,
        1763,
    ),
    size: (
        56,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2514 => SpriteInfo {
    pos: (
        247,
        812,
    ),
    size: (
        48,
        92,
    ),
    rotated: false,
    offset: (
        -20.0,
        2.0,
    ),
},2576 => SpriteInfo {
    pos: (
        1975,
        1973,
    ),
    size: (
        56,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        -12.0,
    ),
},3478 => SpriteInfo {
    pos: (
        2600,
        1996,
    ),
    size: (
        60,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        -4.0,
    ),
},1863 => SpriteInfo {
    pos: (
        1395,
        2435,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2591 => SpriteInfo {
    pos: (
        3171,
        1513,
    ),
    size: (
        64,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},3526 => SpriteInfo {
    pos: (
        1080,
        923,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        -2.0,
        -2.0,
    ),
},4191 => SpriteInfo {
    pos: (
        4085,
        2090,
    ),
    size: (
        4,
        16,
    ),
    rotated: false,
    offset: (
        -8.0,
        -4.0,
    ),
},2554 => SpriteInfo {
    pos: (
        60,
        787,
    ),
    size: (
        48,
        52,
    ),
    rotated: false,
    offset: (
        8.0,
        -6.0,
    ),
},3362 => SpriteInfo {
    pos: (
        4030,
        954,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        -2.0,
        2.0,
    ),
},3364 => SpriteInfo {
    pos: (
        3502,
        941,
    ),
    size: (
        4,
        4,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3151 => SpriteInfo {
    pos: (
        1041,
        2553,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3601 => SpriteInfo {
    pos: (
        3434,
        1534,
    ),
    size: (
        92,
        92,
    ),
    rotated: false,
    offset: (
        0.0,
        1.0,
    ),
},3124 => SpriteInfo {
    pos: (
        2406,
        2492,
    ),
    size: (
        52,
        64,
    ),
    rotated: false,
    offset: (
        6.0,
        0.0,
    ),
},878 => SpriteInfo {
    pos: (
        652,
        690,
    ),
    size: (
        11,
        6,
    ),
    rotated: false,
    offset: (
        70.5,
        -26.0,
    ),
},3221 => SpriteInfo {
    pos: (
        3161,
        532,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        -2.0,
        -2.0,
    ),
},2463 => SpriteInfo {
    pos: (
        3905,
        2740,
    ),
    size: (
        64,
        40,
    ),
    rotated: false,
    offset: (
        0.0,
        -8.0,
    ),
},891 => SpriteInfo {
    pos: (
        841,
        610,
    ),
    size: (
        114,
        54,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2582 => SpriteInfo {
    pos: (
        0,
        2006,
    ),
    size: (
        44,
        48,
    ),
    rotated: false,
    offset: (
        -6.0,
        8.0,
    ),
},1865 => SpriteInfo {
    pos: (
        372,
        522,
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
},1849 => SpriteInfo {
    pos: (
        164,
        1814,
    ),
    size: (
        143,
        63,
    ),
    rotated: false,
    offset: (
        -18.5,
        -8.5,
    ),
},3122 => SpriteInfo {
    pos: (
        3144,
        2183,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2657 => SpriteInfo {
    pos: (
        3482,
        2204,
    ),
    size: (
        52,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},2700 => SpriteInfo {
    pos: (
        3253,
        1028,
    ),
    size: (
        56,
        52,
    ),
    rotated: false,
    offset: (
        -2.0,
        0.0,
    ),
},2385 => SpriteInfo {
    pos: (
        3338,
        1539,
    ),
    size: (
        64,
        36,
    ),
    rotated: false,
    offset: (
        0.0,
        6.0,
    ),
},3325 => SpriteInfo {
    pos: (
        0,
        779,
    ),
    size: (
        56,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2887 => SpriteInfo {
    pos: (
        635,
        2556,
    ),
    size: (
        256,
        103,
    ),
    rotated: false,
    offset: (
        -5.0,
        2.5,
    ),
},3501 => SpriteInfo {
    pos: (
        2359,
        328,
    ),
    size: (
        144,
        116,
    ),
    rotated: false,
    offset: (
        2.0,
        2.0,
    ),
},4173 => SpriteInfo {
    pos: (
        1855,
        1075,
    ),
    size: (
        56,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2681 => SpriteInfo {
    pos: (
        3137,
        1094,
    ),
    size: (
        52,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3549 => SpriteInfo {
    pos: (
        1413,
        1782,
    ),
    size: (
        24,
        80,
    ),
    rotated: false,
    offset: (
        -28.0,
        0.0,
    ),
},2510 => SpriteInfo {
    pos: (
        2781,
        915,
    ),
    size: (
        60,
        32,
    ),
    rotated: false,
    offset: (
        -2.0,
        0.0,
    ),
},4065 => SpriteInfo {
    pos: (
        3901,
        1910,
    ),
    size: (
        56,
        20,
    ),
    rotated: false,
    offset: (
        0.0,
        22.0,
    ),
},2558 => SpriteInfo {
    pos: (
        3493,
        853,
    ),
    size: (
        36,
        36,
    ),
    rotated: false,
    offset: (
        14.0,
        10.0,
    ),
},3536 => SpriteInfo {
    pos: (
        2604,
        1647,
    ),
    size: (
        60,
        56,
    ),
    rotated: false,
    offset: (
        2.0,
        0.0,
    ),
},2250 => SpriteInfo {
    pos: (
        3945,
        2556,
    ),
    size: (
        104,
        44,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},4178 => SpriteInfo {
    pos: (
        895,
        2496,
    ),
    size: (
        4,
        4,
    ),
    rotated: false,
    offset: (
        -8.0,
        0.0,
    ),
},3086 => SpriteInfo {
    pos: (
        2917,
        199,
    ),
    size: (
        111,
        95,
    ),
    rotated: false,
    offset: (
        0.5,
        2.5,
    ),
},2465 => SpriteInfo {
    pos: (
        2141,
        0,
    ),
    size: (
        48,
        64,
    ),
    rotated: false,
    offset: (
        -4.0,
        0.0,
    ),
},4006 => SpriteInfo {
    pos: (
        2662,
        2773,
    ),
    size: (
        64,
        4,
    ),
    rotated: false,
    offset: (
        0.0,
        -26.0,
    ),
},3404 => SpriteInfo {
    pos: (
        2721,
        585,
    ),
    size: (
        20,
        12,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3455 => SpriteInfo {
    pos: (
        237,
        1381,
    ),
    size: (
        44,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2473 => SpriteInfo {
    pos: (
        3954,
        698,
    ),
    size: (
        120,
        60,
    ),
    rotated: false,
    offset: (
        4.0,
        2.0,
    ),
},2311 => SpriteInfo {
    pos: (
        2084,
        1626,
    ),
    size: (
        12,
        44,
    ),
    rotated: false,
    offset: (
        4.0,
        0.0,
    ),
},2654 => SpriteInfo {
    pos: (
        2998,
        956,
    ),
    size: (
        52,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},2367 => SpriteInfo {
    pos: (
        3327,
        1022,
    ),
    size: (
        64,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2310 => SpriteInfo {
    pos: (
        2043,
        1913,
    ),
    size: (
        12,
        44,
    ),
    rotated: false,
    offset: (
        4.0,
        0.0,
    ),
},3448 => SpriteInfo {
    pos: (
        2587,
        441,
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
},412 => SpriteInfo {
    pos: (
        296,
        2072,
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
},2578 => SpriteInfo {
    pos: (
        2475,
        2116,
    ),
    size: (
        56,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3521 => SpriteInfo {
    pos: (
        909,
        542,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        2.0,
        2.0,
    ),
},3318 => SpriteInfo {
    pos: (
        3931,
        304,
    ),
    size: (
        56,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},4226 => SpriteInfo {
    pos: (
        2278,
        1228,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        2.0,
        -2.0,
    ),
},3319 => SpriteInfo {
    pos: (
        372,
        1988,
    ),
    size: (
        56,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2572 => SpriteInfo {
    pos: (
        2423,
        1056,
    ),
    size: (
        64,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        -12.0,
    ),
},3367 => SpriteInfo {
    pos: (
        1133,
        1403,
    ),
    size: (
        64,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        -4.0,
    ),
},1862 => SpriteInfo {
    pos: (
        1595,
        1609,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3146 => SpriteInfo {
    pos: (
        3885,
        1934,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2509 => SpriteInfo {
    pos: (
        3387,
        2744,
    ),
    size: (
        64,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2515 => SpriteInfo {
    pos: (
        2379,
        2030,
    ),
    size: (
        92,
        92,
    ),
    rotated: false,
    offset: (
        2.0,
        2.0,
    ),
},2482 => SpriteInfo {
    pos: (
        1413,
        1682,
    ),
    size: (
        52,
        56,
    ),
    rotated: false,
    offset: (
        6.0,
        0.0,
    ),
},4097 => SpriteInfo {
    pos: (
        2342,
        1276,
    ),
    size: (
        60,
        64,
    ),
    rotated: false,
    offset: (
        2.0,
        0.0,
    ),
},4229 => SpriteInfo {
    pos: (
        252,
        0,
    ),
    size: (
        48,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        -4.0,
    ),
},4151 => SpriteInfo {
    pos: (
        1799,
        855,
    ),
    size: (
        60,
        36,
    ),
    rotated: false,
    offset: (
        -8.0,
        0.0,
    ),
},3465 => SpriteInfo {
    pos: (
        256,
        1281,
    ),
    size: (
        40,
        36,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},2556 => SpriteInfo {
    pos: (
        2175,
        771,
    ),
    size: (
        64,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},413 => SpriteInfo {
    pos: (
        3054,
        1014,
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
},2279 => SpriteInfo {
    pos: (
        586,
        2691,
    ),
    size: (
        40,
        116,
    ),
    rotated: false,
    offset: (
        -12.0,
        6.0,
    ),
},4165 => SpriteInfo {
    pos: (
        3422,
        893,
    ),
    size: (
        88,
        44,
    ),
    rotated: false,
    offset: (
        32.0,
        4.0,
    ),
},410 => SpriteInfo {
    pos: (
        1169,
        2417,
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
},3312 => SpriteInfo {
    pos: (
        2295,
        296,
    ),
    size: (
        60,
        64,
    ),
    rotated: false,
    offset: (
        2.0,
        0.0,
    ),
},2553 => SpriteInfo {
    pos: (
        2861,
        2356,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        -2.0,
        -2.0,
    ),
},3331 => SpriteInfo {
    pos: (
        1638,
        92,
    ),
    size: (
        56,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1463 => SpriteInfo {
    pos: (
        3711,
        2140,
    ),
    size: (
        38,
        42,
    ),
    rotated: false,
    offset: (
        31.0,
        4.0,
    ),
},3517 => SpriteInfo {
    pos: (
        3225,
        468,
    ),
    size: (
        44,
        64,
    ),
    rotated: false,
    offset: (
        -6.0,
        0.0,
    ),
},4231 => SpriteInfo {
    pos: (
        1927,
        520,
    ),
    size: (
        56,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4135 => SpriteInfo {
    pos: (
        363,
        2667,
    ),
    size: (
        40,
        36,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},2689 => SpriteInfo {
    pos: (
        2389,
        68,
    ),
    size: (
        44,
        4,
    ),
    rotated: false,
    offset: (
        18.0,
        0.0,
    ),
},2421 => SpriteInfo {
    pos: (
        408,
        1748,
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
},2697 => SpriteInfo {
    pos: (
        3341,
        2281,
    ),
    size: (
        52,
        44,
    ),
    rotated: false,
    offset: (
        0.0,
        -4.0,
    ),
},2551 => SpriteInfo {
    pos: (
        621,
        594,
    ),
    size: (
        64,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},218 => SpriteInfo {
    pos: (
        1142,
        2321,
    ),
    size: (
        64,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},2414 => SpriteInfo {
    pos: (
        1927,
        472,
    ),
    size: (
        56,
        36,
    ),
    rotated: false,
    offset: (
        -2.0,
        -8.0,
    ),
},2063 => SpriteInfo {
    pos: (
        1530,
        1461,
    ),
    size: (
        86,
        134,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3366 => SpriteInfo {
    pos: (
        1021,
        434,
    ),
    size: (
        60,
        56,
    ),
    rotated: false,
    offset: (
        2.0,
        -4.0,
    ),
},3247 => SpriteInfo {
    pos: (
        3882,
        2228,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4230 => SpriteInfo {
    pos: (
        1803,
        1023,
    ),
    size: (
        48,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        -4.0,
    ),
},2488 => SpriteInfo {
    pos: (
        2885,
        0,
    ),
    size: (
        60,
        32,
    ),
    rotated: false,
    offset: (
        -2.0,
        -12.0,
    ),
},4175 => SpriteInfo {
    pos: (
        1735,
        721,
    ),
    size: (
        84,
        54,
    ),
    rotated: false,
    offset: (
        2.0,
        -5.0,
    ),
},1275 => SpriteInfo {
    pos: (
        3073,
        0,
    ),
    size: (
        110,
        61,
    ),
    rotated: false,
    offset: (
        0.0,
        -1.5,
    ),
},3057 => SpriteInfo {
    pos: (
        2379,
        2218,
    ),
    size: (
        54,
        48,
    ),
    rotated: false,
    offset: (
        3.0,
        0.0,
    ),
},2694 => SpriteInfo {
    pos: (
        3347,
        1105,
    ),
    size: (
        132,
        112,
    ),
    rotated: false,
    offset: (
        0.0,
        6.0,
    ),
},2574 => SpriteInfo {
    pos: (
        3633,
        2453,
    ),
    size: (
        60,
        32,
    ),
    rotated: false,
    offset: (
        -2.0,
        -12.0,
    ),
},1600 => SpriteInfo {
    pos: (
        2957,
        1468,
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
},2592 => SpriteInfo {
    pos: (
        2145,
        2307,
    ),
    size: (
        64,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},3055 => SpriteInfo {
    pos: (
        2425,
        2396,
    ),
    size: (
        54,
        48,
    ),
    rotated: false,
    offset: (
        3.0,
        0.0,
    ),
},3136 => SpriteInfo {
    pos: (
        288,
        1881,
    ),
    size: (
        16,
        28,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3516 => SpriteInfo {
    pos: (
        533,
        460,
    ),
    size: (
        64,
        44,
    ),
    rotated: false,
    offset: (
        0.0,
        -6.0,
    ),
},4324 => SpriteInfo {
    pos: (
        311,
        1832,
    ),
    size: (
        64,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},4353 => SpriteInfo {
    pos: (
        1609,
        0,
    ),
    size: (
        20,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},2891 => SpriteInfo {
    pos: (
        2264,
        2560,
    ),
    size: (
        264,
        102,
    ),
    rotated: false,
    offset: (
        -1.0,
        0.0,
    ),
},4001 => SpriteInfo {
    pos: (
        1573,
        1889,
    ),
    size: (
        64,
        44,
    ),
    rotated: false,
    offset: (
        0.0,
        -6.0,
    ),
},4169 => SpriteInfo {
    pos: (
        1177,
        2129,
    ),
    size: (
        28,
        8,
    ),
    rotated: false,
    offset: (
        -4.0,
        26.0,
    ),
},2541 => SpriteInfo {
    pos: (
        1426,
        1126,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        2.0,
        2.0,
    ),
},1885 => SpriteInfo {
    pos: (
        124,
        510,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1861 => SpriteInfo {
    pos: (
        460,
        672,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2490 => SpriteInfo {
    pos: (
        2427,
        828,
    ),
    size: (
        60,
        32,
    ),
    rotated: false,
    offset: (
        2.0,
        -12.0,
    ),
},4188 => SpriteInfo {
    pos: (
        4074,
        1399,
    ),
    size: (
        16,
        36,
    ),
    rotated: false,
    offset: (
        -8.0,
        -14.0,
    ),
},2470 => SpriteInfo {
    pos: (
        3409,
        739,
    ),
    size: (
        56,
        60,
    ),
    rotated: false,
    offset: (
        -4.0,
        -2.0,
    ),
},3311 => SpriteInfo {
    pos: (
        124,
        1445,
    ),
    size: (
        60,
        64,
    ),
    rotated: false,
    offset: (
        2.0,
        0.0,
    ),
},2413 => SpriteInfo {
    pos: (
        64,
        1566,
    ),
    size: (
        56,
        56,
    ),
    rotated: false,
    offset: (
        -2.0,
        2.0,
    ),
},3527 => SpriteInfo {
    pos: (
        3471,
        1376,
    ),
    size: (
        64,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        8.0,
    ),
},3555 => SpriteInfo {
    pos: (
        667,
        750,
    ),
    size: (
        12,
        24,
    ),
    rotated: false,
    offset: (
        -34.0,
        -28.0,
    ),
},3795 => SpriteInfo {
    pos: (
        2871,
        767,
    ),
    size: (
        56,
        28,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},1709 => SpriteInfo {
    pos: (
        3867,
        1575,
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
},1734 => SpriteInfo {
    pos: (
        943,
        1579,
    ),
    size: (
        308,
        310,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3361 => SpriteInfo {
    pos: (
        220,
        1881,
    ),
    size: (
        64,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},3313 => SpriteInfo {
    pos: (
        2261,
        0,
    ),
    size: (
        60,
        16,
    ),
    rotated: false,
    offset: (
        2.0,
        -8.0,
    ),
},3147 => SpriteInfo {
    pos: (
        2304,
        1785,
    ),
    size: (
        40,
        64,
    ),
    rotated: false,
    offset: (
        -12.0,
        0.0,
    ),
},3220 => SpriteInfo {
    pos: (
        2115,
        260,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2894 => SpriteInfo {
    pos: (
        1966,
        680,
    ),
    size: (
        285,
        87,
    ),
    rotated: false,
    offset: (
        0.5,
        4.5,
    ),
},1707 => SpriteInfo {
    pos: (
        2914,
        468,
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
},2632 => SpriteInfo {
    pos: (
        2633,
        2048,
    ),
    size: (
        12,
        20,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1589 => SpriteInfo {
    pos: (
        1518,
        1072,
    ),
    size: (
        60,
        84,
    ),
    rotated: false,
    offset: (
        -11.0,
        2.0,
    ),
},4071 => SpriteInfo {
    pos: (
        1073,
        1295,
    ),
    size: (
        56,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},2251 => SpriteInfo {
    pos: (
        568,
        1307,
    ),
    size: (
        56,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4228 => SpriteInfo {
    pos: (
        564,
        596,
    ),
    size: (
        48,
        4,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4085 => SpriteInfo {
    pos: (
        627,
        882,
    ),
    size: (
        56,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},2278 => SpriteInfo {
    pos: (
        1041,
        2621,
    ),
    size: (
        52,
        124,
    ),
    rotated: false,
    offset: (
        -26.0,
        2.0,
    ),
},3329 => SpriteInfo {
    pos: (
        0,
        68,
    ),
    size: (
        56,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2503 => SpriteInfo {
    pos: (
        3004,
        516,
    ),
    size: (
        64,
        40,
    ),
    rotated: false,
    offset: (
        0.0,
        12.0,
    ),
},4150 => SpriteInfo {
    pos: (
        1141,
        0,
    ),
    size: (
        8,
        12,
    ),
    rotated: false,
    offset: (
        8.0,
        2.0,
    ),
},2229 => SpriteInfo {
    pos: (
        564,
        2257,
    ),
    size: (
        64,
        28,
    ),
    rotated: false,
    offset: (
        0.0,
        -18.0,
    ),
},1697 => SpriteInfo {
    pos: (
        1620,
        1517,
    ),
    size: (
        120,
        88,
    ),
    rotated: false,
    offset: (
        0.0,
        16.0,
    ),
},3476 => SpriteInfo {
    pos: (
        901,
        1991,
    ),
    size: (
        20,
        20,
    ),
    rotated: false,
    offset: (
        0.0,
        -16.0,
    ),
},3423 => SpriteInfo {
    pos: (
        376,
        366,
    ),
    size: (
        28,
        40,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},4098 => SpriteInfo {
    pos: (
        0,
        1630,
    ),
    size: (
        36,
        40,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},2571 => SpriteInfo {
    pos: (
        620,
        654,
    ),
    size: (
        64,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        -12.0,
    ),
},1464 => SpriteInfo {
    pos: (
        1020,
        150,
    ),
    size: (
        63,
        24,
    ),
    rotated: false,
    offset: (
        11.5,
        15.0,
    ),
},2585 => SpriteInfo {
    pos: (
        3781,
        2740,
    ),
    size: (
        52,
        56,
    ),
    rotated: false,
    offset: (
        6.0,
        -4.0,
    ),
},1701 => SpriteInfo {
    pos: (
        3471,
        1428,
    ),
    size: (
        58,
        58,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},877 => SpriteInfo {
    pos: (
        3781,
        1641,
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
},4086 => SpriteInfo {
    pos: (
        1751,
        2692,
    ),
    size: (
        28,
        28,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},3426 => SpriteInfo {
    pos: (
        256,
        1225,
    ),
    size: (
        40,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},2479 => SpriteInfo {
    pos: (
        172,
        359,
    ),
    size: (
        64,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        -4.0,
    ),
},2512 => SpriteInfo {
    pos: (
        172,
        426,
    ),
    size: (
        72,
        80,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4168 => SpriteInfo {
    pos: (
        861,
        2727,
    ),
    size: (
        28,
        8,
    ),
    rotated: false,
    offset: (
        -4.0,
        26.0,
    ),
},4174 => SpriteInfo {
    pos: (
        1543,
        936,
    ),
    size: (
        56,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2513 => SpriteInfo {
    pos: (
        3406,
        2196,
    ),
    size: (
        72,
        80,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2568 => SpriteInfo {
    pos: (
        2801,
        44,
    ),
    size: (
        64,
        28,
    ),
    rotated: false,
    offset: (
        0.0,
        -18.0,
    ),
},3577 => SpriteInfo {
    pos: (
        1847,
        124,
    ),
    size: (
        8,
        44,
    ),
    rotated: false,
    offset: (
        -4.0,
        8.0,
    ),
},3535 => SpriteInfo {
    pos: (
        1441,
        1835,
    ),
    size: (
        60,
        56,
    ),
    rotated: false,
    offset: (
        2.0,
        0.0,
    ),
},2627 => SpriteInfo {
    pos: (
        340,
        634,
    ),
    size: (
        56,
        52,
    ),
    rotated: false,
    offset: (
        -2.0,
        0.0,
    ),
},3233 => SpriteInfo {
    pos: (
        1358,
        1138,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3518 => SpriteInfo {
    pos: (
        3533,
        2606,
    ),
    size: (
        44,
        64,
    ),
    rotated: false,
    offset: (
        -6.0,
        0.0,
    ),
},1703 => SpriteInfo {
    pos: (
        814,
        1912,
    ),
    size: (
        62,
        53,
    ),
    rotated: false,
    offset: (
        0.0,
        -1.5,
    ),
},3532 => SpriteInfo {
    pos: (
        2766,
        1349,
    ),
    size: (
        56,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},2696 => SpriteInfo {
    pos: (
        1424,
        1519,
    ),
    size: (
        52,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3481 => SpriteInfo {
    pos: (
        1847,
        184,
    ),
    size: (
        12,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        4.0,
    ),
},3523 => SpriteInfo {
    pos: (
        2394,
        1638,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3323 => SpriteInfo {
    pos: (
        2175,
        835,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        2.0,
        -2.0,
    ),
},2517 => SpriteInfo {
    pos: (
        428,
        2321,
    ),
    size: (
        92,
        92,
    ),
    rotated: false,
    offset: (
        -2.0,
        2.0,
    ),
},1868 => SpriteInfo {
    pos: (
        84,
        1949,
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
},4000 => SpriteInfo {
    pos: (
        2081,
        2287,
    ),
    size: (
        60,
        44,
    ),
    rotated: false,
    offset: (
        2.0,
        -6.0,
    ),
},2472 => SpriteInfo {
    pos: (
        1467,
        1919,
    ),
    size: (
        20,
        20,
    ),
    rotated: false,
    offset: (
        -22.0,
        -22.0,
    ),
},2889 => SpriteInfo {
    pos: (
        2069,
        2371,
    ),
    size: (
        267,
        102,
    ),
    rotated: false,
    offset: (
        -5.5,
        8.0,
    ),
},3551 => SpriteInfo {
    pos: (
        1880,
        1135,
    ),
    size: (
        24,
        80,
    ),
    rotated: false,
    offset: (
        -28.0,
        0.0,
    ),
},2660 => SpriteInfo {
    pos: (
        2711,
        525,
    ),
    size: (
        36,
        56,
    ),
    rotated: false,
    offset: (
        -12.0,
        -2.0,
    ),
},3519 => SpriteInfo {
    pos: (
        2007,
        62,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        -2.0,
        -2.0,
    ),
},2501 => SpriteInfo {
    pos: (
        885,
        474,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2355 => SpriteInfo {
    pos: (
        4038,
        634,
    ),
    size: (
        48,
        20,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},2361 => SpriteInfo {
    pos: (
        3462,
        2280,
    ),
    size: (
        56,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},217 => SpriteInfo {
    pos: (
        641,
        192,
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
},2538 => SpriteInfo {
    pos: (
        627,
        942,
    ),
    size: (
        52,
        16,
    ),
    rotated: false,
    offset: (
        0.0,
        18.0,
    ),
},4068 => SpriteInfo {
    pos: (
        4063,
        64,
    ),
    size: (
        16,
        20,
    ),
    rotated: false,
    offset: (
        -20.0,
        22.0,
    ),
},2249 => SpriteInfo {
    pos: (
        1081,
        1123,
    ),
    size: (
        48,
        44,
    ),
    rotated: false,
    offset: (
        -2.0,
        2.0,
    ),
},2945 => SpriteInfo {
    pos: (
        179,
        2627,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2584 => SpriteInfo {
    pos: (
        2361,
        2270,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3138 => SpriteInfo {
    pos: (
        1897,
        716,
    ),
    size: (
        44,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1708 => SpriteInfo {
    pos: (
        3371,
        124,
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
},3324 => SpriteInfo {
    pos: (
        1021,
        494,
    ),
    size: (
        56,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3229 => SpriteInfo {
    pos: (
        1462,
        1004,
    ),
    size: (
        64,
        28,
    ),
    rotated: false,
    offset: (
        0.0,
        -18.0,
    ),
},4070 => SpriteInfo {
    pos: (
        1717,
        1889,
    ),
    size: (
        16,
        20,
    ),
    rotated: false,
    offset: (
        20.0,
        22.0,
    ),
},3234 => SpriteInfo {
    pos: (
        1083,
        1893,
    ),
    size: (
        60,
        52,
    ),
    rotated: false,
    offset: (
        2.0,
        -2.0,
    ),
},1614 => SpriteInfo {
    pos: (
        1427,
        512,
    ),
    size: (
        76,
        75,
    ),
    rotated: false,
    offset: (
        0.0,
        0.5,
    ),
},4325 => SpriteInfo {
    pos: (
        1743,
        397,
    ),
    size: (
        64,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},3227 => SpriteInfo {
    pos: (
        3395,
        1069,
    ),
    size: (
        64,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        -16.0,
    ),
},2580 => SpriteInfo {
    pos: (
        1387,
        755,
    ),
    size: (
        56,
        56,
    ),
    rotated: false,
    offset: (
        4.0,
        4.0,
    ),
},2362 => SpriteInfo {
    pos: (
        0,
        2098,
    ),
    size: (
        40,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1053 => SpriteInfo {
    pos: (
        2781,
        951,
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
},1859 => SpriteInfo {
    pos: (
        2604,
        1785,
    ),
    size: (
        204,
        91,
    ),
    rotated: false,
    offset: (
        -38.0,
        -1.5,
    ),
},3360 => SpriteInfo {
    pos: (
        2136,
        1739,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        2.0,
        2.0,
    ),
},2704 => SpriteInfo {
    pos: (
        1735,
        433,
    ),
    size: (
        120,
        118,
    ),
    rotated: false,
    offset: (
        0.0,
        -1.0,
    ),
},2946 => SpriteInfo {
    pos: (
        471,
        1443,
    ),
    size: (
        120,
        116,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2476 => SpriteInfo {
    pos: (
        1073,
        1359,
    ),
    size: (
        48,
        28,
    ),
    rotated: false,
    offset: (
        -8.0,
        -18.0,
    ),
},2354 => SpriteInfo {
    pos: (
        862,
        126,
    ),
    size: (
        40,
        48,
    ),
    rotated: false,
    offset: (
        -10.0,
        -6.0,
    ),
},2690 => SpriteInfo {
    pos: (
        537,
        64,
    ),
    size: (
        128,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},4087 => SpriteInfo {
    pos: (
        2721,
        909,
    ),
    size: (
        56,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},4005 => SpriteInfo {
    pos: (
        1743,
        305,
    ),
    size: (
        64,
        4,
    ),
    rotated: false,
    offset: (
        0.0,
        -26.0,
    ),
},2665 => SpriteInfo {
    pos: (
        1530,
        1228,
    ),
    size: (
        36,
        60,
    ),
    rotated: false,
    offset: (
        -12.0,
        0.0,
    ),
},2581 => SpriteInfo {
    pos: (
        1806,
        1259,
    ),
    size: (
        60,
        52,
    ),
    rotated: false,
    offset: (
        2.0,
        6.0,
    ),
},2663 => SpriteInfo {
    pos: (
        3995,
        68,
    ),
    size: (
        60,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},3403 => SpriteInfo {
    pos: (
        3447,
        508,
    ),
    size: (
        20,
        16,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},4007 => SpriteInfo {
    pos: (
        893,
        1246,
    ),
    size: (
        60,
        56,
    ),
    rotated: false,
    offset: (
        2.0,
        0.0,
    ),
},2419 => SpriteInfo {
    pos: (
        1133,
        1531,
    ),
    size: (
        36,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},3578 => SpriteInfo {
    pos: (
        283,
        76,
    ),
    size: (
        8,
        44,
    ),
    rotated: false,
    offset: (
        -4.0,
        8.0,
    ),
},3320 => SpriteInfo {
    pos: (
        1255,
        1611,
    ),
    size: (
        56,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},3479 => SpriteInfo {
    pos: (
        1141,
        2385,
    ),
    size: (
        44,
        28,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},3794 => SpriteInfo {
    pos: (
        2721,
        997,
    ),
    size: (
        56,
        44,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},3129 => SpriteInfo {
    pos: (
        2064,
        128,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        -2.0,
        2.0,
    ),
},2564 => SpriteInfo {
    pos: (
        2427,
        928,
    ),
    size: (
        32,
        32,
    ),
    rotated: false,
    offset: (
        -16.0,
        -16.0,
    ),
},2570 => SpriteInfo {
    pos: (
        3410,
        272,
    ),
    size: (
        60,
        32,
    ),
    rotated: false,
    offset: (
        2.0,
        -12.0,
    ),
},3582 => SpriteInfo {
    pos: (
        1605,
        1733,
    ),
    size: (
        120,
        28,
    ),
    rotated: false,
    offset: (
        -2.0,
        -18.0,
    ),
},2477 => SpriteInfo {
    pos: (
        2845,
        983,
    ),
    size: (
        64,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},3550 => SpriteInfo {
    pos: (
        376,
        410,
    ),
    size: (
        24,
        60,
    ),
    rotated: false,
    offset: (
        -28.0,
        -10.0,
    ),
},2511 => SpriteInfo {
    pos: (
        2183,
        316,
    ),
    size: (
        84,
        76,
    ),
    rotated: false,
    offset: (
        6.0,
        -2.0,
    ),
},2481 => SpriteInfo {
    pos: (
        1479,
        659,
    ),
    size: (
        24,
        12,
    ),
    rotated: false,
    offset: (
        0.0,
        -22.0,
    ),
},4066 => SpriteInfo {
    pos: (
        905,
        2079,
    ),
    size: (
        16,
        60,
    ),
    rotated: false,
    offset: (
        -20.0,
        2.0,
    ),
},2542 => SpriteInfo {
    pos: (
        2363,
        952,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        2.0,
        2.0,
    ),
},4003 => SpriteInfo {
    pos: (
        2604,
        1578,
    ),
    size: (
        60,
        44,
    ),
    rotated: false,
    offset: (
        -2.0,
        -6.0,
    ),
},1461 => SpriteInfo {
    pos: (
        1638,
        180,
    ),
    size: (
        120,
        111,
    ),
    rotated: false,
    offset: (
        0.0,
        3.5,
    ),
},3531 => SpriteInfo {
    pos: (
        2600,
        1944,
    ),
    size: (
        48,
        48,
    ),
    rotated: false,
    offset: (
        -8.0,
        -8.0,
    ),
},1597 => SpriteInfo {
    pos: (
        2604,
        1626,
    ),
    size: (
        57,
        17,
    ),
    rotated: false,
    offset: (
        0.5,
        12.5,
    ),
},4076 => SpriteInfo {
    pos: (
        3977,
        2604,
    ),
    size: (
        72,
        88,
    ),
    rotated: false,
    offset: (
        2.0,
        2.0,
    ),
},3553 => SpriteInfo {
    pos: (
        1623,
        2081,
    ),
    size: (
        80,
        24,
    ),
    rotated: false,
    offset: (
        0.0,
        -8.0,
    ),
},409 => SpriteInfo {
    pos: (
        1853,
        2032,
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
},216 => SpriteInfo {
    pos: (
        447,
        2595,
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
},3150 => SpriteInfo {
    pos: (
        2278,
        2478,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2363 => SpriteInfo {
    pos: (
        1085,
        1055,
    ),
    size: (
        40,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3226 => SpriteInfo {
    pos: (
        2841,
        1047,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2499 => SpriteInfo {
    pos: (
        2922,
        1304,
    ),
    size: (
        48,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2603 => SpriteInfo {
    pos: (
        1530,
        1352,
    ),
    size: (
        36,
        24,
    ),
    rotated: false,
    offset: (
        -10.0,
        20.0,
    ),
},1736 => SpriteInfo {
    pos: (
        1263,
        815,
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
},3088 => SpriteInfo {
    pos: (
        1915,
        1173,
    ),
    size: (
        116,
        78,
    ),
    rotated: false,
    offset: (
        0.0,
        1.0,
    ),
},2668 => SpriteInfo {
    pos: (
        1387,
        727,
    ),
    size: (
        60,
        24,
    ),
    rotated: false,
    offset: (
        0.0,
        18.0,
    ),
},3314 => SpriteInfo {
    pos: (
        3088,
        1726,
    ),
    size: (
        64,
        16,
    ),
    rotated: false,
    offset: (
        0.0,
        -8.0,
    ),
},2672 => SpriteInfo {
    pos: (
        1705,
        0,
    ),
    size: (
        60,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},3520 => SpriteInfo {
    pos: (
        1263,
        556,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        2.0,
        2.0,
    ),
},2552 => SpriteInfo {
    pos: (
        0,
        679,
    ),
    size: (
        64,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        4.0,
    ),
},2647 => SpriteInfo {
    pos: (
        616,
        0,
    ),
    size: (
        60,
        20,
    ),
    rotated: false,
    offset: (
        0.0,
        16.0,
    ),
},3310 => SpriteInfo {
    pos: (
        2617,
        955,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},458 => SpriteInfo {
    pos: (
        616,
        2289,
    ),
    size: (
        33,
        34,
    ),
    rotated: false,
    offset: (
        -2.5,
        3.0,
    ),
},4227 => SpriteInfo {
    pos: (
        287,
        908,
    ),
    size: (
        4,
        4,
    ),
    rotated: false,
    offset: (
        0.0,
        26.0,
    ),
},3228 => SpriteInfo {
    pos: (
        633,
        486,
    ),
    size: (
        64,
        36,
    ),
    rotated: false,
    offset: (
        0.0,
        -14.0,
    ),
},3232 => SpriteInfo {
    pos: (
        628,
        1143,
    ),
    size: (
        32,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2498 => SpriteInfo {
    pos: (
        572,
        1375,
    ),
    size: (
        48,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3230 => SpriteInfo {
    pos: (
        3149,
        2365,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2683 => SpriteInfo {
    pos: (
        0,
        168,
    ),
    size: (
        52,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},4323 => SpriteInfo {
    pos: (
        3523,
        2104,
    ),
    size: (
        60,
        32,
    ),
    rotated: false,
    offset: (
        2.0,
        -2.0,
    ),
},2867 => SpriteInfo {
    pos: (
        364,
        1578,
    ),
    size: (
        81,
        98,
    ),
    rotated: false,
    offset: (
        11.5,
        16.0,
    ),
},2562 => SpriteInfo {
    pos: (
        2146,
        2477,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        2.0,
        2.0,
    ),
},4357 => SpriteInfo {
    pos: (
        2801,
        0,
    ),
    size: (
        12,
        28,
    ),
    rotated: false,
    offset: (
        -4.0,
        -2.0,
    ),
},2416 => SpriteInfo {
    pos: (
        173,
        1381,
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
},2489 => SpriteInfo {
    pos: (
        2278,
        1192,
    ),
    size: (
        60,
        32,
    ),
    rotated: false,
    offset: (
        -2.0,
        -12.0,
    ),
},2301 => SpriteInfo {
    pos: (
        2101,
        2231,
    ),
    size: (
        52,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2698 => SpriteInfo {
    pos: (
        355,
        302,
    ),
    size: (
        52,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3552 => SpriteInfo {
    pos: (
        0,
        1874,
    ),
    size: (
        80,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        -10.0,
    ),
},1598 => SpriteInfo {
    pos: (
        701,
        486,
    ),
    size: (
        57,
        17,
    ),
    rotated: false,
    offset: (
        0.5,
        9.5,
    ),
},3446 => SpriteInfo {
    pos: (
        2122,
        1199,
    ),
    size: (
        16,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3365 => SpriteInfo {
    pos: (
        1812,
        1521,
    ),
    size: (
        60,
        64,
    ),
    rotated: false,
    offset: (
        -2.0,
        0.0,
    ),
},2312 => SpriteInfo {
    pos: (
        3978,
        428,
    ),
    size: (
        8,
        16,
    ),
    rotated: false,
    offset: (
        2.0,
        2.0,
    ),
},3058 => SpriteInfo {
    pos: (
        413,
        1390,
    ),
    size: (
        54,
        48,
    ),
    rotated: false,
    offset: (
        3.0,
        0.0,
    ),
},2631 => SpriteInfo {
    pos: (
        3811,
        288,
    ),
    size: (
        48,
        52,
    ),
    rotated: false,
    offset: (
        8.0,
        -6.0,
    ),
},4096 => SpriteInfo {
    pos: (
        1021,
        306,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        -2.0,
        2.0,
    ),
},2692 => SpriteInfo {
    pos: (
        2957,
        1460,
    ),
    size: (
        128,
        4,
    ),
    rotated: false,
    offset: (
        0.0,
        4.0,
    ),
},1699 => SpriteInfo {
    pos: (
        2766,
        1239,
    ),
    size: (
        92,
        106,
    ),
    rotated: false,
    offset: (
        0.0,
        7.0,
    ),
},3308 => SpriteInfo {
    pos: (
        1338,
        80,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        2.0,
        2.0,
    ),
},2471 => SpriteInfo {
    pos: (
        1865,
        0,
    ),
    size: (
        20,
        20,
    ),
    rotated: false,
    offset: (
        -22.0,
        -22.0,
    ),
},4190 => SpriteInfo {
    pos: (
        2433,
        8,
    ),
    size: (
        4,
        32,
    ),
    rotated: false,
    offset: (
        -2.0,
        16.0,
    ),
},2228 => SpriteInfo {
    pos: (
        3185,
        815,
    ),
    size: (
        64,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2678 => SpriteInfo {
    pos: (
        2600,
        1888,
    ),
    size: (
        56,
        52,
    ),
    rotated: false,
    offset: (
        2.0,
        0.0,
    ),
},2518 => SpriteInfo {
    pos: (
        2193,
        44,
    ),
    size: (
        48,
        92,
    ),
    rotated: false,
    offset: (
        20.0,
        2.0,
    ),
},2024 => SpriteInfo {
    pos: (
        628,
        1306,
    ),
    size: (
        311,
        453,
    ),
    rotated: false,
    offset: (
        -27.5,
        -93.5,
    ),
},4004 => SpriteInfo {
    pos: (
        2340,
        2470,
    ),
    size: (
        60,
        4,
    ),
    rotated: false,
    offset: (
        2.0,
        -26.0,
    ),
},2540 => SpriteInfo {
    pos: (
        84,
        1874,
    ),
    size: (
        64,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},4170 => SpriteInfo {
    pos: (
        124,
        438,
    ),
    size: (
        44,
        12,
    ),
    rotated: false,
    offset: (
        -8.0,
        0.0,
    ),
},2546 => SpriteInfo {
    pos: (
        1411,
        903,
    ),
    size: (
        60,
        56,
    ),
    rotated: false,
    offset: (
        -2.0,
        0.0,
    ),
},1587 => SpriteInfo {
    pos: (
        2649,
        189,
    ),
    size: (
        84,
        44,
    ),
    rotated: false,
    offset: (
        0.0,
        -18.0,
    ),
},2687 => SpriteInfo {
    pos: (
        1701,
        68,
    ),
    size: (
        128,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},2225 => SpriteInfo {
    pos: (
        529,
        1695,
    ),
    size: (
        64,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},2545 => SpriteInfo {
    pos: (
        2325,
        64,
    ),
    size: (
        60,
        56,
    ),
    rotated: false,
    offset: (
        -2.0,
        0.0,
    ),
},4164 => SpriteInfo {
    pos: (
        511,
        2539,
    ),
    size: (
        120,
        52,
    ),
    rotated: false,
    offset: (
        20.0,
        0.0,
    ),
},1866 => SpriteInfo {
    pos: (
        929,
        2071,
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
},2468 => SpriteInfo {
    pos: (
        1263,
        620,
    ),
    size: (
        60,
        60,
    ),
    rotated: false,
    offset: (
        2.0,
        -2.0,
    ),
},1054 => SpriteInfo {
    pos: (
        168,
        2196,
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
},1884 => SpriteInfo {
    pos: (
        3375,
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
},4177 => SpriteInfo {
    pos: (
        3253,
        815,
    ),
    size: (
        4,
        4,
    ),
    rotated: false,
    offset: (
        -8.0,
        0.0,
    ),
},2531 => SpriteInfo {
    pos: (
        68,
        935,
    ),
    size: (
        28,
        52,
    ),
    rotated: false,
    offset: (
        10.0,
        2.0,
    ),
},918 => SpriteInfo {
    pos: (
        627,
        1096,
    ),
    size: (
        44,
        43,
    ),
    rotated: false,
    offset: (
        -23.0,
        -77.5,
    ),
},3322 => SpriteInfo {
    pos: (
        627,
        818,
    ),
    size: (
        56,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},4002 => SpriteInfo {
    pos: (
        3701,
        2543,
    ),
    size: (
        60,
        44,
    ),
    rotated: false,
    offset: (
        -2.0,
        -6.0,
    ),
},1698 => SpriteInfo {
    pos: (
        2379,
        2126,
    ),
    size: (
        91,
        88,
    ),
    rotated: false,
    offset: (
        14.5,
        16.0,
    ),
},3424 => SpriteInfo {
    pos: (
        1729,
        1609,
    ),
    size: (
        4,
        44,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},4134 => SpriteInfo {
    pos: (
        172,
        276,
    ),
    size: (
        32,
        28,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},3534 => SpriteInfo {
    pos: (
        1867,
        880,
    ),
    size: (
        56,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2583 => SpriteInfo {
    pos: (
        2781,
        1027,
    ),
    size: (
        56,
        64,
    ),
    rotated: false,
    offset: (
        4.0,
        0.0,
    ),
},3796 => SpriteInfo {
    pos: (
        387,
        2342,
    ),
    size: (
        20,
        16,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},4189 => SpriteInfo {
    pos: (
        2721,
        617,
    ),
    size: (
        16,
        36,
    ),
    rotated: false,
    offset: (
        -8.0,
        -14.0,
    ),
},3503 => SpriteInfo {
    pos: (
        2843,
        799,
    ),
    size: (
        48,
        44,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},3295 => SpriteInfo {
    pos: (
        2427,
        964,
    ),
    size: (
        40,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        -4.0,
    ),
},4148 => SpriteInfo {
    pos: (
        2535,
        409,
    ),
    size: (
        72,
        28,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3332 => SpriteInfo {
    pos: (
        2158,
        2730,
    ),
    size: (
        56,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2475 => SpriteInfo {
    pos: (
        3811,
        256,
    ),
    size: (
        48,
        28,
    ),
    rotated: false,
    offset: (
        -8.0,
        -18.0,
    ),
},3402 => SpriteInfo {
    pos: (
        4080,
        2604,
    ),
    size: (
        12,
        20,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},3544 => SpriteInfo {
    pos: (
        305,
        1454,
    ),
    size: (
        80,
        80,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2565 => SpriteInfo {
    pos: (
        1874,
        1255,
    ),
    size: (
        120,
        60,
    ),
    rotated: false,
    offset: (
        4.0,
        2.0,
    ),
},3477 => SpriteInfo {
    pos: (
        184,
        1580,
    ),
    size: (
        60,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        -4.0,
    ),
},2227 => SpriteInfo {
    pos: (
        3150,
        1867,
    ),
    size: (
        64,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2508 => SpriteInfo {
    pos: (
        1611,
        295,
    ),
    size: (
        60,
        32,
    ),
    rotated: false,
    offset: (
        2.0,
        0.0,
    ),
},2373 => SpriteInfo {
    pos: (
        3894,
        1443,
    ),
    size: (
        56,
        60,
    ),
    rotated: false,
    offset: (
        2.0,
        0.0,
    ),
},3530 => SpriteInfo {
    pos: (
        1805,
        1913,
    ),
    size: (
        64,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        -8.0,
    ),
},4142 => SpriteInfo {
    pos: (
        192,
        8,
    ),
    size: (
        32,
        16,
    ),
    rotated: false,
    offset: (
        -12.0,
        0.0,
    ),
},2703 => SpriteInfo {
    pos: (
        3797,
        1020,
    ),
    size: (
        120,
        116,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},1867 => SpriteInfo {
    pos: (
        1081,
        1171,
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
},2688 => SpriteInfo {
    pos: (
        179,
        2619,
    ),
    size: (
        128,
        4,
    ),
    rotated: false,
    offset: (
        0.0,
        -24.0,
    ),
},3533 => SpriteInfo {
    pos: (
        3434,
        308,
    ),
    size: (
        56,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2353 => SpriteInfo {
    pos: (
        1449,
        2711,
    ),
    size: (
        60,
        36,
    ),
    rotated: false,
    offset: (
        0.0,
        -12.0,
    ),
},2507 => SpriteInfo {
    pos: (
        2532,
        2624,
    ),
    size: (
        60,
        56,
    ),
    rotated: false,
    offset: (
        2.0,
        0.0,
    ),
},3363 => SpriteInfo {
    pos: (
        2483,
        2332,
    ),
    size: (
        60,
        64,
    ),
    rotated: false,
    offset: (
        2.0,
        0.0,
    ),
},2226 => SpriteInfo {
    pos: (
        529,
        1631,
    ),
    size: (
        64,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},2478 => SpriteInfo {
    pos: (
        1395,
        2052,
    ),
    size: (
        56,
        48,
    ),
    rotated: false,
    offset: (
        4.0,
        -4.0,
    ),
},3137 => SpriteInfo {
    pos: (
        68,
        671,
    ),
    size: (
        36,
        28,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2676 => SpriteInfo {
    pos: (
        203,
        2382,
    ),
    size: (
        56,
        60,
    ),
    rotated: false,
    offset: (
        2.0,
        0.0,
    ),
},2588 => SpriteInfo {
    pos: (
        1398,
        0,
    ),
    size: (
        60,
        64,
    ),
    rotated: false,
    offset: (
        -2.0,
        0.0,
    ),
},4149 => SpriteInfo {
    pos: (
        3659,
        1531,
    ),
    size: (
        68,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3123 => SpriteInfo {
    pos: (
        360,
        2274,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},1582 => SpriteInfo {
    pos: (
        3146,
        1330,
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
},2422 => SpriteInfo {
    pos: (
        3221,
        604,
    ),
    size: (
        48,
        20,
    ),
    rotated: false,
    offset: (
        -2.0,
        0.0,
    ),
},2587 => SpriteInfo {
    pos: (
        3195,
        759,
    ),
    size: (
        64,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        -6.0,
    ),
},2032 => SpriteInfo {
    pos: (
        48,
        2063,
    ),
    size: (
        120,
        126,
    ),
    rotated: false,
    offset: (
        1.0,
        0.0,
    ),
},2360 => SpriteInfo {
    pos: (
        1151,
        1961,
    ),
    size: (
        56,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2673 => SpriteInfo {
    pos: (
        3028,
        2142,
    ),
    size: (
        60,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},3480 => SpriteInfo {
    pos: (
        1910,
        2744,
    ),
    size: (
        52,
        40,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},3368 => SpriteInfo {
    pos: (
        2243,
        810,
    ),
    size: (
        56,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},2605 => SpriteInfo {
    pos: (
        2348,
        1834,
    ),
    size: (
        56,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3525 => SpriteInfo {
    pos: (
        3486,
        2140,
    ),
    size: (
        64,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},2550 => SpriteInfo {
    pos: (
        1391,
        1866,
    ),
    size: (
        40,
        60,
    ),
    rotated: false,
    offset: (
        8.0,
        -2.0,
    ),
},3522 => SpriteInfo {
    pos: (
        2297,
        2291,
    ),
    size: (
        60,
        64,
    ),
    rotated: false,
    offset: (
        2.0,
        0.0,
    ),
},1864 => SpriteInfo {
    pos: (
        3529,
        2689,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3321 => SpriteInfo {
    pos: (
        2483,
        2400,
    ),
    size: (
        56,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},2027 => SpriteInfo {
    pos: (
        1211,
        2056,
    ),
    size: (
        180,
        410,
    ),
    rotated: false,
    offset: (
        4.0,
        -19.0,
    ),
},2944 => SpriteInfo {
    pos: (
        2291,
        2666,
    ),
    size: (
        61,
        120,
    ),
    rotated: false,
    offset: (
        29.5,
        0.0,
    ),
},2886 => SpriteInfo {
    pos: (
        2399,
        1429,
    ),
    size: (
        270,
        81,
    ),
    rotated: false,
    offset: (
        1.0,
        12.5,
    ),
},3529 => SpriteInfo {
    pos: (
        2662,
        2721,
    ),
    size: (
        64,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        -8.0,
    ),
},1601 => SpriteInfo {
    pos: (
        2919,
        799,
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
},3440 => SpriteInfo {
    pos: (
        0,
        224,
    ),
    size: (
        56,
        28,
    ),
    rotated: false,
    offset: (
        0.0,
        4.0,
    ),
},2417 => SpriteInfo {
    pos: (
        2261,
        20,
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
},2366 => SpriteInfo {
    pos: (
        2107,
        907,
    ),
    size: (
        64,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2516 => SpriteInfo {
    pos: (
        2000,
        291,
    ),
    size: (
        92,
        92,
    ),
    rotated: false,
    offset: (
        -2.0,
        2.0,
    ),
},2313 => SpriteInfo {
    pos: (
        943,
        1326,
    ),
    size: (
        8,
        16,
    ),
    rotated: false,
    offset: (
        2.0,
        2.0,
    ),
},2467 => SpriteInfo {
    pos: (
        2376,
        248,
    ),
    size: (
        44,
        64,
    ),
    rotated: false,
    offset: (
        2.0,
        0.0,
    ),
},3373 => SpriteInfo {
    pos: (
        3826,
        2002,
    ),
    size: (
        56,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        -4.0,
    ),
},2420 => SpriteInfo {
    pos: (
        3406,
        2069,
    ),
    size: (
        36,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},4187 => SpriteInfo {
    pos: (
        2200,
        1739,
    ),
    size: (
        16,
        64,
    ),
    rotated: false,
    offset: (
        -8.0,
        0.0,
    ),
},411 => SpriteInfo {
    pos: (
        4045,
        1018,
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
},2368 => SpriteInfo {
    pos: (
        247,
        2551,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3148 => SpriteInfo {
    pos: (
        0,
        128,
    ),
    size: (
        44,
        36,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3149 => SpriteInfo {
    pos: (
        1081,
        1443,
    ),
    size: (
        48,
        64,
    ),
    rotated: false,
    offset: (
        8.0,
        0.0,
    ),
},2691 => SpriteInfo {
    pos: (
        2501,
        2272,
    ),
    size: (
        128,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},3447 => SpriteInfo {
    pos: (
        3293,
        512,
    ),
    size: (
        20,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3425 => SpriteInfo {
    pos: (
        1530,
        1292,
    ),
    size: (
        32,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2949 => SpriteInfo {
    pos: (
        2914,
        560,
    ),
    size: (
        116,
        61,
    ),
    rotated: false,
    offset: (
        0.0,
        -29.5,
    ),
},1591 => SpriteInfo {
    pos: (
        387,
        2417,
    ),
    size: (
        120,
        20,
    ),
    rotated: false,
    offset: (
        0.0,
        12.0,
    ),
},3054 => SpriteInfo {
    pos: (
        2737,
        2260,
    ),
    size: (
        60,
        48,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3296 => SpriteInfo {
    pos: (
        2685,
        835,
    ),
    size: (
        24,
        16,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},2483 => SpriteInfo {
    pos: (
        124,
        1513,
    ),
    size: (
        56,
        36,
    ),
    rotated: false,
    offset: (
        0.0,
        6.0,
    ),
},2023 => SpriteInfo {
    pos: (
        2423,
        1113,
    ),
    size: (
        339,
        312,
    ),
    rotated: false,
    offset: (
        -9.5,
        -18.0,
    ),
},4064 => SpriteInfo {
    pos: (
        3962,
        762,
    ),
    size: (
        56,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2559 => SpriteInfo {
    pos: (
        862,
        178,
    ),
    size: (
        40,
        24,
    ),
    rotated: false,
    offset: (
        -12.0,
        20.0,
    ),
},2666 => SpriteInfo {
    pos: (
        247,
        944,
    ),
    size: (
        36,
        60,
    ),
    rotated: false,
    offset: (
        -12.0,
        0.0,
    ),
},2480 => SpriteInfo {
    pos: (
        4053,
        2556,
    ),
    size: (
        36,
        44,
    ),
    rotated: false,
    offset: (
        6.0,
        -6.0,
    ),
},2506 => SpriteInfo {
    pos: (
        1020,
        178,
    ),
    size: (
        64,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3450 => SpriteInfo {
    pos: (
        1183,
        2029,
    ),
    size: (
        24,
        28,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},2586 => SpriteInfo {
    pos: (
        247,
        752,
    ),
    size: (
        52,
        56,
    ),
    rotated: false,
    offset: (
        6.0,
        -4.0,
    ),
},2154 => SpriteInfo {
    pos: (
        2898,
        1372,
    ),
    size: (
        68,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        -4.0,
    ),
},2680 => SpriteInfo {
    pos: (
        1431,
        456,
    ),
    size: (
        52,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3546 => SpriteInfo {
    pos: (
        3149,
        2337,
    ),
    size: (
        80,
        24,
    ),
    rotated: false,
    offset: (
        0.0,
        -28.0,
    ),
},1702 => SpriteInfo {
    pos: (
        2007,
        0,
    ),
    size: (
        62,
        58,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3374 => SpriteInfo {
    pos: (
        2297,
        2235,
    ),
    size: (
        60,
        52,
    ),
    rotated: false,
    offset: (
        2.0,
        -2.0,
    ),
},2464 => SpriteInfo {
    pos: (
        3422,
        941,
    ),
    size: (
        60,
        56,
    ),
    rotated: false,
    offset: (
        -2.0,
        0.0,
    ),
},3524 => SpriteInfo {
    pos: (
        3761,
        1781,
    ),
    size: (
        64,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        -2.0,
    ),
},1735 => SpriteInfo {
    pos: (
        3886,
        2026,
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
},4356 => SpriteInfo {
    pos: (
        1068,
        798,
    ),
    size: (
        8,
        24,
    ),
    rotated: false,
    offset: (
        -2.0,
        2.0,
    ),
},1462 => SpriteInfo {
    pos: (
        2463,
        448,
    ),
    size: (
        120,
        103,
    ),
    rotated: false,
    offset: (
        0.0,
        8.5,
    ),
},2055 => SpriteInfo {
    pos: (
        3514,
        941,
    ),
    size: (
        279,
        228,
    ),
    rotated: false,
    offset: (
        -3.5,
        -16.0,
    ),
},3464 => SpriteInfo {
    pos: (
        595,
        1443,
    ),
    size: (
        24,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2669 => SpriteInfo {
    pos: (
        880,
        1899,
    ),
    size: (
        56,
        60,
    ),
    rotated: false,
    offset: (
        2.0,
        0.0,
    ),
},3439 => SpriteInfo {
    pos: (
        2869,
        2260,
    ),
    size: (
        56,
        28,
    ),
    rotated: false,
    offset: (
        0.0,
        4.0,
    ),
},211 => SpriteInfo {
    pos: (
        535,
        336,
    ),
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
        3831,
        2299,
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
},2411 => SpriteInfo {
    pos: (
        0,
        2166,
    ),
    size: (
        40,
        40,
    ),
    rotated: false,
    offset: (
        10.0,
        -10.0,
    ),
},1583 => SpriteInfo {
    pos: (
        239,
        2466,
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
},2418 => SpriteInfo {
    pos: (
        3149,
        2433,
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
},2502 => SpriteInfo {
    pos: (
        64,
        1626,
    ),
    size: (
        64,
        16,
    ),
    rotated: false,
    offset: (
        0.0,
        -20.0,
    ),
},3000 => SpriteInfo {
    pos: (
        0,
        1838,
    ),
    size: (
        120,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        4.0,
    ),
},2590 => SpriteInfo {
    pos: (
        3883,
        1507,
    ),
    size: (
        60,
        64,
    ),
    rotated: false,
    offset: (
        2.0,
        0.0,
    ),
},4171 => SpriteInfo {
    pos: (
        511,
        2481,
    ),
    size: (
        48,
        44,
    ),
    rotated: false,
    offset: (
        0.0,
        6.0,
    ),
},2026 => SpriteInfo {
    pos: (
        1587,
        2416,
    ),
    size: (
        195,
        272,
    ),
    rotated: false,
    offset: (
        12.5,
        -30.0,
    ),
},3144 => SpriteInfo {
    pos: (
        2356,
        2734,
    ),
    size: (
        64,
        64,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},2300 => SpriteInfo {
    pos: (
        1201,
        1519,
    ),
    size: (
        52,
        52,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},266 => SpriteInfo {
    pos: (
        2931,
        832,
    ),
    size: (
        120,
        120,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3443 => SpriteInfo {
    pos: (
        905,
        2143,
    ),
    size: (
        16,
        20,
    ),
    rotated: false,
    offset: (
        0.0,
        0.0,
    ),
},3002 => SpriteInfo {
    pos: (
        428,
        0,
    ),
    size: (
        120,
        32,
    ),
    rotated: false,
    offset: (
        0.0,
        4.0,
    ),
},2412 => SpriteInfo {
    pos: (
        0,
        1966,
    ),
    size: (
        60,
        36,
    ),
    rotated: false,
    offset: (
        0.0,
        -8.0,
    ),
},3125 => SpriteInfo {
    pos: (
        400,
        634,
    ),
    size: (
        56,
        60,
    ),
    rotated: false,
    offset: (
        0.0,
        2.0,
    ),
},3145 => SpriteInfo {
    pos: (
        1495,
        2651,
    ),
    size: (
        64,
        56,
    ),
    rotated: false,
    offset: (
        0.0,
        -4.0,
    ),
},
        _ => return None,
    })
}
    

    