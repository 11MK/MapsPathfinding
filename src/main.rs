use minidom::Element;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Clone, Copy)]
struct Point {
    lat: f64,
    lon: f64,
}

struct Node {
    id: usize,
    position: Point,
    connected: Vec<(f64, usize)>, // -> (distance_to, connected node)
}

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

// Method to calculate distance (meters) using Haversite Formula
fn calculate_distance(cur: &Point, next: &Point) -> f64 {
    let lat1 = cur.lat.to_radians();
    let lon1 = cur.lon.to_radians();
    let lat2 = next.lat.to_radians();
    let lon2 = next.lon.to_radians();
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
            .parse::<f64>()
            .expect("Couldn't parse &str to f64");
        let longitude = node_element
            .attr("lon")
            .expect("No attribute called \'lat\'")
            .parse::<f64>()
            .expect("Couldn't parse &str to f64");
        graph.add_node(id, latitude, longitude);
    }

    for way_element in root.children().filter(|e| e.name() == "way") {
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

#[derive(Clone)]
struct State {
    id: usize,
    f_cost: f64,
    g_cost: f64,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f_cost
            .partial_cmp(&self.f_cost)
            .unwrap_or(Ordering::Equal)
    }
}

// Backtrack through nodes to recover path, return vec[Points]
fn reconstruct_path(graph: &Graph, came_from: HashMap<usize, usize>, current: usize) -> Vec<Point> {
    let mut total_path = vec![graph.nodes[&current].position];
    let mut node = current;
    while came_from.contains_key(&node) {
        node = came_from[&node];
        total_path.push(graph.nodes[&node].position);
    }
    total_path.reverse();
    total_path
}

// Pathfinding Algorithm,
fn a_star(graph: &Graph, start: Point, goal: Point) -> Option<Vec<Point>> {
    let st_node = graph.find_nearest(start);
    let gl_node = graph.find_nearest(goal);
    let mut came_from = HashMap::new();
    let mut g_score: HashMap<usize, f64> = HashMap::new();
    let mut open_set = BinaryHeap::new();
    open_set.push(State {
        id: st_node.id,
        f_cost: calculate_distance(&start, &goal),
        g_cost: 0.,
    });
    g_score.insert(st_node.id, 0.0);
    println!("{:?}, {:?}",st_node.id, gl_node.id);
    while let Some(State { id, .. }) = open_set.pop() {
        if id == gl_node.id {
            return Some(reconstruct_path(graph, came_from, id));
        }
        for &(distance, neighbor) in graph.get_neighbors(id).unwrap_or(&vec![]) {
            println!("{:?}", open_set.len());
            let tentative_g_score = g_score[&id] + distance;
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f64::MAX) {
                came_from.insert(neighbor, id);
                g_score.insert(neighbor, tentative_g_score);
                let f_score =
                    tentative_g_score + calculate_distance(&graph.nodes[&neighbor].position, &goal);
                open_set.push(State {
                    id: neighbor,
                    f_cost: f_score,
                    g_cost: tentative_g_score,
                });
            }
        }
    }
    None
}

fn main() {
    // read ./data.xml into str
    let data = include_str!("../data3.xml");
    // let data = std::fs::read_to_string("./data.xml").expect("unable to read file");
    let graph = parse_xml(data);
    // let start = Point { lat: 45.6762759, lon: -111.042363 };
    // let goal = Point { lat: 45.6723792, lon: -111.0479298 };
    let start = Point {
        lat: 45.6408383,
        lon: -111.0372238,
    };
    let goal = Point {
        lat: 45.6480019,
        lon: -111.0377540,
    };
    let path = a_star(&graph, start, goal);
    // print path
    match path {
        Some(p) => {
            for point in p {
                println!("lat: {}, lon: {}", point.lat, point.lon);
            }
        }
        None => println!("No path found"),
    }
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
