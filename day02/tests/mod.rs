use day02::calculate_safe_reports;


#[test]
fn day02a_example() -> Result<(), String>{
    let sum = calculate_safe_reports("tests/example.txt", false)?;
    assert_eq!(sum, 2);
    Ok(())
}

#[test]
fn day02a() -> Result<(), String>{
    let sum = calculate_safe_reports("tests/input.txt", false)?;
    assert_eq!(sum, 246);
    Ok(())
}

#[test]
fn day02b_example() -> Result<(), String>{
    let sum = calculate_safe_reports("tests/example.txt", true)?;
    assert_eq!(sum, 4);
    Ok(())
}

#[test]
fn day02b() -> Result<(), String>{
    let sum = calculate_safe_reports("tests/input.txt", true)?;
    assert_eq!(sum, 318);
    Ok(())
}