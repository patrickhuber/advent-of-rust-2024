use day02::calcualte_safe_reports;


#[test]
fn day02a_example() -> Result<(), String>{
    let sum = calcualte_safe_reports("tests/example.txt")?;
    assert_eq!(sum, 2);
    Ok(())
}