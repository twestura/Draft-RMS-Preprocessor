//! Utility functions for writing code for actor areas.
//!
//! Throughout this module the TC squares are named with the following convention
//! - A is the left-center
//! - B is the top-center
//! - C is the bottom-center
//! - D is the right-center
//! By using `find_closest` with `min_distance_to_players` set to `1`, we
//! obtain a set R of two more squares:
//! - above and to the right of D
//! - below and to the right of D
//!
//! The boxes are then actor areas defining square boxes from the center
//! of the TC. `box0` is the 4 centermost tiles, `box1` is those tiles
//! and their adjacent neighbors, and so on.

/// Returns the lines used to define the placeholder constants.
///
/// Defines the following constants:
/// - `PHOFF 649`
/// - `PHON 1291`
/// - `TERRAIN_BLOCKER 1613`
/// - `SHEP0`, ..., `SHEP5` as `590` or `592`
///
/// Individual constants for `HERDABLE_A` and `STRAGGLER` must be defined in
/// each map script.
pub fn make_constants() -> Vec<String> {
    let mut lines = vec![
        String::from("#const PHOFF 649"),
        String::from("#const PHON 1291"),
        String::from("#const TERRAIN_BLOCKER 1613"),
        String::from("#const TEMPORARY_REVEALER 651"),
        String::from("#const TRIBUTE_INEFFICIENCY 46"),
    ];
    // 590 is VILLAGER_SHEPHERD_F; 592 is VILLAGER_SHEPHPERD_M
    for i in 0..6 {
        lines.push(String::from("start_random"));
        lines.push(format!("percent_chance 50 #const SHEP{i} 590"));
        lines.push(format!("percent_chance 50 #const SHEP{i} 592"));
        lines.push(String::from("end_random"));
    }
    // 123 is VILLAGER_WOOD_M; 218 is VILLAGER_WOOD_F
    for i in 0..3 {
        lines.push(String::from("start_random"));
        lines.push(format!("percent_chance 50 #const LUMBERJACK{i} 123"));
        lines.push(format!("percent_chance 50 #const LUMBERJACK{i} 218"));
        lines.push(String::from("end_random"));
    }
    lines
}

/// Returns the lines for clearing the placeholder attributes in `<PLAYER_SETUP>`.
/// Also sets the Gaia HP for `HERDABLE_A` to `0`.
pub fn set_placeholder_attributes() -> Vec<String> {
    vec![
        String::from("effect_amount SET_ATTRIBUTE PHOFF ATTR_DEAD_ID -1"),
        String::from("effect_amount SET_ATTRIBUTE PHOFF ATTR_TERRAIN_ID 0"),
        String::from("effect_amount SET_ATTRIBUTE PHOFF ATTR_LINE_OF_SIGHT 0"),
        String::from("effect_amount SET_ATTRIBUTE PHOFF ATTR_HITPOINTS 0"),
        String::from("effect_amount SET_ATTRIBUTE PHOFF ATTR_STORAGE_VALUE 0"),
        String::from("effect_amount SET_ATTRIBUTE PHON ATTR_DEAD_ID -1"),
        String::from("effect_amount SET_ATTRIBUTE PHON ATTR_TERRAIN_ID 0"),
        String::from("effect_amount SET_ATTRIBUTE PHON ATTR_LINE_OF_SIGHT 0"),
        String::from("effect_amount SET_ATTRIBUTE PHON ATTR_HITPOINTS 0"),
        String::from("effect_amount SET_ATTRIBUTE PHON ATTR_STORAGE_VALUE 0"),
        String::from("effect_amount GAIA_SET_ATTRIBUTE HERDABLE_A ATTR_HITPOINTS 0"),
        String::from("effect_amount SET_ATTRIBUTE TEMPORARY_REVEALER ATTR_DEAD_ID -1"),
        String::from("effect_amount SET_ATTRIBUTE TEMPORARY_REVEALER ATTR_HITPOINTS 0"),
        String::from("effect_amount SET_ATTRIBUTE TEMPORARY_REVEALER ATTR_LINE_OF_SIGHT 18"),
        String::from("effect_amount MOD_RESOURCE AMOUNT_STARTING_WOOD ATTR_ADD -30"),
        String::from("effect_amount MOD_RESOURCE AMOUNT_STARTING_FOOD ATTR_ADD -100"),
        String::from("effect_percent MOD_RESOURCE TRIBUTE_INEFFICIENCY ATTR_SET 50"),
    ]
}

