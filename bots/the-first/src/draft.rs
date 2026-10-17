use cambc::{ChampionDraft, Result, Team};

use crate::memory;

pub fn play() -> Result<()> {
    // Choose randomly from the champions our team has not picked.
    let chosen = cambc::draft::current_draft(Team::Ally)?;

    if chosen.is_empty() {
        // get altar positions
        let alter_positions = cambc::map::altars()?.positions()?;

        // get camp positions
        let groups = cambc::map::camp_groups()?;
        let mut camp_positions = Vec::new();

        // find all camps
        for group in groups {
            let tiles = group.spawn_tiles()?;
            for tile in tiles.positions()? {
                camp_positions.push(tile);
            }
        }

        let memory = memory::DraftMemory::new(alter_positions, camp_positions);

        memory.save()?;
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
