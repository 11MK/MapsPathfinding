use minidom::Element;
use std::collections::HashMap;

struct Node {
    lat: f32,
    lon: f32,
    connected: Vec<(f32, usize)>, // -> (distance_to, connected node)
}

struct Graph {
    nodes: HashMap<usize, Node>, // -> (uid, node)
}

impl Graph {
    // Method to add a node to the graph
    fn add_node(&mut self, id: usize, latitude: f32, longitude: f32) {
        let node = Node {
            lat: latitude,
            lon: longitude,
            connected: Vec::new(),
        };
        print!("Node: {}, lat: {}, lon: {}", id, node.lat, node.lon);
        self.nodes.insert(id, node);
    }

    // Method to add a connected node
    fn add_connection(&mut self, cur_id: usize, next_id: usize) {
        if self.nodes.get(&cur_id).is_none() || self.nodes.get(&next_id).is_none() {
            return;
        }
        let cur_node_ref = self.nodes.get(&cur_id).unwrap();
        let next_node_ref = self.nodes.get(&next_id).unwrap();
        let distance = calculate_distance(cur_node_ref, next_node_ref);
        let cur_node = self.nodes.get_mut(&cur_id).unwrap();
        cur_node.connected.push((distance, next_id))
    }

    // Method to get a reference to a node by its ID
    fn get_node(&mut self, id: usize) -> Option<&Node> {
        self.nodes.get(&id)
    }

    fn find_nearest(&self, lat: f32, lon: f32) -> &Node {
        let mut min_distance = f32::MAX;
        let mut nearest_node = &self.nodes[&0];
        for node in self.nodes.values() {
            let distance = calculate_distance(&Node { lat, lon, connected: Vec::new() }, node);
            if distance < min_distance {
                min_distance = distance;
                nearest_node = node;
            }
        }
        nearest_node 
    }
}

// Method to calculate distance (meters) using Haversite Formula
fn calculate_distance(cur_node: &Node, next_node: &Node) -> f32 {
    let lat1 = cur_node.lat.to_radians();
    let lon1 = cur_node.lon.to_radians();
    let lat2 = next_node.lat.to_radians();
    let lon2 = next_node.lon.to_radians();
    let delta_lat = lat2 - lat1;
    let delta_lon = lon2 - lon1;
    let a =
        (delta_lat / 2.0).sin().powi(2) + lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    (6371.0 * c) * 1000.
}

fn parse_xml(data: &str) -> Graph {
    let mut graph = Graph {
        nodes: HashMap::new(),
    };
    // MiniDom requires "xmlns" attribute, which is not returned from API request
    let data = data.replace("<osm", "<osm xmlns=\"arbitrary\"");
    let root: Element = data.parse().unwrap();
    for node_element in root.children().filter(|e| e.name() == "node") {
        let id = node_element
            .attr("id")
            .expect("No attribute called \'id\'")
            .parse::<usize>()
            .expect("Couldn't parse &str to usize");
        let latitude = node_element
            .attr("lat")
            .expect("No attribute called \'lat\'")
            .parse::<f32>()
            .expect("Couldn't parse &str to f64");
        let longitude = node_element
            .attr("lon")
            .expect("No attribute called \'lat\'")
            .parse::<f32>()
            .expect("Couldn't parse &str to f64");
        graph.add_node(id, latitude, longitude);
    }

    for way_element in root.children().filter(|e| e.name() == "way") {
        println!("nd ref elements:");
        let mut filtered_elements = way_element.children().filter(|e| e.name() == "nd");
        let mut cur_node_id = filtered_elements
            .next()
            .expect("Empty Iterator")
            .attr("ref")
            .expect("No attribute called \'ref\'")
            .parse::<usize>()
            .expect("Couldn't parse &str to usize");
        for nd_element in filtered_elements {
            let next_node_id = nd_element
                .attr("ref")
                .expect("No attribute called \'ref\'")
                .parse::<usize>()
                .expect("Couldn't parse &str to usize");
            graph.add_connection(cur_node_id, next_node_id);
            cur_node_id = next_node_id;
        }
    }
    graph
}

fn main() {
    // read ./data.xml into str
    let data = include_str!("../data2.xml");
    // let data = std::fs::read_to_string("./data.xml").expect("Unable to read file");
    parse_xml(data);
}

// async fn fetch_street_data(l: f64, r: f64, t: f64, b: f64) {
//     let request_url = format!("https://www.openstreetmap.org/api/0.6/map?bbox={l},{b},{r},{t}");
//
//     let resp = match reqwest::get(request_url).await {
//         Ok(resp) => resp.text().await.unwrap(),
//         Err(err) => panic!("Error: {}", err)
//     };
//     // output resp to json file in ./data.json
//     std::fs::write("./data.json", resp).expect("Unable to write file");
// }

// #[tokio::main]
// async fn main() {
//     let left = -111.0542;
//     let bot = 45.6687;
//     let right = -111.0324;
//     let top = 45.6794;
//     fetch_street_data(left, right, top, bot).await;
// }
