use std::{fs::File, io::Read};
use regex::Regex;

pub fn run_program(path: &str, enable_control: bool) -> Result<i32, String>{
    let instructions = parse_file(path)?;
    let mut sum =0;
    let mut enabled = true;
    for inst in instructions{
        match inst.kind{
            Kind::Do => { enabled=true},
            Kind::Dont => { enabled=false },
            Kind::Mul => {
                if enabled || !enable_control{
                    sum += inst.left * inst.right;
                } 
            }
        }
    }
    Ok(sum)
}

struct Instruction{
    kind: Kind,
    left: i32,
    right: i32
}

#[derive(PartialEq)]
enum Kind {
    Do,
    Dont,
    Mul,
}

fn parse_file(path: &str) -> Result<Vec<Instruction>, String>{  
    let mut file = File::open(path).map_err(|e|e.to_string())?;
    let mut buf = String::new();
    file.read_to_string(&mut buf).map_err(|e|e.to_string())?;
    return parse(buf);
}

fn parse(content: String) -> Result<Vec<Instruction>, String>{    
    let mut instructions = Vec::new();
    let pattern = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").map_err(|e|e.to_string())?;
    for capture in pattern.find_iter(&content){
        let instruction = parse_instruction(capture.as_str())?;
        instructions.push(instruction);
    }
    Ok(instructions)
}

fn parse_instruction(content: &str) -> Result<Instruction, String>{
    let name_split: Vec<&str> = content.split("(").collect();
    let kind = match name_split[0]{
        "mul" => Kind::Mul,
        "do" => Kind::Do,
        "don't" => Kind::Dont,
        _ => { return Err("invalid kind".to_string()) }
    };
    let mut left = 0;
    let mut right = 0;
    if kind == Kind::Mul{
        let splits: Vec<&str> = name_split[1].strip_suffix(")").unwrap().split(",").collect();
        left = splits[0].parse::<i32>().map_err(|e|e.to_string())?;
        right = splits[1].parse::<i32>().map_err(|e|e.to_string())?;
    }
    Ok(Instruction{
        kind: kind,
        left: left,
        right: right
    })
}