use super::ObjectInfo;

use crate::objects::ObjectCategory;
use std::collections::HashMap;


pub fn get_available_objects() -> HashMap<u32, ObjectInfo> {
    [
        (1, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Blocks
        }),
        (83, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Blocks
        }),
        (2, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Blocks
        }),
        (3, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Blocks
        }),
        (4, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Blocks
        }),
        (5, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (6, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Blocks
        }),
        (7, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Blocks
        }),
    
        (467, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Outlines
        }),
        (468, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 14.25,
            tintable: true,
            solid: true,
            category: ObjectCategory::Outlines
        }),
        (469, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Outlines
        }),
        (470, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Outlines
        }),
        (471, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Outlines
        }),
        (472, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Outlines
        }),
        (473, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Outlines
        }),
        (474, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Outlines
        }),
        (1338, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Outlines
        }),
        (1339, ObjectInfo {
            place_offset_x: 15.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Outlines,
        }),
        (1210, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Outlines
        }),
        (1202, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 13.5,
            tintable: true,
            solid: true,
            category: ObjectCategory::Outlines
        }),
        (1203, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Outlines
        }),
        (1204, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Outlines
        }),
        (1209, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Outlines
        }),
        (1205, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Outlines
        }),
        (1206, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Outlines
        }),
        (1207, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Outlines
        }),
        (143, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Outlines
        }),
    
        (693, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Slopes
        }),
        (694, ObjectInfo {
            place_offset_x: 15.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Slopes,
        }),
        (695, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Slopes
        }),
        (696, ObjectInfo {
            place_offset_x: 15.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Slopes,
        }),
        (697, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Slopes
        }),
        (698, ObjectInfo {
            place_offset_x: 15.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Slopes,
        }),
        (699, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Slopes
        }),
        (700, ObjectInfo {
            place_offset_x: 15.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Slopes,
        }),
        (701, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Slopes
        }),
        (702, ObjectInfo {
            place_offset_x: 15.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Slopes,
        }),
        (877, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Slopes
        }),
        (878, ObjectInfo {
            place_offset_x: 15.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Slopes,
        }),
        (888, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Slopes
        }),
        (889, ObjectInfo {
            place_offset_x: 15.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Slopes,
        }),
        (895, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Slopes
        }),
        (896, ObjectInfo {
            place_offset_x: 15.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Slopes,
        }),
    
        (216, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Spikes,
        }),
        (217, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -9.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Spikes,
        }),
        (218, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -6.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Spikes,
        }),
        (458, ObjectInfo {
            place_offset_x: -7.5,
            place_offset_y: -9.75,
            tintable: true,
            solid: false,
            category: ObjectCategory::Spikes,
        }),
        (1889, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Spikes,
        }),
        (1890, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -9.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Spikes,
        }),
        (1891, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -6.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Spikes,
        }),
        (1892, ObjectInfo {
            place_offset_x: -7.5,
            place_offset_y: -9.75,
            tintable: true,
            solid: false,
            category: ObjectCategory::Spikes,
        }),
        (177, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Spikes,
        }),
        (178, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -8.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Spikes,
        }),
        (179, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -6.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Spikes,
        }),
        (1715, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -12.5,
            tintable: true,
            solid: false,
            category: ObjectCategory::Spikes,
        }),
        (1722, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -11.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Spikes,
        }),
        (1720, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -11.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Spikes,
        }),
        (1721, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -11.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Spikes,
        }),
        (135, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -11.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Spikes,
        }),
        (1717, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Spikes,
        }),
        (1718, ObjectInfo {
            place_offset_x: 15.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Spikes,
        }),
        (1723, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Spikes,
        }),
        (1724, ObjectInfo {
            place_offset_x: 15.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: true,
            category: ObjectCategory::Spikes,
        }),
        (1725, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -9.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Spikes,
        }),
        (1728, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -7.5,
            tintable: true,
            solid: false,
            category: ObjectCategory::Spikes,
        }),
        (1729, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -7.5,
            tintable: true,
            solid: false,
            category: ObjectCategory::Spikes,
        }),
        (1730, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -7.5,
            tintable: true,
            solid: false,
            category: ObjectCategory::Spikes,
        }),
        (1731, ObjectInfo {
            place_offset_x: -11.5,
            place_offset_y: -11.5,
            tintable: true,
            solid: false,
            category: ObjectCategory::Spikes,
        }),
    
        (211, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1825, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (259, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (266, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (273, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (658, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (722, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (659, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (734, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
    
        (476, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (477, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (478, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (479, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (480, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (481, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (482, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (641, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (642, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (739, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (643, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (644, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (645, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (646, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (647, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (648, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (649, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (650, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
    
        (869, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (870, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (871, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1266, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1267, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (873, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 7.5,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (874, ObjectInfo {
            place_offset_x: -7.5,
            place_offset_y: -7.5,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (880, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (881, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (882, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (883, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (890, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1247, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1279, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1280, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1281, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1277, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1278, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
    
        (927, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (928, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (929, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (930, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (931, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (932, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (933, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (934, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (935, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
    
        (35, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -13.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (140, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -13.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (1332, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -12.5,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (67, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -12.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (36, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (141, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (1333, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (84, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (1022, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (1330, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (1704, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (1751, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (10, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (11, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (12, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (13, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (47, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
    
        (111, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (660, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (745, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (1331, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (45, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (46, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (99, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (101, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (1755, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (1813, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (1829, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (1859, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: false,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (1586, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Utilities
        }),
        (1700, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Utilities
        }),
        (918, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (1584, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (919, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -10.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (1697, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Utilities
        }),
        (1698, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Utilities
        }),
        (1699, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Utilities
        }),
        (1053, ObjectInfo {
            place_offset_x: -7.5,
            place_offset_y: -7.5,
            tintable: true,
            solid: false,
            category: ObjectCategory::Utilities
        }),
        (1054, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -7.5,
            tintable: true,
            solid: false,
            category: ObjectCategory::Utilities
        }),
        (1583, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (1582, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Utilities,
        }),
        (1519, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Utilities
        }),
        (1618, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Utilities
        }),
    
        (503, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -5.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (505, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (504, ObjectInfo {
            place_offset_x: 5.0,
            place_offset_y: -5.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (1273, ObjectInfo {
            place_offset_x: 5.0,
            place_offset_y: -5.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (1274, ObjectInfo {
            place_offset_x: 5.0,
            place_offset_y: -5.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (1758, ObjectInfo {
            place_offset_x: -7.25,
            place_offset_y: 7.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (1759, ObjectInfo {
            place_offset_x: 10.5,
            place_offset_y: 9.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (1888, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
    
        (18, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 4.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (19, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 4.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (20, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -2.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (21, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -8.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (48, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 2.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (49, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -2.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (113, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 1.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (114, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -2.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (115, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -5.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (157, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -1.5,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (158, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -1.5,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (159, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -1.5,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (227, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -4.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (228, ObjectInfo {
            place_offset_x: -7.5,
            place_offset_y: -7.5,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (242, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (419, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -2.5,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (420, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -2.5,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (1050, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -2.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (1051, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -2.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
        (1052, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -2.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::GroundDeco
        }),
    
        (41, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 20.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (110, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 2.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (106, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 18.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (107, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 4.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1764, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1765, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1766, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1767, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1768, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
    
        (15, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 6.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Pulsing
        }),
        (16, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -1.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Pulsing
        }),
        (17, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -8.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Pulsing
        }),
        (132, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Pulsing
        }),
        (460, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Pulsing
        }),
        (494, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Pulsing
        }),
        (50, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Pulsing
        }),
        (51, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Pulsing
        }),
        (52, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Pulsing
        }),
        (53, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Pulsing
        }),
        (54, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Pulsing
        }),
        (60, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Pulsing
        }),
        (148, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Pulsing
        }),
        (149, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Pulsing
        }),
        (150, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Pulsing
        }),
        (133, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Pulsing
        }),
        (136, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Pulsing
        }),
    
        (1734, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws,
        }),
        (1735, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws,
        }),
        (1736, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws,
        }),
        (186, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws,
        }),
        (187, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws,
        }),
        (188, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws,
        }),
        (1705, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws,
        }),
        (1706, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws,
        }),
        (1707, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws,
        }),
        (1708, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws,
        }),
        (1709, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws,
        }),
        (1710, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws,
        }),
        (678, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws,
        }),
        (679, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws,
        }),
        (680, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws,
        }),
        (1619, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws,
        }),
        (1620, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws,
        }),
        (183, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws,
        }),
        (184, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws,
        }),
        (185, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws,
        }),
        (85, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (86, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (87, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (97, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (137, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (138, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (139, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (1019, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (1020, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (1021, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (394, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (395, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (396, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (154, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (155, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (156, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (222, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (223, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (224, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (1831, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (1832, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (1833, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (1834, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (1058, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (1059, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
        (1060, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Saws
        }),
    
        (719, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -7.5,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (721, ObjectInfo {
            place_offset_x: -11.5,
            place_offset_y: -11.5,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
    
        (409, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (410, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (411, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (412, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (413, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1756, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1001, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1002, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1003, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1004, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1005, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (916, ObjectInfo {
            place_offset_x: -7.5,
            place_offset_y: -7.5,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (917, ObjectInfo {
            place_offset_x: -11.25,
            place_offset_y: -11.25,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1740, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1741, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
    
        (937, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (938, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (414, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -9.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (406, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -8.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (408, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -12.5,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (907, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -4.5,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (908, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -7.5,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (909, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -7.5,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (939, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: -6.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1597, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1596, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1135, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1136, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1137, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1134, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1133, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1844, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1845, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1846, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1847, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1848, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1602, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1603, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1604, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1605, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (914, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1606, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1607, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1601, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1600, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1843, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
    
        (1837, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1835, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1753, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1754, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1757, ObjectInfo {
            place_offset_x: -7.5,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
        (1830, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Deco
        }),
    
        (1861, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
    
        (1862, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1863, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1864, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1865, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1866, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1867, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1868, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1869, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1870, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1871, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1872, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1873, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1874, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1875, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1876, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1877, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1878, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1879, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1880, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1881, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1882, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1883, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1884, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
        (1885, ObjectInfo {
            place_offset_x: 0.0,
            place_offset_y: 0.0,
            tintable: true,
            solid: false,
            category: ObjectCategory::Blocks
        }),
    ].into_iter().collect()
}
