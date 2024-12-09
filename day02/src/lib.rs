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

struct Eval {
    increasing: bool,
    ok: bool
}

fn evaluate_report(report: Report) -> bool{
    
    if report.levels.len() < 2{
        return true;
    }

    let mut evals: Vec<Eval> = Vec::new();
    for i in 0..report.levels.len()-1 {
        let current = report.levels[i];
        let next = report.levels[i+1];
        let eval = evaluate(current, next);
        evals.push(eval);
    }

    let increasing = evals[0].increasing;
    let mut faults = 0;
    for eval in evals{
        if eval.increasing != increasing{
            faults += 1;
        }
        if !eval.ok{
            faults += 1;
        }
    }
    faults == 0
}

fn evaluate(current: u32, next: u32) -> Eval {
    let diff = current.abs_diff(next);
    Eval{
        increasing: current < next,
        ok: 1 <= diff && diff <= 3,
    }    
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