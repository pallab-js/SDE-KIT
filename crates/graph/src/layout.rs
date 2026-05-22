use crate::types::{GraphEdge, GraphNode, NodePosition};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::collections::HashMap;

// Constants for the simple function-based layout
const REPULSION: f64 = 5000.0;
const ATTRACTION: f64 = 0.01;
const DAMPING: f64 = 0.85;
const ITERATIONS: usize = 100;
const CENTER_GRAVITY: f64 = 0.01;
const MIN_DIST: f64 = 10.0;

/// Quick force-directed layout using a simple function API.
/// Positions nodes in a circle initially, then iterates to minimize edge crossings.
pub fn force_directed(
    nodes: &[GraphNode],
    edges: &[GraphEdge],
    width: f64,
    height: f64,
) -> Vec<NodePosition> {
    let mut positions: HashMap<String, (f64, f64)> = HashMap::new();
    let mut velocities: HashMap<String, (f64, f64)> = HashMap::new();

    let count = nodes.len() as f64;
    let cx = width / 2.0;
    let cy = height / 2.0;
    let radius = (width.min(height) / 2.0) * 0.6;

    for (i, node) in nodes.iter().enumerate() {
        let angle = 2.0 * std::f64::consts::PI * (i as f64) / count;
        let x = cx + radius * angle.cos();
        let y = cy + radius * angle.sin();
        positions.insert(node.id.clone(), (x, y));
        velocities.insert(node.id.clone(), (0.0, 0.0));
    }

    let edge_set: Vec<(String, String)> = edges
        .iter()
        .map(|e| (e.source_id.clone(), e.target_id.clone()))
        .collect();

    for _ in 0..ITERATIONS {
        let mut forces: HashMap<String, (f64, f64)> = HashMap::new();
        for node in nodes {
            forces.insert(node.id.clone(), (0.0, 0.0));
        }

        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let a = &nodes[i];
                let b = &nodes[j];
                let (ax, ay) = positions[&a.id];
                let (bx, by) = positions[&b.id];
                let dx = bx - ax;
                let dy = by - ay;
                let dist = dx.hypot(dy).max(MIN_DIST);
                let force = REPULSION / (dist * dist);
                let fx = force * dx / dist;
                let fy = force * dy / dist;
                let f = forces.get_mut(&a.id).unwrap();
                f.0 -= fx;
                f.1 -= fy;
                let f = forces.get_mut(&b.id).unwrap();
                f.0 += fx;
                f.1 += fy;
            }
        }

        for (src, tgt) in &edge_set {
            if let (Some(&sp), Some(&tp)) = (positions.get(src), positions.get(tgt)) {
                let dx = tp.0 - sp.0;
                let dy = tp.1 - sp.1;
                let dist = dx.hypot(dy).max(MIN_DIST);
                let force = ATTRACTION * (dist - 100.0);
                let fx = force * dx / dist;
                let fy = force * dy / dist;
                let f = forces.get_mut(src).unwrap();
                f.0 += fx;
                f.1 += fy;
                let f = forces.get_mut(tgt).unwrap();
                f.0 -= fx;
                f.1 -= fy;
            }
        }

        for node in nodes {
            let (x, y) = positions[&node.id];
            let f = forces.get_mut(&node.id).unwrap();
            f.0 -= (x - cx) * CENTER_GRAVITY;
            f.1 -= (y - cy) * CENTER_GRAVITY;

            let v = velocities.get_mut(&node.id).unwrap();
            v.0 = (v.0 + f.0) * DAMPING;
            v.1 = (v.1 + f.1) * DAMPING;

            let pos = positions.get_mut(&node.id).unwrap();
            pos.0 += v.0;
            pos.1 += v.1;
            pos.0 = pos.0.clamp(20.0, width - 20.0);
            pos.1 = pos.1.clamp(20.0, height - 20.0);
        }
    }

    nodes
        .iter()
        .map(|n| {
            let (x, y) = positions[&n.id];
            NodePosition {
                id: n.id.clone(),
                x,
                y,
            }
        })
        .collect()
}

/// Configurable force-directed layout engine.
///
/// Provides deterministic seeding for reproducible layouts (local-first),
/// configurable physics parameters, mass-per-node-type, and convergence detection.
pub struct ForceDirectedLayout {
    /// Coulomb-like repulsion constant between any two nodes
    pub repulsion_strength: f64,
    /// Hooke-like spring constant for edges
    pub attraction_strength: f64,
    /// Center gravity to prevent drift (applied toward center of bounds)
    pub center_gravity: f64,
    /// Velocity damping factor (0.0–1.0); lower = more damping
    pub damping: f64,
    /// Maximum velocity magnitude (velocity clamping)
    pub max_velocity: f64,
    /// Simulation time step
    pub step_size: f64,
    /// Rest length for edge springs
    pub rest_length: f64,
    /// Minimum distance to prevent division-by-zero
    pub min_distance: f64,
    /// Layout bounds (min_x, min_y, max_x, max_y)
    pub bounds: (f64, f64, f64, f64),
    /// Deterministic seed for reproducible layout (local-first guarantee)
    pub seed: u64,
    /// Mass assigned per node type (by node_type string)
    pub masses: HashMap<String, f64>,
}