/// Sets placeholder attributes for Four Seasons.
/// Sets the Gaia HP for all biome `BIOME_HERDABLE_A` to `0`.
pub fn set_placeholder_attributes_four_seasons() -> Vec<String> {
    vec![
        String::from("effect_amount SET_ATTRIBUTE PHOFF ATTR_DEAD_ID -1"),
        String::from("effect_amount SET_ATTRIBUTE PHOFF ATTR_TERRAIN_ID 0"),
        String::from("effect_amount SET_ATTRIBUTE PHOFF ATTR_LINE_OF_SIGHT 0"),
        String::from("effect_amount SET_ATTRIBUTE PHOFF ATTR_HITPOINTS 0"),
        String::from("effect_amount SET_ATTRIBUTE PHOFF ATTR_STORAGE_VALUE 0"),
        String::from("effect_amount SET_ATTRIBUTE PHON ATTR_DEAD_ID -1"),
        String::from("effect_amount SET_ATTRIBUTE PHON ATTR_TERRAIN_ID 0"),
        String::from("effect_amount SET_ATTRIBUTE PHON ATTR_LINE_OF_SIGHT 0"),
        String::from("effect_amount SET_ATTRIBUTE PHON ATTR_HITPOINTS 0"),
        String::from("effect_amount SET_ATTRIBUTE PHON ATTR_STORAGE_VALUE 0"),
        String::from("effect_amount GAIA_SET_ATTRIBUTE GRASS_HERDABLE_A ATTR_HITPOINTS 0"),
        String::from("effect_amount GAIA_SET_ATTRIBUTE SNOW_HERDABLE_A ATTR_HITPOINTS 0"),
        String::from("effect_amount GAIA_SET_ATTRIBUTE DIRT_HERDABLE_A ATTR_HITPOINTS 0"),
        String::from("effect_amount GAIA_SET_ATTRIBUTE JUNGLE_HERDABLE_A ATTR_HITPOINTS 0"),
        String::from("effect_amount SET_ATTRIBUTE TEMPORARY_REVEALER ATTR_DEAD_ID -1"),
        String::from("effect_amount SET_ATTRIBUTE TEMPORARY_REVEALER ATTR_HITPOINTS 0"),
        String::from("effect_amount SET_ATTRIBUTE TEMPORARY_REVEALER ATTR_LINE_OF_SIGHT 18"),
        String::from("effect_amount MOD_RESOURCE AMOUNT_STARTING_WOOD ATTR_ADD -30"),
        String::from("effect_amount MOD_RESOURCE AMOUNT_STARTING_FOOD ATTR_ADD -100"),
        String::from("effect_percent MOD_RESOURCE TRIBUTE_INEFFICIENCY ATTR_SET 50"),
    ]
}

/// Same as `set_zewall_placeholder_attributes` but uses `SET_ATTRIBUTE`
/// instead of `GAIA_SET_ATTRIBUTE` for the initial dying herdable.
// Note the Goose still lives, so this function isn't useful.
// pub fn set_zewall_placeholder_attributes() -> Vec<String> {
//     vec![
//         String::from("effect_amount SET_ATTRIBUTE PHOFF ATTR_DEAD_ID -1"),
//         String::from("effect_amount SET_ATTRIBUTE PHOFF ATTR_TERRAIN_ID 0"),
//         String::from("effect_amount SET_ATTRIBUTE PHOFF ATTR_LINE_OF_SIGHT 0"),
//         String::from("effect_amount SET_ATTRIBUTE PHOFF ATTR_HITPOINTS 0"),
//         String::from("effect_amount SET_ATTRIBUTE PHOFF ATTR_STORAGE_VALUE 0"),
//         String::from("effect_amount SET_ATTRIBUTE PHON ATTR_DEAD_ID -1"),
//         String::from("effect_amount SET_ATTRIBUTE PHON ATTR_TERRAIN_ID 0"),
//         String::from("effect_amount SET_ATTRIBUTE PHON ATTR_LINE_OF_SIGHT 0"),
//         String::from("effect_amount SET_ATTRIBUTE PHON ATTR_HITPOINTS 0"),
//         String::from("effect_amount SET_ATTRIBUTE PHON ATTR_STORAGE_VALUE 0"),
//         String::from("effect_amount SET_ATTRIBUTE HERDABLE_A ATTR_HITPOINTS 0"),
//         String::from("effect_amount SET_ATTRIBUTE TEMPORARY_REVEALER ATTR_DEAD_ID -1"),
//         String::from("effect_amount SET_ATTRIBUTE TEMPORARY_REVEALER ATTR_HITPOINTS 0"),
//         String::from("effect_amount SET_ATTRIBUTE TEMPORARY_REVEALER ATTR_LINE_OF_SIGHT 18"),
//         String::from("effect_amount MOD_RESOURCE AMOUNT_STARTING_WOOD ATTR_ADD -30"),
//         String::from("effect_amount MOD_RESOURCE AMOUNT_STARTING_FOOD ATTR_ADD -100"),
//         String::from("effect_percent MOD_RESOURCE TRIBUTE_INEFFICIENCY ATTR_SET 50"),
//     ]
// }

