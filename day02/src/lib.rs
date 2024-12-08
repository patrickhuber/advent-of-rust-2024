use std::{fs::File, io::{BufRead, BufReader}};

pub struct Report{
    levels: Vec<u32>
}

pub fn calculate_safe_reports(path: &str) -> Result<u32, String>{
    let reports = read_reports(path)?;
    let mut sum = 0;
    for report in reports{
        if evaluate_report(report) {
            sum += 1;
        }
    }
    Ok(sum)
}

pub fn calculate_safe_reports_with_daper(path: &str, damper: u32) -> Result<u32, String>{
    Ok(1)
}

fn evaluate_report(report: Report) -> bool{
    let mut current = 0;
    let mut next = 1;
    let mut increasing = false;
    while next < report.levels.len(){
        let current_value = report.levels[current];
        let next_value = report.levels[next];
        if current == 0{
            increasing = current_value < next_value;
        }
        let diff = current_value.abs_diff(next_value) ;
        if diff < 1 || diff > 3{
            return false
        }
        if increasing && current_value > next_value{
            return false
        }
        if !increasing && current_value < next_value{
            return false
        }
        current = next;
        next += 1
    }
    true
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
            let value = level.parse::<u32>().map_err(|e|e.to_string())?;
            report.levels.push(value);
        }
        reports.push(report);
    }
    return Ok(reports)
}