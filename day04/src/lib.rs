use std::fs::read_to_string;

pub fn count_xmas(path: &str) -> Result<i32, String>{
    let content = read_to_string(path).map_err(|e|e.to_string())?;
    let grid = read_grid(content.lines().collect());
    let mut sum:i32 = 0;
    let search:Vec<char> = "XMAS".chars().collect();
    for r in 0.. grid.len(){
        for c in 0..grid[r].len(){
            if grid[c][r] != search[0]{
                continue;
            }
            sum += in_window(&grid, &search, c, r);
        }
    }
    Ok(sum)
}

pub fn count_masx(path: &str)->Result<i32, String>{
    let content = read_to_string(path).map_err(|x|x.to_string())?;
    let grid = read_grid(content.lines().collect());
    let mut sum:i32 = 0;

    for r in 1..grid.len()-1{
        for c in 1..grid[r].len()-1{
            if grid[c][r] != 'A'{
                continue;
            }
            let left_top = grid[c-1][r-1];
            let right_top = grid[c+1][r-1];
            let left_bottom = grid[c-1][r+1];
            let right_bottom = grid[c+1][r+1];
            match (left_top, right_bottom) {
                ('M', 'S')=>{}
                ('S', 'M')=>{}
                _=>{ continue ; }
            }
            match(left_bottom, right_top){
                ('M', 'S')=>{}
                ('S', 'M')=>{}
                _=> {continue;}
            }
            sum += 1;
        }        
    }   
    Ok(sum)
}

fn in_window(grid: &Vec<Vec<char>>, search: &Vec<char>, col:usize, row: usize)-> i32{
    let col_i32 = i32::try_from(col).unwrap();
    let row_i32 = i32::try_from(row).unwrap();

    let x_directions = vec![-1, 0, 1];
    let y_directions = x_directions.clone();

    let mut count = 0;
    for x_dir in &x_directions{
        for y_dir in &y_directions{
            if match_direction(&grid, &search, col_i32, row_i32, *x_dir, *y_dir){
                count +=1
            }
        }
    }
    count
}

fn match_direction(grid: &Vec<Vec<char>>, search: &Vec<char>, col: i32, row: i32, dir_x: i32, dir_y: i32) -> bool{
    print!("(c:{}, r:{}), (x:{}, y:{}) ->", col, row, dir_x, dir_y);
    if dir_x == 0 && dir_y == 0{
        println!("(0,0)");
        return false;
    }
    let search_len = i32::try_from(search.len()).unwrap();
    let max_x = (search_len-1) * dir_x;
    let max_y = (search_len-1) * dir_y;
    let grid_max_row = i32::try_from(grid.len()).unwrap();
    let grid_max_col = i32::try_from(grid[0].len()).unwrap();
    
    if max_y + row < 0 || max_y + row >=grid_max_row{
        println!("(oob:row)");
        return false;
    }
    if max_x + col < 0 || max_x + col >= grid_max_col{
        println!("(oob:col)");
        return false;
    }    
    
    for search_index in 0..search.len(){
        let search_i32_index = i32::try_from(search_index).unwrap();
        let c_usize = usize::try_from(col + search_i32_index * dir_x).unwrap();
        let r_usize = usize::try_from(row + search_i32_index * dir_y).unwrap();
        let grid_char = grid[c_usize][r_usize];
        let search_char = search[search_index];
        if grid_char != search_char{
            println!("({} != {})", grid_char, search_char);
            return false;
        }else{
            print!("({} == {}) ", grid_char, search_char);
        } 
    }  
    println!("(found)");
    true
}

fn read_grid(lines: Vec<&str>) -> Vec<Vec<char>>{
    let mut columns:Vec<Vec<char>> = Vec::new();    
    for (r,line) in lines.iter().enumerate(){
        for(c, ch) in line.chars().enumerate(){
            if r == 0{
                columns.push(Vec::new());
            }
            columns[c].push(ch);
        }
    }
    return columns;
}
