use std::{collections::{HashMap,HashSet}, fs::File, io::{BufRead, BufReader}};

struct Order {
    left: i32,
    right:i32,
}

struct Page {
    orders: Vec<Order>,
    updates: Vec<Vec<i32>>,
}

pub fn get_sum_of_mid(path: &str) -> Result<i32,String>{
    let page = read_page(path)?;
    let mut lookup = HashMap::<i32, HashSet<i32>>::new();

    // organize the orders
    for order in page.orders{
        lookup.entry(order.left)
            .or_default()
            .insert(order.right);
    }

    let mut sum = 0;
    // loop over the updates and look for out of order updates
    for update in page.updates{
        // skip over non ordered updates
        if !is_ordered(&update, &lookup){
            continue;
        }
        // find the mid point
        let mid = update.len()/2;
        sum += update[mid];
    }

    Ok(sum)
}

fn is_ordered(update: &Vec<i32>, lookup: &HashMap<i32,HashSet<i32>>)-> bool{
    for index in 0..update.len()-1{
        // look for rule violations where index+1 appears before index in the lookup table
        let left = update[index];
        let right = update[index+1];
        match lookup.get(&right){
            Some(set)=>{
                match set.get(&left){
                    Some(_)=>{
                        // we have a violation
                        return false;
                    }
                    None=>{ continue; }
                }
            }
            None=>{ continue;}
        }
    }
    true
}

fn read_page(path: &str) -> Result<Page,String>{
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
        buf.clear();
        let result = &reader.read_line(&mut buf).map_err(|x|x.to_string())?;
        if *result == 0{
            break;
        }
        if buf.ends_with('\n'){         
            buf.pop();            
            if buf.ends_with('\r'){
                buf.pop();
            }
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
    let mut buf = String::new();
    let mut updates = Vec::new();
    loop{
        buf.clear();
        let result = &reader.read_line(&mut buf).map_err(|x|x.to_string())?;
        if *result == 0{
            break;
        }
        if buf.ends_with('\n'){         
            buf.pop();            
            if buf.ends_with('\r'){
                buf.pop();
            }
        }
        if buf.len() == 0{
            break;
        }
        let splits = buf
            .split(',')
            .map(|x|x.parse::<i32>().unwrap())
            .collect();
        updates.push(splits);
    }
    Ok(updates)
}