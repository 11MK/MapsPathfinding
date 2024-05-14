
struct Graph {
    nodes: HashMap<usize, Node>, // -> (uid, node)
}

impl Graph {
    // Method to add a node to the graph
    fn add_node(&mut self, id: usize, latitude: f64, longitude: f64) {
        let node = Node {
            id,
            position: Point {
                lat: latitude,
                lon: longitude,
            },
            connected: Vec::new(),
        };
        self.nodes.insert(id, node);
    }

    // Method to add a connected node
    fn add_connection(&mut self, cur_id: usize, next_id: usize) {
        if let Some(cur) = self.nodes.get(&cur_id) {
            if let Some(nxt) = self.nodes.get(&next_id) {
                let distance = calculate_distance(&cur.position, &nxt.position);
                self.nodes
                    .get_mut(&cur_id)
                    .unwrap()
                    .connected
                    .push((distance, next_id))
            }
        }
    }

    // Method to get a reference to a node by its ID
    fn get_node(&mut self, id: usize) -> Option<&Node> {
        self.nodes.get(&id)
    }

    // Method to find nearest node to given (lat, lon) coordinates
    fn find_nearest(&self, target: Point) -> &Node {
        let mut min_distance = f64::MAX;
        let mut nearest_node = None;
        for n in self.nodes.values() {
            let distance = calculate_distance(&target, &n.position);
            if distance < min_distance {
                min_distance = distance;
                nearest_node = Some(n);
            }
        }
        nearest_node.unwrap()
    }

    // Method to get connected nodes for a node
    fn get_neighbors(&self, id: usize) -> Option<&Vec<(f64, usize)>> {
        self.nodes.get(&id).map(|node| &node.connected)
    }
}
