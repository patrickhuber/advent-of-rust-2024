use day04::{count_masx, count_xmas};

#[test]
fn day04a_example() -> Result<(), String>{
    let count = count_xmas("tests/example.txt")?;
    assert_eq!(count, 18);
    Ok(())
}

#[test]
fn day04a() -> Result<(), String>{
    let count = count_xmas("tests/input.txt")?;
    assert_eq!(count, 2521);
    Ok(())
}


#[test]
fn day04b_example() -> Result<(), String>{
    let count = count_masx("tests/example.txt")?;
    assert_eq!(count, 9);
    Ok(())
}

#[test]
fn day04b() -> Result<(), String>{
    let count = count_masx("tests/input.txt")?;
    assert_eq!(count, 1912);
    Ok(())
}