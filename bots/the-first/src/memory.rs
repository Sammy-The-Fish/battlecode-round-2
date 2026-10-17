use cambc::{Vec2, Result};


const MEMORY_SEPERATOR: u8 = 255;



pub struct DraftMemory {
    alter_locations: Vec<Vec2>,
    camp_locations: Vec<Vec2>,
}

impl DraftMemory {

    pub fn new(alter_locations: Vec<Vec2>, camp_locations: Vec<Vec2>) -> DraftMemory {
        DraftMemory { alter_locations, camp_locations }
    }

    pub fn load() -> Result<DraftMemory> {
        todo!()
    }


    pub fn save(&self) -> Result<()>{
        let mut result:  [u8; 1024] = [0; 1024];
        let mut end_pointer = 0;

        result[end_pointer] = MEMORY_SEPERATOR;
        end_pointer+=1;



        // flatten into memory
        for pos in &self.alter_locations {
            result[end_pointer] = pos.x as u8;
            end_pointer += 1;
            result[end_pointer] = pos.y as u8;
            end_pointer += 1; 
        }

        for camp in &self.camp_locations {
            result[end_pointer] = camp.x as u8;
            end_pointer += 1;
            result[end_pointer] = camp.y as u8;
            end_pointer += 1;
        }
        
        cambc::draft_memory().submit(&result)
    }

}