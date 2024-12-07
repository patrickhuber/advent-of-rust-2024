use std::{fs::File, io::{BufRead, BufReader}};

pub struct Report{
    levels: Vec<u32>
}

pub fn calcualte_safe_reports(path: &str) -> Result<u32, String>{
    let reports = read_reports(path)?;
    let mut sum = 0;
    for report in reports{
        if evaluate_report(report) {
            sum += 1;
        }
    }
    Ok(sum)
}

fn evaluate_report(report: Report) -> bool{
    let mut previous: u32;
    let mut increasing = false;
    let mut decreasing = false;
    for (pos, level) in report.levels.iter().enumerate(){
        if pos == 0{
            continue;
        }
        previous = report.levels[pos-1];
        
        // diff outside of 1 and 3 is unsafe
        let diff = level.abs_diff(previous);
        if diff < 1 || diff > 3 {
            return false
        }

        if pos == 1 {
            decreasing = previous > *level;
            increasing = previous < *level;
            continue;
        }
        if decreasing && previous < *level {
            return false;
        }
        if increasing && previous > *level{
            return false;
        }
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