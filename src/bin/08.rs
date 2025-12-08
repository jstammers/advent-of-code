use kiddo::KdTree;
use kiddo::SquaredEuclidean;
use ordered_float::OrderedFloat;
use std::collections::{BinaryHeap, HashMap, HashSet};
use union_find::QuickUnionUf;
use union_find::{UnionBySize, UnionFind}; // add crate union-find = "0.3"
advent_of_code::solution!(8);
fn parse_input(_input: &str) -> Vec<[f64; 3]> {
    // parse the input into a vector of x,y,z coordinates
    let mut coords = Vec::new();
    for line in _input.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 3 {
            continue;
        }
        let x: f64 = parts[0].parse().unwrap();
        let y: f64 = parts[1].parse().unwrap();
        let z: f64 = parts[2].parse().unwrap();
        coords.push([x, y, z]);
    }
    coords
}

fn k_closest_pairs(points: &Vec<[f64; 3]>, k: usize) -> Vec<(f64, usize, usize)> {
    let mut tree: KdTree<f64, 3> = KdTree::new();
    for (i, p) in points.iter().enumerate() {
        tree.add(p, i as u64);
    }

    let mut heap = BinaryHeap::new(); // max-heap by distance

    for (i, p) in points.iter().enumerate() {
        let neighbours = tree.nearest_n::<SquaredEuclidean>(p, k + 5); // overshoot slightly

        for neigbour in neighbours.iter() {
            let j = neigbour.item as usize;
            if i >= j {
                continue;
            }

            let d = neigbour.distance;

            if heap.len() < k {
                heap.push((OrderedFloat(d), i, j));
            } else if d < heap.peek().unwrap().0.0 {
                heap.pop();
                heap.push((OrderedFloat(d), i, j));
            }
        }
    }

    heap.into_sorted_vec()
        .into_iter()
        .map(|(of, i, j)| (of.0, i, j)) // convert OrderedFloat<f64> → f64
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let coords = parse_input(input);
    let mut k = 1000;
    let num_points = coords.len();
    if num_points < k {
        k = 10;
    }
    let k_closest = k_closest_pairs(&coords, k);
    let mut uf: QuickUnionUf<UnionBySize> = UnionFind::<UnionBySize>::new(num_points);

    for (_, i, j) in k_closest {
        uf.union(i, j);
    }

    // Build circuits by grouping ids by representative
    let mut circuits_map: HashMap<usize, HashSet<u64>> = HashMap::new();
    for point in 0..num_points {
        let root = uf.find(point);
        circuits_map
            .entry(root)
            .or_insert_with(HashSet::new)
            .insert(point as u64);
    }

    let circuits: Vec<HashSet<u64>> = circuits_map.into_values().collect();
    // return the product of the size of the first three circuits
    // sort circuits by size
    let mut circuit_sizes: Vec<usize> = circuits.iter().map(|c| c.len()).collect();
    circuit_sizes.sort_by(|a, b| b.cmp(a));
    let result = circuit_sizes
        .iter()
        .take(3)
        .fold(1, |acc, &x| acc * x as u64);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let coords = parse_input(input);
    let k = coords.len() * 1000;
    let num_points = coords.len();
    let k_closest = k_closest_pairs(&coords, k);
    let mut uf: QuickUnionUf<UnionBySize> = UnionFind::<UnionBySize>::new(num_points);

    for (_, i, j) in k_closest {
        uf.union(i, j);
        //get the number of points in the union
        // get max index of points

        let mut roots = Vec::new();
        for point in 0..num_points {
            let root = uf.find(point);
            roots.push(root);
        }
        let max_index = roots.iter().max().unwrap();
        let min_index = roots.iter().min().unwrap();
        if *max_index == *min_index {
            let c1 = coords[i];
            let c2 = coords[j];
            return Some((c1[0] * c2[0]) as u64);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
