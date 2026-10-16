use std::{str::Bytes, thread::spawn};

use cambc::{ChampionDraft, Result, Team};

pub fn play() -> Result<()> {
    // Choose randomly from the champions our team has not picked.
    let chosen = cambc::draft::current_draft(Team::Ally)?;

    if chosen.is_empty() {
        let groups = cambc::map::camp_groups()?;
        let mut camp_positions: Vec<(u8, u8)> = Vec::new();

        // find all camps
        for group in groups {
            let tiles = group.spawn_tiles()?;
            for tile in tiles.positions()? {
                camp_positions.push((tile.x as u8, tile.y as u8));
            }
        }
        
        // flatten into memory
        let mut result: [u8; 1024] = [0; 1024];
        let mut end_pointer = 0;
        for camp in camp_positions {
            result[end_pointer] = camp.0;
            end_pointer += 1;
            result[end_pointer] = camp.1;
            end_pointer += 1;
        }

        cambc::draft_memory().submit(&result);
        
    }

    let mut available = Vec::new();
    for i in 0..11 {
        let pick = ChampionDraft::try_from(i)?;
        if !chosen.contains(&pick) {
            available.push(pick);
        }
    }


    cambc::draft::pick(available[crate::random_number(available.len() as u32) as usize])
}
