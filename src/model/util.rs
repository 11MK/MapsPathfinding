use minidom::Element;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use crate::model::structs::{graph::Graph, point::Point};


pub fn parse_xml(data: &str) -> Graph {
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

// Method to calculate distance (meters) using Haversite Formula
pub fn calculate_distance(cur: &Point, next: &Point) -> f64 {
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
