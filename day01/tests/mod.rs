use day01::{calculate_diff, calculate_similarity};

#[test]
fn day01a_example() -> Result<(),String>{
    let actual = calculate_diff("tests/example.txt")?;
    assert_eq!(actual, 11 );
    Ok(())
}

#[test]
fn day01a() -> Result<(), String>{    
    let actual = calculate_diff("tests/input.txt")?;
    assert_eq!(actual, 3714264 );
    Ok(())
}

#[test]
fn day01b_example()->Result<(), String>{
    let actual = calculate_similarity("tests/example.txt")?;
    assert_eq!(actual, 31);
    Ok(())
}

#[test]
fn day01b()->Result<(), String>{
    let actual = calculate_similarity("tests/input.txt")?;
    assert_eq!(actual, 18805872);
    Ok(())
}