impl Default for ForceDirectedLayout {
    fn default() -> Self {
        Self {
            repulsion_strength: 5000.0,
            attraction_strength: 0.05,
            center_gravity: 0.01,
            damping: 0.9,
            max_velocity: 10.0,
            step_size: 0.1,
            rest_length: 100.0,
            min_distance: 5.0,
            bounds: (-500.0, -500.0, 500.0, 500.0),
            seed: 42,
            masses: HashMap::new(),
        }
    }
}

impl ForceDirectedLayout {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            ..Self::default()
        }
    }

    /// Run the full simulation, returning final positions.
    /// `max_iterations`: hard cap on steps.
    /// `energy_threshold`: stop early when total kinetic energy drops below this.
    pub fn simulate(
        &self,
        nodes: &[GraphNode],
        edges: &[GraphEdge],
        max_iterations: usize,
        energy_threshold: f64,
    ) -> Vec<NodePosition> {
        if nodes.is_empty() {
            return vec![];
        }

        let (min_x, min_y, max_x, max_y) = self.bounds;
        let cx = (min_x + max_x) / 2.0;
        let cy = (min_y + max_y) / 2.0;

        // Initialize positions deterministically
        let mut rng = StdRng::seed_from_u64(self.seed);
        let mut pos: HashMap<&str, (f64, f64)> = HashMap::new();
        let mut vel: HashMap<&str, (f64, f64)> = HashMap::new();

        for node in nodes {
            let x = rng.gen_range(min_x..max_x);
            let y = rng.gen_range(min_y..max_y);
            pos.insert(&node.id, (x, y));
            vel.insert(&node.id, (0.0, 0.0));
        }

        // Build edge index list
        let edge_pairs: Vec<(&str, &str)> = edges
            .iter()
            .map(|e| (e.source_id.as_str(), e.target_id.as_str()))
            .collect();

        // Pre-compute masses per node type
        let default_mass = 1.0;

        for _iter in 0..max_iterations {
            // Compute forces
            let mut forces: HashMap<&str, (f64, f64)> = HashMap::new();
            for node in nodes {
                forces.insert(node.id.as_str(), (0.0, 0.0));
            }

            // Repulsion
            for i in 0..nodes.len() {
                for j in (i + 1)..nodes.len() {
                    let a = &nodes[i];
                    let b = &nodes[j];
                    let (ax, ay) = pos[&a.id.as_str()];
                    let (bx, by) = pos[&b.id.as_str()];
                    let dx = bx - ax;
                    let dy = by - ay;
                    let dist_sq = dx * dx + dy * dy + self.min_distance * self.min_distance;
                    let dist = dist_sq.sqrt();

                    let force = self.repulsion_strength / dist_sq;
                    let fx = force * dx / dist;
                    let fy = force * dy / dist;

                    let f_a = forces.get_mut(a.id.as_str()).unwrap();
                    f_a.0 -= fx;
                    f_a.1 -= fy;
                    let f_b = forces.get_mut(b.id.as_str()).unwrap();
                    f_b.0 += fx;
                    f_b.1 += fy;
                }
            }

            // Attraction along edges
            for &(src, tgt) in &edge_pairs {
                if let (Some(&sp), Some(&tp)) = (pos.get(src), pos.get(tgt)) {
                    let dx = tp.0 - sp.0;
                    let dy = tp.1 - sp.1;
                    let dist = dx.hypot(dy).max(self.min_distance);

                    let force = self.attraction_strength * (dist - self.rest_length);
                    let fx = force * dx / dist;
                    let fy = force * dy / dist;

                    if let Some(f) = forces.get_mut(src) {
                        f.0 += fx;
                        f.1 += fy;
                    }
                    if let Some(f) = forces.get_mut(tgt) {
                        f.0 -= fx;
                        f.1 -= fy;
                    }
                }
            }

            // Center gravity + integrate
            let mut total_energy = 0.0;
            for node in nodes {
                let id = node.id.as_str();
                let (x, y) = pos[id];
                let mass = self
                    .masses
                    .get(&node.node_type)
                    .copied()
                    .unwrap_or(default_mass);
                let f = forces.get_mut(id).unwrap();

                // Center gravity
                f.0 -= (x - cx) * self.center_gravity;
                f.1 -= (y - cy) * self.center_gravity;

                // Integrate
                let v = vel.get_mut(id).unwrap();
                v.0 = (v.0 + f.0 / mass) * self.damping;
                v.1 = (v.1 + f.1 / mass) * self.damping;

                // Velocity clamping
                let speed = (v.0 * v.0 + v.1 * v.1).sqrt();
                if speed > self.max_velocity {
                    let scale = self.max_velocity / speed;
                    v.0 *= scale;
                    v.1 *= scale;
                }

                // Position update
                let p = pos.get_mut(id).unwrap();
                p.0 += v.0 * self.step_size;
                p.1 += v.1 * self.step_size;
                p.0 = p.0.clamp(min_x, max_x);
                p.1 = p.1.clamp(min_y, max_y);

                total_energy += 0.5 * mass * (v.0 * v.0 + v.1 * v.1);
            }

            if total_energy < energy_threshold {
                break;
            }
        }

        nodes
            .iter()
            .map(|n| {
                let (x, y) = pos[n.id.as_str()];
                NodePosition {
                    id: n.id.clone(),
                    x,
                    y,
                }
            })
            .collect()
    }
}
