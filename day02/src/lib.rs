use std::{fs::File, io::{BufRead, BufReader}};

pub struct Report{
    levels: Vec<i32>
}

pub fn calculate_safe_reports(path: &str, damper: bool) -> Result<i32, String>{
    let reports = read_reports(path)?;
    let mut safe_count = 0;
    for report in reports{
        print!("{:?}", report.levels);
        if evaluate_report(report, damper) {
            safe_count += 1;
            println!(" // Safe");
        }else{
            println!(" // Unsafe");
        }
    }
    Ok(safe_count)
}

fn read_reports(path: &str) -> Result<Vec<Report>, String>{
    let mut reports = Vec::new();
    let file = File::open(path).map_err(|e|e.to_string())?;
    let reader = BufReader::new(file);
    for line_result in reader.lines(){
        let line = line_result.map_err(|e|e.to_string())?;
        let mut report = Report{
            levels: Vec::new()
        };
        for (_, level) in line.split_whitespace().enumerate(){
            let value = level.parse::<i32>().map_err(|e|e.to_string())?;
            report.levels.push(value);
        }
        reports.push(report);
    }
    return Ok(reports)
}

fn evaluate_report(report: Report, damper: bool) -> bool{    
    if report.levels.len() < 2{
        return true;
    }
    let diffs = diffs(report);
    let err = find_error(&diffs);
    if err.is_none() {
        return true;
    }
    if !damper{
        return false;
    }
    let err_index = err.unwrap();
    
    if err_index > 0{
        let left = merge(&diffs, err_index-1);
        if find_error(&left).is_none(){
            return true;
        }
    }
    if err_index < diffs.len() -1{
        let right = merge(&diffs, err_index);
        if find_error(&right).is_none(){
            return true;
        }
    }
    if err_index <= 1 {
        if find_error(&diffs[1..]).is_none(){
            return true;
        }
    }
    if err_index >= diffs.len() -2 {
        if find_error(&diffs[..diffs.len()-1]).is_none(){
            return true;
        }
    }
    false
}

fn merge(diffs: &Vec<i32>, index: usize) -> Vec<i32>{
    let mut result = Vec::new();
    for i in 0.. diffs.len(){
        if i == index{
            result.push(diffs[i]+diffs[i+1]);
            continue;
        }
        if i == index+1{
            continue;
        }
        result.push(diffs[i]);
    }
    return result;
}

fn diffs(report: Report) -> Vec<i32>{
    let mut diffs = Vec::new();
    for i in 1.. report.levels.len(){
        let left = report.levels[i-1];
        let right = report.levels[i];
        let diff = left - right;
        diffs.push(diff);
    }
    diffs
}

fn find_error(diffs: &[i32])->Option<usize>{   
    let increasing = diffs[0] < 0;
    let mut index = 0;
    for diff in diffs{
        let abs_diff = diff.abs();
        if abs_diff < 1 || abs_diff > 3 || increasing != (*diff < 0){
            return Some(index);
        }
        index += 1;
    }
    return None;
}
