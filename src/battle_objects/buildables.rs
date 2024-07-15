use crate::battle_objects::coordinates::{GridCoord};

#[derive(Clone, Copy)]
pub struct Wall{
    //walls are corner-aligned. To convert them to game coordinates, default to the top left
    pub endpoints: (GridCoord, GridCoord),
    pub health: (usize, usize),
}

#[derive(Clone, Copy)]
pub struct Window{
    //windows are corner-aligned. To convert them to game coordinates, default to the top left
    pub endpoints: (GridCoord, GridCoord),
    pub health: (usize, usize),
}