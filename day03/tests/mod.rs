use day03::run_program;

#[test]
fn day03a_example() -> Result<(), String>{
    let sum = run_program("tests/example_03a.txt", false)?;
    assert_eq!(sum, 161);
    Ok(())
}

#[test]
fn day03a() -> Result<(), String>{
    let sum = run_program("tests/input.txt", false)?;
    assert_eq!(sum, 188116424);
    Ok(())
}

#[test]
fn day03b_example() -> Result<(), String>{
    let sum = run_program("tests/example_03b.txt", true)?;
    assert_eq!(sum, 48);
    Ok(())
}

#[test]
fn day03b() -> Result<(), String> {
    let sum = run_program("tests/input.txt", true)?;
    assert_eq!(sum, 188116424);
    Ok(()) 
}