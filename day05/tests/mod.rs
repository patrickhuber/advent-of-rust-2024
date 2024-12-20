use day05::get_sum_of_mid;

#[test]
fn day05a_example() -> Result<(), String>{
    let sum = get_sum_of_mid("tests/example.txt")?;
    assert_eq!(sum, 143);
    Ok(())
}

#[test]
fn day05a()-> Result<(), String>{
    let sum = get_sum_of_mid("tests/input.txt")?;
    assert_eq!(sum, 143);
    Ok(())
}