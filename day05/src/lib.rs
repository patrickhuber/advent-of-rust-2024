use std::{collections::{HashMap,HashSet}, fs::File, io::{BufRead, BufReader}};

struct Order {
    left: i32,
    right:i32,
}

struct Page {
    orders: Vec<Order>,
    updates: Vec<Vec<i32>>,
}

pub fn get_sum_of_mid_after_reorder(path: &str) ->Result<i32, String>{
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
        if is_ordered(&update, &lookup){
            continue;
        }
        let mut owned = update.to_vec();
        sort(&mut owned,  &lookup);

        // find the mid point
        let mid = owned.len()/2;
        sum += owned[mid];
    }
    Ok(sum)
}

fn sort(update: &mut Vec<i32>, lookup: &HashMap<i32, HashSet<i32>>){
    
    // arrays of 0 or 1 are already sorted
    if update.len() <=1{
        return
    }

    // track an index in the array, backtrack if a swap occurs
    let mut index = 0;
    while index < update.len()-1{
        let left = update[index];
        let right = update[index+1];
        
        if in_order(left, right, lookup){
            index+=1;
            continue;
        }
        
        swap(update, index, index+1);
        if index == 0{
            continue;
        }
        
        index -= 1;        
    }
}

fn swap(update: &mut Vec<i32>, left: usize, right: usize){
    let temp = update[left];
    update[left] = update[right];
    update[right] = temp;    
}

fn in_order(left: i32, right: i32, lookup: &HashMap<i32, HashSet<i32>>) -> bool{    
    let right_lookup = lookup.get(&right);
    if right_lookup.is_none(){
        return true;
    }
    return right_lookup.unwrap().get(&left).is_none();
}

pub fn get_sum_of_mid_correctly_ordered(path: &str) -> Result<i32,String>{
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
        if !in_order(left, right, lookup){
            return false
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