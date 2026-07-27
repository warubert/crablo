//Monsters
pub struct Monster {
    pub x: usize,
    pub y: usize,
    pub hp: i32,
    pub cd: f32,
}

//Floating text
pub struct DmgText {
    pub x: f32,
    pub y: f32,
    pub dmg: i32, //negative dmg will be treated as gold collected
    pub life: f32,
}
