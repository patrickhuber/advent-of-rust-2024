use day05::{get_sum_of_mid_correctly_ordered,get_sum_of_mid_after_reorder};

#[test]
fn day05a_example() -> Result<(), String>{
    let sum = get_sum_of_mid_correctly_ordered("tests/example.txt")?;
    assert_eq!(sum, 143);
    Ok(())
}

#[test]
fn day05a()-> Result<(), String>{
    let sum = get_sum_of_mid_correctly_ordered("tests/input.txt")?;
    assert_eq!(sum, 5275);
    Ok(())
}

#[test]
fn day05b_example() -> Result<(), String>{
    let sum = get_sum_of_mid_after_reorder("tests/example.txt")?;
    assert_eq!(sum, 123);
    Ok(())
}

#[test]
fn day05b()-> Result<(), String>{
    let sum = get_sum_of_mid_after_reorder("tests/input.txt")?;
    assert_eq!(sum, 6191);
    Ok(())
}