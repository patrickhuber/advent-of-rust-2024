use std::{fs::File, io::{BufRead, BufReader}};

struct Order {
    left: i32,
    right:i32,
}

struct Page {
    orders: Vec<Order>,
    updates: Vec<Vec<i32>>,
}

fn read_file(path: &str) -> Result<Page,String>{
    let file = File::open(path).map_err(|e|e.to_string())?;
    let mut reader = BufReader::new(file);
    let orders = read_orders(&mut reader)?;
    let updates = read_updates(&mut reader)?;
    Ok(Page{
        orders: orders,
        updates: updates,
    })
}

fn read_orders(reader: &mut BufReader<File>)-> Result<Vec<Order>, String>{
    let mut buf = String::new();
    let mut orders = Vec::new();
    loop{ 
        let result = &reader.read_line(&mut buf).map_err(|x|x.to_string())?;
        if *result == 0{
            break;
        }
        if buf.len() == 0{
            break;
        }
        let splits: Vec<&str> = buf.split('|').collect();
        let order = Order{
            left: splits[0].parse::<i32>().map_err(|e|e.to_string())?,
            right: splits[1].parse::<i32>().map_err(|e|e.to_string())?,
        };
        orders.push(order);
    }
    Ok(orders)
}

fn read_updates(reader: &mut BufReader<File>) -> Result<Vec<Vec<i32>>, String>{
    Ok(Vec::new())
}