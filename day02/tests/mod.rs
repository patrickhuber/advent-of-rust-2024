use day02::{calculate_safe_reports, calculate_safe_reports_with_daper};


#[test]
fn day02a_example() -> Result<(), String>{
    let sum = calculate_safe_reports("tests/example.txt")?;
    assert_eq!(sum, 2);
    Ok(())
}

#[test]
fn day02a() -> Result<(), String>{
    let sum = calculate_safe_reports("tests/input.txt")?;
    assert_eq!(sum, 246);
    Ok(())
}


#[test]
fn day02b_example() -> Result<(), String>{
    let sum = calculate_safe_reports_with_daper("tests/example.txt", 1)?;
    assert_eq!(sum, 6);
    Ok(())
}

#[test]
fn day02b() -> Result<(), String>{
    let sum = calculate_safe_reports_with_daper("tests/input.txt",1)?;
    assert_eq!(sum, 246);
    Ok(())
}