use day04::count_xmas;

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