/// Returns a String of lines for setting up the placeholdres
/// `tc_a`, `tc_b`, `tc_c`, `tc_d`, `tc_r0`, `rc_r1`.
///
/// The map requires that `PHON` is setup with the invisible object as the
/// on-grid placeholder.
///
/// May be used for multiple TCs.
pub fn tc_center() -> Vec<String> {
    String::from(
        "create_object PHON {
set_place_for_every_player
set_gaia_object_only
max_distance_to_players 0
actor_area tc_d
actor_area_radius 0
}
create_object PHON {
number_of_objects 2
set_place_for_every_player
set_gaia_object_only
find_closest
min_distance_to_players 1
max_distance_to_players 1
actor_area tc_r0
actor_area_radius 0
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
find_closest
min_distance_to_players 1
max_distance_to_players 1
avoid_actor_area tc_r0
actor_area tc_c
actor_area_radius 0
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
find_closest
min_distance_to_players 1
max_distance_to_players 1
avoid_actor_area tc_r0
avoid_actor_area tc_c
actor_area tc_b
actor_area_radius 0
}
create_object PHON {
number_of_objects 2
set_place_for_every_player
set_gaia_object_only
find_closest
min_distance_to_players 1
max_distance_to_players 1
actor_area tc_r1
actor_area_radius 1
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
max_distance_to_players 1
avoid_actor_area tc_r1
actor_area tc_a
actor_area_radius 0
}",
    )
    .split("\n")
    .map(String::from)
    .collect::<Vec<String>>()
}

