use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;
use std::f64::INFINITY;
use serde::{Deserialize, Serialize};
use crate::models::{Node, Edge, Coordinate, RouteResult};

#[derive(Debug, Clone, PartialEq)]
pub struct AStarNode {
    pub id: u64,
    pub g_score: f64,
    pub f_score: f64,
    pub parent: Option<u64>,
}

impl Eq for AStarNode {}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.partial_cmp(&self.f_score).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteRequest {
    pub start: Coordinate,
    pub end: Coordinate,
    pub vehicle_type: String,
    pub avoid_tolls: bool,
    pub avoid_highways: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteResponse {
    pub path: Vec<Coordinate>,
    pub distance: f64,
    pub duration: f64,
    pub instructions: Vec<String>,
    pub success: bool,
    pub error: Option<String>,
}

pub struct RoutingService {
    nodes: HashMap<u64, Node>,
    edges: HashMap<u64, Vec<Edge>>,
}

impl RoutingService {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn load_graph(&mut self, nodes: Vec<Node>, edges: Vec<Edge>) {
        self.nodes = nodes.into_iter().map(|n| (n.id, n)).collect();
        
        for edge in edges {
            self.edges.entry(edge.from_node).or_insert_with(Vec::new).push(edge);
        }
    }

    pub fn find_route(&self, request: &RouteRequest) -> RouteResponse {
        let start_node = match self.find_nearest_node(&request.start) {
            Some(node) => node,
            None => return RouteResponse {
                path: vec![],
                distance: 0.0,
                duration: 0.0,
                instructions: vec![],
                success: false,
                error: Some("Start location not found".to_string()),
            }
        };

        let end_node = match self.find_nearest_node(&request.end) {
            Some(node) => node,
            None => return RouteResponse {
                path: vec![],
                distance: 0.0,
                duration: 0.0,
                instructions: vec![],
                success: false,
                error: Some("End location not found".to_string()),
            }
        };

        match self.a_star(start_node, end_node, request) {
            Some(result) => {
                let instructions = self.generate_instructions(&result.path);
                RouteResponse {
                    path: result.path,
                    distance: result.distance,
                    duration: result.duration,
                    instructions,
                    success: true,
                    error: None,
                }
            }
            None => RouteResponse {
                path: vec![],
                distance: 0.0,
                duration: 0.0,
                instructions: vec![],
                success: false,
                error: Some("No route found".to_string()),
            }
        }
    }

    fn a_star(&self, start: u64, goal: u64, request: &RouteRequest) -> Option<RouteResult> {
        let mut open_set = BinaryHeap::new();
        let mut closed_set = HashSet::new();
        let mut g_