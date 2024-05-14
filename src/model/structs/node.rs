struct Node {
    id: usize,
    position: Point,
    connected: Vec<(f64, usize)>, // -> (distance_to, connected node)
}
