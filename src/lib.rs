use std::cmp::min;
use std::collections::HashMap;
use std::str::FromStr;

use kodama::{linkage, Method, Dendrogram};
use rayon::prelude::*;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn dissimilarity(a: &str, b: &str) -> f32 {
    let length_a = a.chars().count();
    let length_b = b.chars().count();
    let len_sum = length_a + length_b;
    levenshtein(a, length_a, b, length_b, 2) as f32 / len_sum as f32
}

fn dissimilarity_matrix_par(l: &[&str]) -> Vec<f32> {
    let n = l.len();
    // FIXME should iterate over `triu`
    // let mat_len = (n - 1) * n / 2;
    let mat_len = n * n;
    (0..mat_len).into_par_iter()
        .map(|raw_index| (raw_index / n, raw_index % n))
        .filter(|(i, j)| i < j)
        .map(|(i, j)| dissimilarity(l[i], l[j]))
        .collect()
}

fn levenshtein(a: &str, length_a: usize, b: &str, length_b: usize, rep_cost: usize) -> usize {
    if length_a == 0 {
        return length_b;
    }
    if length_b == 0 {
        return length_a;
    }

    let mut cache: Vec<usize> = (1..length_b + 1).collect();

    let mut result = 0;
    for (i, a_elem) in a.chars().enumerate() {
        result = i + 1;
        let mut distance_b = i;

        for (j, b_elem) in b.chars().enumerate() {
            let cost = if a_elem == b_elem { 0 } else { rep_cost };
            let distance_a = distance_b + cost;
            distance_b = cache[j];
            result = min(result + 1, min(distance_a, distance_b + 1));
            cache[j] = result;
        }
    }
    result
}

fn linkage_by_levenshtein_dissimilarity(data: Vec<&str>, method: Method) -> Dendrogram<f32> {
    let mut matrix = dissimilarity_matrix_par(&data);
    linkage(&mut matrix, data.len(), method)
}

#[pyfunction]
fn clustering(data: Vec<&str>, threshold: f32, method: &str) -> Vec<i32> {
    let observations = data.len();
    let clustering = linkage_by_levenshtein_dissimilarity(data, Method::from_str(method).unwrap());

    let mut marked = HashMap::new();
    let mut id_iter = 0..;
    let mut processed = vec![-1; observations];
    for (i, step) in clustering.steps().iter().enumerate().rev() {
        if step.dissimilarity >= threshold {
            continue;
        }

        let cluster = i + observations;
        let cur_id = match marked.remove(&cluster) {
            Some(id) => id,
            _ => id_iter.next().unwrap(),
        };

        if step.cluster1 < observations {
            processed[step.cluster1] = cur_id;
        } else {
            marked.insert(step.cluster1, cur_id);
        }

        if step.cluster2 < observations {
            processed[step.cluster2] = cur_id;
        } else {
            marked.insert(step.cluster2, cur_id);
        }
    }

    for item in processed.iter_mut() {
        if *item == -1 {
            *item = id_iter.next().unwrap();
        }
    }
    processed
}

#[pymodule]
fn leven_clustering(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(dissimilarity, m)?)?;
    m.add_function(wrap_pyfunction!(clustering, m)?)?;

    Ok(())
}
