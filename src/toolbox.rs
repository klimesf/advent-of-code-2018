use core::mem;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::{Add, Sub};

pub(crate) fn gcd(mut a: i32, mut b: i32) -> i32 {
    if b > a {
        mem::swap(&mut a, &mut b);
    }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a.max(-a);
}

pub(crate) fn lcm(a: i32, b: i32) -> i32 {
    if a > b {
        (a * b) / gcd(a, b)
    } else {
        (a * b) / gcd(b, a)
    }
}

pub(crate) fn gcd_64(mut a: i64, mut b: i64) -> i64 {
    if b > a {
        mem::swap(&mut a, &mut b);
    }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a.max(-a);
}

pub(crate) fn lcm_64(a: i64, b: i64) -> i64 {
    if a > b {
        (a * b) / gcd_64(a, b)
    } else {
        (a * b) / gcd_64(b, a)
    }
}

pub(crate) fn manhattan_distance<T>(a: (T, T), b: (T, T)) -> <<T as Sub>::Output as Add>::Output
    where T: Sub + PartialOrd, <T as Sub<T>>::Output: Add
{
    let x = if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 };
    let y = if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 };
    x + y
}

pub(crate) fn topological_order<T: Hash + Eq + Copy + Ord>(graph: &HashMap<T, Vec<T>>) -> Vec<T> {
    let mut ordering: Vec<T> = vec!();
    let mut visited = HashSet::new();
    for start in graph.keys() {
        run_topological_order(*start, &mut ordering, &mut visited, graph);
    }
    return ordering;
}

fn run_topological_order<T: Hash + Eq + Copy + Ord>(
    current: T,
    ordering: &mut Vec<T>,
    visited: &mut HashSet<T>,
    graph: &HashMap<T, Vec<T>>,
) {
    if visited.contains(&current) { return; }

    let edges = match graph.get(&current) {
        Some(edges) => edges.clone(),
        None => Vec::new(),
    };

    for e in edges {
        run_topological_order(e, ordering, visited, graph);
    }

    visited.insert(current.clone());
    ordering.push(current.clone());
}

#[cfg(test)]
mod toolbox_tests {
    use std::collections::HashMap;

    use crate::toolbox::{gcd, gcd_64, lcm, lcm_64, manhattan_distance, topological_order};

    #[test]
    fn gcd_works() {
        assert_eq!(6, gcd(30, 24));
        assert_eq!(6, gcd(24, 30));
        assert_eq!(6, gcd_64(30, 24));
        assert_eq!(6, gcd_64(24, 30));
    }

    #[test]
    fn lcm_works() {
        assert_eq!(40, lcm(5, 8));
        assert_eq!(40, lcm(8, 5));
        assert_eq!(40, lcm_64(5, 8));
        assert_eq!(40, lcm_64(8, 5));
    }

    #[test]
    fn manhattan_distance_works() {
        assert_eq!(10, manhattan_distance((0, 0), (5, 5)));
        assert_eq!(10, manhattan_distance((5, 5), (0, 0)));
        assert_eq!(0, manhattan_distance((0, 0), (0, 0)));
        assert_eq!(10, manhattan_distance((-5, -5), (0, 0)));
        assert_eq!(10, manhattan_distance((-5, -5), (-10, -10)));
    }

    #[test]
    fn topological_order_works() {
        let mut graph = HashMap::new();
        graph.insert('A', vec!['B', 'C']);
        graph.insert('B', vec!['C', 'D', 'E']);
        graph.insert('C', vec!['D']);
        graph.insert('D', vec!['E']);

        let order = topological_order(&graph);
        assert_eq!(vec!['E', 'D', 'C', 'B', 'A'], order);
    }
}
