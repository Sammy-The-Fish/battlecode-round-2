use cambc::{Result, Vec2};

const MEMORY_SEPERATOR: u8 = 255;

pub struct DraftMemory {
    pub alter_locations: Vec<Vec2>,
    camp_locations: Vec<Vec2>,
}

impl DraftMemory {
    pub fn new(alter_locations: Vec<Vec2>, camp_locations: Vec<Vec2>) -> DraftMemory {
        DraftMemory {
            alter_locations,
            camp_locations,
        }
    }

    pub fn load() -> DraftMemory {
        let mut alter_locations = Vec::new();
        let mut camp_locations = Vec::new();

        let memory = cambc::draft_memory().bytes();

        // get alter locations
        let mut is_x_byte = false;
        let mut pos = Vec2::new(0, 0);
        for byte in memory {
            is_x_byte = !is_x_byte;
            if is_x_byte {
                pos = Vec2 { x: 0, y: 0 }
            }
            match byte {
                255 => {
                    break;
                }
                _ => {
                    if is_x_byte {
                        pos.x = *byte as i32;
                    } else {
                        pos.y = *byte as i32;

                        alter_locations.push(pos);
                    }
                }
            }
        }

        // get alter locations
        let mut is_x_byte = false;
        let mut pos = Vec2::new(0, 0);
        for byte in memory {
            is_x_byte = !is_x_byte;
            if is_x_byte {
                pos = Vec2 { x: 0, y: 0 }
            }
            match byte {
                255 => {
                    break;
                }
                _ => {
                    if is_x_byte {
                        pos.x = *byte as i32;
                    } else {
                        pos.y = *byte as i32;

                        camp_locations.push(pos);
                    }
                }
            }
        }

        DraftMemory {
            alter_locations,
            camp_locations,
        }
    }

    pub fn save(&self) -> Result<()> {
        let mut result: [u8; 1024] = [0; 1024];
        let mut end_pointer = 0;

        // flatten into memory
        for pos in &self.alter_locations {
            result[end_pointer] = pos.x as u8;
            end_pointer += 1;
            result[end_pointer] = pos.y as u8;
            end_pointer += 1;
        }
        result[end_pointer] = MEMORY_SEPERATOR;
        end_pointer += 1;

        for camp in &self.camp_locations {
            result[end_pointer] = camp.x as u8;
            end_pointer += 1;
            result[end_pointer] = camp.y as u8;
            end_pointer += 1;
        }

        cambc::draft_memory().submit(&result)
    }
}
