use crate::model::structs::point::Point;

pub struct Node {
    pub id: usize,
    pub position: Point,
    pub connected: Vec<(f64, usize)>, // -> (distance_to, connected node)
}