/// Returns a vector of lines for placing actor area boxes around player TCs.
pub fn tc_boxes() -> Vec<String> {
    let mut lines = vec![];
    for tile in ["a", "b", "c", "d"] {
        lines.push(String::from("create_object PHON {"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("set_gaia_object_only"));
        lines.push(format!("actor_area_to_place_in tc_{tile}"));
        lines.push(String::from("actor_area box0"));
        lines.push(String::from("actor_area_radius 0"));
        lines.push(String::from("}"));
    }
    for i in 1..64 {
        lines.push(String::from("create_object PHON {"));
        lines.push(String::from("number_of_objects 4"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("set_gaia_object_only"));
        lines.push(String::from("actor_area_to_place_in box0"));
        lines.push(format!("actor_area box{i}"));
        lines.push(format!("actor_area_radius {i}"));
        lines.push(String::from("}"));
    }
    lines
}

/// Returns a vector of lines for generating boxes for multiple TCs.
pub fn tc_multiboxes() -> Vec<String> {
    let mut lines = vec![];
    for tile in ["a", "b", "c", "d"] {
        lines.push(String::from("create_object PHON {"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("set_gaia_object_only"));
        lines.push(format!("actor_area_to_place_in tc_{tile}"));
        lines.push(String::from("avoid_actor_area box0"));
        lines.push(String::from("actor_area box0"));
        lines.push(String::from("actor_area_radius 0"));
        lines.push(String::from("}"));
    }
    for i in 1..64 {
        lines.push(String::from("create_object PHON {"));
        lines.push(String::from("number_of_objects 8"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("set_gaia_object_only"));
        lines.push(String::from("actor_area_to_place_in box0"));
        lines.push(format!("actor_area box{i}"));
        lines.push(format!("actor_area_radius {i}"));
        lines.push(String::from("}"));
    }
    lines
}

/// Places 9 Villagers under the TC.
///
/// Requries that the constants `SHEP0`, ..., `SHEP5` are set (randomly) as
/// male or female Shepherds.
pub fn vils_9_tc() -> Vec<String> {
    let mut lines = vec![];
    lines.append(
        &mut String::from(
            " create_object PHON {
set_place_for_every_player
set_gaia_object_only
number_of_objects 5
min_distance_to_players 1
max_distance_to_players 1
find_closest
actor_area near_positioner
actor_area_radius 0
}
create_object PHOFF {
set_place_for_every_player
set_gaia_object_only
min_distance_to_players 1
max_distance_to_players 1
find_closest
second_object HERDABLE_A
actor_area herd0
actor_area_radius 0
avoid_actor_area near_positioner
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in herd0
actor_area herd1
actor_area_radius 1
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
max_distance_to_players 0
actor_area tc_d1
actor_area_radius 1
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
number_of_objects 5
max_distance_to_players 2
temp_min_distance_group_placement 1
find_closest
avoid_actor_area tc_d1
actor_area far_positioner
actor_area_radius 1
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
max_distance_to_players 2
find_closest
avoid_actor_area tc_d1
avoid_actor_area far_positioner
actor_area blocking_seventh_villager
actor_area_radius 0
}",
        )
        .split("\n")
        .map(String::from)
        .collect::<Vec<String>>(),
    );
    for i in 0..6 {
        lines.push(String::from("create_object PHOFF {"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("actor_area_to_place_in herd1"));
        lines.push(String::from("avoid_actor_area tc_d"));
        lines.push(String::from("avoid_actor_area villager0"));
        lines.push(String::from("avoid_actor_area herd0"));
        lines.push(String::from("avoid_actor_area blocking_seventh_villager"));
        lines.push(String::from("actor_area villager0"));
        lines.push(String::from("actor_area_radius 0"));
        lines.push(format!("second_object SHEP{i}"));
        lines.push(String::from("}"));
    }
    lines
}

/// ZeWall version of `vils9tc` that does not make the herdable gaia.
pub fn vils_9_tc_ze_wall() -> Vec<String> {
    let mut lines = vec![];
    lines.append(
        &mut String::from(
            " create_object PHON {
set_place_for_every_player
set_gaia_object_only
number_of_objects 5
min_distance_to_players 1
max_distance_to_players 1
find_closest
actor_area near_positioner
actor_area_radius 0
}
create_object PHOFF {
set_place_for_every_player
set_gaia_object_only
min_distance_to_players 1
max_distance_to_players 1
find_closest
second_object HERDABLE_A
actor_area herd0
actor_area_radius 0
avoid_actor_area near_positioner
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in herd0
actor_area herd1
actor_area_radius 1
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
max_distance_to_players 0
actor_area tc_d1
actor_area_radius 1
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
number_of_objects 5
max_distance_to_players 2
temp_min_distance_group_placement 1
find_closest
avoid_actor_area tc_d1
actor_area far_positioner
actor_area_radius 1
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
max_distance_to_players 2
find_closest
avoid_actor_area tc_d1
avoid_actor_area far_positioner
actor_area blocking_seventh_villager
actor_area_radius 0
}",
        )
        .split("\n")
        .map(String::from)
        .collect::<Vec<String>>(),
    );
    for i in 0..6 {
        lines.push(String::from("create_object PHOFF {"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("actor_area_to_place_in herd1"));
        lines.push(String::from("avoid_actor_area tc_d"));
        lines.push(String::from("avoid_actor_area villager0"));
        lines.push(String::from("avoid_actor_area herd0"));
        lines.push(String::from("avoid_actor_area blocking_seventh_villager"));
        lines.push(String::from("actor_area villager0"));
        lines.push(String::from("actor_area_radius 0"));
        lines.push(format!("second_object SHEP{i}"));
        lines.push(String::from("}"));
    }
    lines
}

/// Returns the lines for 9 Villagers with a 2-TC start.
pub fn multi_vils_9_tc() -> Vec<String> {
    let mut lines = vec![
        String::from("create_object PHON {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("number_of_objects 5"),
        String::from("min_distance_to_players 1"),
        String::from("max_distance_to_players 1"),
        String::from("find_closest"),
        String::from("actor_area near_positioner"),
        String::from("actor_area_radius 0"),
        String::from("}"),
        String::from("create_object PHOFF {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("min_distance_to_players 1"),
        String::from("max_distance_to_players 1"),
        String::from("find_closest"),
        String::from("second_object GRASS_HERDABLE_A"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("actor_area herd0"),
        String::from("actor_area_radius 0"),
        String::from("avoid_actor_area near_positioner"),
        String::from("}"),
        String::from("create_object PHOFF {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("min_distance_to_players 1"),
        String::from("max_distance_to_players 1"),
        String::from("find_closest"),
        String::from("second_object SNOW_HERDABLE_A"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("actor_area herd0"),
        String::from("actor_area_radius 0"),
        String::from("avoid_actor_area near_positioner"),
        String::from("}"),
        String::from("create_object PHOFF {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("min_distance_to_players 1"),
        String::from("max_distance_to_players 1"),
        String::from("find_closest"),
        String::from("second_object DIRT_HERDABLE_A"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("actor_area herd0"),
        String::from("actor_area_radius 0"),
        String::from("avoid_actor_area near_positioner"),
        String::from("}"),
        String::from("create_object PHOFF {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("min_distance_to_players 1"),
        String::from("max_distance_to_players 1"),
        String::from("find_closest"),
        String::from("second_object JUNGLE_HERDABLE_A"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("actor_area herd0"),
        String::from("actor_area_radius 0"),
        String::from("avoid_actor_area near_positioner"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 2"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in herd0"),
        String::from("actor_area herd1"),
        String::from("actor_area_radius 1"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("max_distance_to_players 0"),
        String::from("actor_area tc_d1"),
        String::from("actor_area_radius 1"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("number_of_objects 5"),
        String::from("max_distance_to_players 2"),
        String::from("temp_min_distance_group_placement 1"),
        String::from("find_closest"),
        String::from("avoid_actor_area tc_d1"),
        String::from("actor_area far_positioner"),
        String::from("actor_area_radius 1"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("max_distance_to_players 2"),
        String::from("find_closest"),
        String::from("avoid_actor_area tc_d1"),
        String::from("avoid_actor_area far_positioner"),
        String::from("actor_area blocking_seventh_villager"),
        String::from("actor_area_radius 0"),
        String::from("}"),
    ];
    // Each Shepard is duplicated at each TC.
    // But in total 12 Villagers are placed, 6 at each TC.
    for i in 0..6 {
        lines.push(String::from("create_object PHOFF {"));
        // lines.push(String::from("generate_for_first_land_only"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("actor_area_to_place_in herd1"));
        lines.push(String::from("avoid_actor_area tc_d"));
        lines.push(String::from("avoid_actor_area villager0"));
        lines.push(String::from("avoid_actor_area herd0"));
        lines.push(String::from("avoid_actor_area blocking_seventh_villager"));
        lines.push(String::from("actor_area villager0"));
        lines.push(String::from("actor_area_radius 0"));
        lines.push(String::from("max_distance_to_players 4"));
        lines.push(format!("second_object SHEP{i}"));
        lines.push(String::from("}"));
    }
    // for i in 6..12 {
    //     lines.push(String::from("create_object PHOFF {"));
    //     lines.push(String::from("avoid_actor_area first_land_20"));
    //     lines.push(String::from("set_place_for_every_player"));
    //     lines.push(String::from("actor_area_to_place_in herd1"));
    //     lines.push(String::from("avoid_actor_area tc_d"));
    //     lines.push(String::from("avoid_actor_area villager0"));
    //     lines.push(String::from("avoid_actor_area herd0"));
    //     lines.push(String::from("avoid_actor_area blocking_seventh_villager"));
    //     lines.push(String::from("actor_area villager0"));
    //     lines.push(String::from("actor_area_radius 0"));
    //     lines.push(String::from("max_distance_to_players 4"));
    //     lines.push(format!("second_object SHEP{i}"));
    //     lines.push(String::from("}"));
    // }
    lines
}

/// Returns a list of actor areas for placing Houses.
///
/// The Houses spawn with a 3-tile gap between them and the TC.
/// A sequence of actor areas is needed in order to maintain the same gap
/// on both the left and right sides of the TC. The right corner (D) of the
/// House is the location where it is placed.
///
/// Actor areas `house0`, `house1`, ... `house9` are returned, giving boxes
/// around the Houses of equal distance. Again, these boxes are squares
/// around the houses, with the center `house0` including the 2x2 tiles
/// covering the House.
///
/// Places a terrain blocker 1 tile around the House.
pub fn house_gap_3() -> Vec<String> {
    let mut lines = String::from(
        "create_object PHON {
number_of_objects 2
set_gaia_object_only
set_place_for_every_player
find_closest
min_distance_to_players 5
max_distance_to_players 5
actor_area house_avoid_box5
actor_area_radius 5
}
create_object PHON {
number_of_objects 99
set_gaia_object_only
set_place_for_every_player
actor_area_to_place_in box5
avoid_actor_area house_avoid_box5
avoid_actor_area box4
actor_area house_placement
actor_area_radius 0
}
create_object PHON {
number_of_objects 2
set_gaia_object_only
set_place_for_every_player
find_closest
min_distance_to_players 5
max_distance_to_players 5
actor_area house_placement_box6
actor_area_radius 6
}
create_object PHON {
number_of_objects 99
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in house_placement_box6
avoid_actor_area box5
max_distance_to_players 7
actor_area house_placement
actor_area_radius 0
}",
    )
    .split("\n")
    .map(String::from)
    .collect::<Vec<String>>();
    lines.append(
        &mut String::from(
            "create_object HOUSE {
number_of_objects 2
temp_min_distance_group_placement 7
set_place_for_every_player
avoid_forest_zone 2
actor_area_to_place_in house_placement
actor_area house_right_0
actor_area_radius 0
}
create_object PHON {
number_of_objects 2
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in house_right_0
actor_area house_right_1
actor_area_radius 1
}
create_object PHON {
number_of_objects 18
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in house_right_1
actor_area house1_cover
actor_area_radius 0
}
create_object TERRAIN_BLOCKER {
number_of_objects 10
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in house1_cover
actor_area outside_house1
actor_area_radius 0
}
create_object PHON {
number_of_objects 8
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in house1_cover
avoid_actor_area outside_house1
actor_area house0
actor_area_radius 0
}
create_object PHON {
number_of_objects 8
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in house0
actor_area house1
actor_area_radius 1
}
create_object PHON {
number_of_objects 8
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in house0
actor_area house2
actor_area_radius 2
}
create_object PHON {
number_of_objects 8
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in house0
actor_area house3
actor_area_radius 3
}
create_object PHON {
number_of_objects 8
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in house0
actor_area house4
actor_area_radius 4
}
create_object TERRAIN_BLOCKER {
number_of_objects 14
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in house1
}",
        )
        .split("\n")
        .map(String::from)
        .collect::<Vec<String>>(),
    );
    for i in 2..10 {
        lines.push(String::from("create_object PHON {"));
        lines.push(String::from("number_of_objects 8"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("set_gaia_object_only"));
        lines.push(String::from("actor_area_to_place_in house0"));
        lines.push(format!("actor_area house{i}"));
        lines.push(format!("actor_area_radius {i}"));
        lines.push(String::from("}"));
    }
    lines
}

/// Returns the lines for Houses for a 2-TC start.
pub fn multi_houses() -> Vec<String> {
    vec![
        String::from("create_object PHON {"),
        String::from("number_of_objects 2"),
        String::from("set_gaia_object_only"),
        String::from("set_place_for_every_player"),
        String::from("find_closest"),
        String::from("min_distance_to_players 5"),
        String::from("max_distance_to_players 5"),
        String::from("actor_area house_avoid_box5"),
        String::from("actor_area_radius 5"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 99"),
        String::from("set_gaia_object_only"),
        String::from("set_place_for_every_player"),
        String::from("actor_area_to_place_in box5"),
        String::from("avoid_actor_area house_avoid_box5"),
        String::from("avoid_actor_area box4"),
        String::from("actor_area house_placement"),
        String::from("actor_area_radius 0"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 2"),
        String::from("set_gaia_object_only"),
        String::from("set_place_for_every_player"),
        String::from("find_closest"),
        String::from("min_distance_to_players 5"),
        String::from("max_distance_to_players 5"),
        String::from("actor_area house_placement_box6"),
        String::from("actor_area_radius 6"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 99"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in house_placement_box6"),
        String::from("avoid_actor_area box5"),
        String::from("max_distance_to_players 7"),
        String::from("actor_area house_placement"),
        String::from("actor_area_radius 0"),
        String::from("}"),
        String::from("create_object HOUSE {"),
        String::from("number_of_objects 2"),
        String::from("temp_min_distance_group_placement 7"),
        String::from("set_place_for_every_player"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in house_placement"),
        String::from("actor_area house_right_0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 2"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in house_right_0"),
        String::from("actor_area house_right_1"),
        String::from("actor_area_radius 1"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 18"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in house_right_1"),
        String::from("actor_area house1_cover"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object TERRAIN_BLOCKER {"),
        String::from("number_of_objects 10"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in house1_cover"),
        String::from("actor_area outside_house1"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 8"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in house1_cover"),
        String::from("avoid_actor_area outside_house1"),
        String::from("actor_area house0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 8"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in house0"),
        String::from("actor_area house1"),
        String::from("actor_area_radius 1"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 8"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in house0"),
        String::from("actor_area house2"),
        String::from("actor_area_radius 2"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 8"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in house0"),
        String::from("actor_area house3"),
        String::from("actor_area_radius 3"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 8"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in house0"),
        String::from("actor_area house4"),
        String::from("actor_area_radius 4"),
        String::from("max_distance_to_players 10"),
        String::from("} "),
        String::from("create_object TERRAIN_BLOCKER {"),
        String::from("number_of_objects 14"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in house1"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
    ]
}

/// The same as `house_gap_3`, but uses Huts instead of Houses.
pub fn hut_gap_3() -> Vec<String> {
    house_gap_3()
        .iter()
        .map(|s| s.replace("HOUSE", "HUT"))
        .collect()
}

/// Places straggler trees, with one surrounded by Villagers.
/// Places 2 straggler trees 2 tiles from the TC.
/// Places 3 straggler trees 3 tiles from the TC.
/// Places 3 Lumberjacks around one of the 3-tile stragglers.
pub fn vils_9_straggler() -> Vec<String> {
    let mut lines = String::from(
        "create_object STRAGGLER {
set_place_for_every_player
set_gaia_object_only
avoid_forest_zone 2
actor_area_to_place_in box5
avoid_actor_area box4
avoid_actor_area house2
actor_area villager_tree0
actor_area_radius 0
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in villager_tree0
actor_area villager_tree1
actor_area_radius 1
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in villager_tree0
actor_area straggler2
actor_area_radius 2
}
create_object STRAGGLER {
number_of_objects 2
set_place_for_every_player
set_gaia_object_only
temp_min_distance_group_placement 2
avoid_forest_zone 2
actor_area_to_place_in box4
avoid_actor_area box3
avoid_actor_area house1
avoid_actor_area straggler2
actor_area straggler2
actor_area_radius 2
}
create_object STRAGGLER {
number_of_objects 2
set_place_for_every_player
set_gaia_object_only
temp_min_distance_group_placement 3
avoid_forest_zone 2
actor_area_to_place_in box5
avoid_actor_area box4
avoid_actor_area house1
avoid_actor_area straggler2
actor_area straggler2
actor_area_radius 2
}",
    )
    .split("\n")
    .map(String::from)
    .collect::<Vec<String>>();
    for i in 0..3 {
        lines.push(format!("create_object LUMBERJACK{i} {{"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("actor_area_to_place_in villager_tree1"));
        lines.push(String::from("actor_area villager0"));
        lines.push(String::from("actor_area_radius 0"));
        lines.push(String::from("}"));
    }
    lines
}

/// Places straggler trees, with one surrounded by Villagers.
/// Keeps the one surrounded by Villagers as close to the map
/// edge as possible.
/// Places 2 straggler trees 2 tiles from the TC.
/// Places 3 straggler trees 3 tiles from the TC.
/// Places 3 Lumberjacks around one of the 3-tile stragglers.
pub fn vils_9_straggler_socotra() -> Vec<String> {
    let mut lines = String::from(
        "create_object STRAGGLER {
set_place_for_every_player
set_gaia_object_only
find_closest_to_map_edge
avoid_forest_zone 2
actor_area_to_place_in box5
avoid_actor_area box4
avoid_actor_area house2
actor_area villager_tree0
actor_area_radius 0
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in villager_tree0
actor_area villager_tree1
actor_area_radius 1
}
create_object PHON {
set_place_for_every_player
set_gaia_object_only
actor_area_to_place_in villager_tree0
actor_area straggler2
actor_area_radius 2
}
create_object STRAGGLER {
number_of_objects 2
set_place_for_every_player
set_gaia_object_only
temp_min_distance_group_placement 2
avoid_forest_zone 2
actor_area_to_place_in box4
avoid_actor_area box3
avoid_actor_area house1
avoid_actor_area straggler2
actor_area straggler2
actor_area_radius 2
}
create_object STRAGGLER {
number_of_objects 2
set_place_for_every_player
set_gaia_object_only
temp_min_distance_group_placement 3
avoid_forest_zone 2
actor_area_to_place_in box5
avoid_actor_area box4
avoid_actor_area house1
avoid_actor_area straggler2
actor_area straggler2
actor_area_radius 2
}",
    )
    .split("\n")
    .map(String::from)
    .collect::<Vec<String>>();
    for i in 0..3 {
        lines.push(format!("create_object LUMBERJACK{i} {{"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("actor_area_to_place_in villager_tree1"));
        lines.push(String::from("actor_area villager0"));
        lines.push(String::from("actor_area_radius 0"));
        lines.push(String::from("}"));
    }
    lines
}

/// Returns the lines for straggler for a 2-TC start.
pub fn multi_stragglers() -> Vec<String> {
    let mut lines = vec![
        String::from("create_object GRASS_STRAGGLER {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box5"),
        String::from("avoid_actor_area box4"),
        String::from("avoid_actor_area house2"),
        String::from("actor_area villager_tree0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("}"),
        String::from("create_object SNOW_STRAGGLER {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box5"),
        String::from("avoid_actor_area box4"),
        String::from("avoid_actor_area house2"),
        String::from("actor_area villager_tree0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("}"),
        String::from("create_object DIRT_STRAGGLER {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box5"),
        String::from("avoid_actor_area box4"),
        String::from("avoid_actor_area house2"),
        String::from("actor_area villager_tree0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("}"),
        String::from("create_object JUNGLE_STRAGGLER {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box5"),
        String::from("avoid_actor_area box4"),
        String::from("avoid_actor_area house2"),
        String::from("actor_area villager_tree0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in villager_tree0"),
        String::from("actor_area villager_tree1"),
        String::from("actor_area_radius 1"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in villager_tree0"),
        String::from("actor_area straggler2"),
        String::from("actor_area_radius 2"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object GRASS_STRAGGLER {"),
        String::from("number_of_objects 2"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("temp_min_distance_group_placement 2"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box4"),
        String::from("avoid_actor_area box3"),
        String::from("avoid_actor_area house1"),
        String::from("avoid_actor_area straggler0"),
        String::from("actor_area straggler0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("}"),
        String::from("create_object SNOW_STRAGGLER {"),
        String::from("number_of_objects 2"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("temp_min_distance_group_placement 2"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box4"),
        String::from("avoid_actor_area box3"),
        String::from("avoid_actor_area house1"),
        String::from("avoid_actor_area straggler0"),
        String::from("actor_area straggler0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("}"),
        String::from("create_object DIRT_STRAGGLER {"),
        String::from("number_of_objects 2"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("temp_min_distance_group_placement 2"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box4"),
        String::from("avoid_actor_area box3"),
        String::from("avoid_actor_area house1"),
        String::from("avoid_actor_area straggler0"),
        String::from("actor_area straggler0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("}"),
        String::from("create_object JUNGLE_STRAGGLER {"),
        String::from("number_of_objects 2"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("temp_min_distance_group_placement 2"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box4"),
        String::from("avoid_actor_area box3"),
        String::from("avoid_actor_area house1"),
        String::from("avoid_actor_area straggler0"),
        String::from("actor_area straggler0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 2"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in straggler0"),
        String::from("actor_area straggler2"),
        String::from("actor_area_radius 2"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object GRASS_STRAGGLER {"),
        String::from("number_of_objects 2"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("temp_min_distance_group_placement 2"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box5"),
        String::from("avoid_actor_area box4"),
        String::from("avoid_actor_area house1"),
        String::from("avoid_actor_area straggler0"),
        String::from("actor_area straggler0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("}"),
        String::from("create_object SNOW_STRAGGLER {"),
        String::from("number_of_objects 2"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("temp_min_distance_group_placement 2"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box5"),
        String::from("avoid_actor_area box4"),
        String::from("avoid_actor_area house1"),
        String::from("avoid_actor_area straggler0"),
        String::from("actor_area straggler0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("}"),
        String::from("create_object DIRT_STRAGGLER {"),
        String::from("number_of_objects 2"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("temp_min_distance_group_placement 2"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box5"),
        String::from("avoid_actor_area box4"),
        String::from("avoid_actor_area house1"),
        String::from("avoid_actor_area straggler0"),
        String::from("actor_area straggler0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area jungle-region"),
        String::from("}"),
        String::from("create_object JUNGLE_STRAGGLER {"),
        String::from("number_of_objects 2"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("temp_min_distance_group_placement 2"),
        String::from("avoid_forest_zone 2"),
        String::from("actor_area_to_place_in box5"),
        String::from("avoid_actor_area box4"),
        String::from("avoid_actor_area house1"),
        String::from("avoid_actor_area straggler0"),
        String::from("actor_area straggler0"),
        String::from("actor_area_radius 0"),
        String::from("max_distance_to_players 10"),
        String::from("avoid_actor_area grass-region"),
        String::from("avoid_actor_area snow-region"),
        String::from("avoid_actor_area dirt-region"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("number_of_objects 4"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in straggler0"),
        String::from("actor_area straggler2"),
        String::from("actor_area_radius 2"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
        String::from("create_object PHON {"),
        String::from("set_place_for_every_player"),
        String::from("set_gaia_object_only"),
        String::from("actor_area_to_place_in villager_tree0"),
        String::from("actor_area straggler2"),
        String::from("actor_area_radius 2"),
        String::from("max_distance_to_players 10"),
        String::from("}"),
    ];
    for i in 0..3 {
        lines.push(format!("create_object LUMBERJACK{i} {{"));
        // lines.push(String::from("generate_for_first_land_only"));
        lines.push(String::from("max_distance_to_players 10"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("actor_area_to_place_in villager_tree1"));
        lines.push(String::from("actor_area villager0"));
        lines.push(String::from("actor_area_radius 0"));
        lines.push(String::from("}"));
    }
    // for i in 3..6 {
    //     lines.push(format!("create_object LUMBERJACK{i} {{"));
    //     lines.push(String::from("avoid_actor_area first_land_20"));
    //     lines.push(String::from("max_distance_to_players 10"));
    //     lines.push(String::from("set_place_for_every_player"));
    //     lines.push(String::from("actor_area_to_place_in villager_tree1"));
    //     lines.push(String::from("actor_area villager0"));
    //     lines.push(String::from("actor_area_radius 0"));
    //     lines.push(String::from("}"));
    // }
    lines
}

/// Returns a vector of strings for placing `TEMPORARY_REVEALER`s
/// inside of the `box0` near the TC.
pub fn vision() -> Vec<String> {
    vec![
        String::from("create_object TEMPORARY_REVEALER {"),
        String::from("number_of_objects 4"),
        String::from("actor_area_to_place_in box0"),
        String::from("set_place_for_every_player"),
        String::from("max_distance_to_players 2"),
        String::from("}"),
    ]
}

/// Returns a vector of all strings needed for objects generation
/// for a 9-Villager start.
pub fn objects_9_vils() -> Vec<String> {
    let mut lines = vec![
        String::from("create_object TOWN_CENTER {"),
        String::from("set_place_for_every_player"),
        String::from("max_distance_to_players 0"),
        String::from("}"),
    ];
    lines.append(&mut tc_center());
    lines.append(&mut tc_boxes());
    lines.append(&mut vision());
    lines.append(&mut vils_9_tc());
    lines.append(&mut house_gap_3());
    lines.append(&mut vils_9_straggler());
    for i in 1..10 {
        lines.push(String::from("create_object PHON {"));
        lines.push(String::from("number_of_objects 9"));
        lines.push(String::from("set_place_for_every_player"));
        lines.push(String::from("set_gaia_object_only"));
        lines.push(String::from("actor_area_to_place_in villager0"));
        lines.push(format!("actor_area villager{i}"));
        lines.push(format!("actor_area_radius {i}"));
        lines.push(String::from("}"));
    }
    lines
}

/// Makes the 9-Villager start for ZeWall by using `place_on_specific_land_id`
/// for lands `1`, `2`, `3`, and `4`.
pub fn objects_9_vils_ze_wall() -> Vec<String> {
    let standard = objects_9_vils();
    let mut object: Vec<String> = vec![];
    let mut has_set_place_for_every_player = false;
    let mut lines = vec![];
    for line in standard {
        if line == "}" {
            if has_set_place_for_every_player {
                for i in 1..=4 {
                    for object_line in &object {
                        lines.push(object_line.clone());
                    }
                    lines.push(format!("place_on_specific_land_id {i}"));
                    lines.push(String::from("}"));
                }
                has_set_place_for_every_player = false;
                object.clear();
            } else {
                lines.append(&mut object);
                lines.push(String::from("}"));
            }
        } else if line == "set_place_for_every_player" {
            has_set_place_for_every_player = true;
        } else {
            object.push(line);
        }
    }
    debug_assert!(object.is_empty(), "{object:?}");
    lines
}
