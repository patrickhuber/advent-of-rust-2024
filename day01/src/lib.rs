use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn calculate_diff(path: &str) -> Result<u32, String>{
    let lists = read_lists(path)?;
    let actual = diff(lists[0].clone(), lists[1].clone());
    Ok(actual.iter().sum())
}

pub fn diff(first: Vec<u32>, second: Vec<u32>)-> Vec<u32>{
    let mut diff = Vec::new();
    for i in 0..first.len(){
        let f = first[i];
        let s = second[i];
        diff.push(f.abs_diff(s));
    }
    diff
}

pub fn calculate_similarity(path: &str) -> Result<u32, String>{
    let lists = read_lists(path)?;
    let actual = similarity(lists[0].clone(), lists[1].clone());
    Ok(actual.iter().sum())
}

pub fn similarity(first: Vec<u32>, second: Vec<u32>)-> Vec<u32>{
    let mut sim = Vec::new();
    let mut map: HashMap<u32, u32> = HashMap::new();
    for (_, k) in second.iter().enumerate(){
        match map.get(k){
            Some(v) => {
                _ = map.insert(*k, *v + 1)
            }
            None => {
                _ = map.insert(*k, 1)
            }
        }
    }
    for (_, k) in first.iter().enumerate(){
        match map.get(k){
            Some(v)=>{
                sim.push(v*k);
            }
            None =>{
                sim.push(0);
            }
        }
    }
    sim
}

pub fn read_lists(path: &str) ->  Result<Vec<Vec<u32>>, String> {
    let file = File::open(path).map_err(|e|e.to_string())?;
    let reader = BufReader::new(file);
    let mut vectors: Vec<Vec<u32>> = Vec::new();
    vectors.push(Vec::new());
    vectors.push(Vec::new());

    for line_result in reader.lines(){
        let line = line_result.map_err(|e|e.to_string())?;
        if line.len() == 0 {
            continue;
        }
        let split = line.split_whitespace().map(str::to_string);
        for (pos, e) in split.enumerate(){
            let vector = &mut vectors[pos];
            let num = e.parse::<u32>().map_err(|e|e.to_string())?;
            let pos = vector.binary_search(&num).unwrap_or_else(|e| e);
            vector.insert(pos, num);
        }
    }
    Ok(vectors)
}