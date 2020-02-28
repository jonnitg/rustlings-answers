// option1.rs
// This example panics because the second time it calls `pop`, the `vec`
// is empty, so `pop` returns `None`, and `unwrap` panics if it's called
// on `None`. Handle this in a more graceful way than calling `unwrap`!
// Execute `rustlings hint option1` for hints :)

pub fn pop_too_much() -> Option<()> {
    let mut list = vec![3];

    let last = list.pop();
    match last {
        None => None, // Check if none
        Some(thing) => Some(println!("The last item in the list is {:?}", last)), // Check if Some
    };

    let second_to_last = list.pop();
    let result = match second_to_last {
        None => None, // Check if none
        Some(thing) => Some(println!(
            "The second-to-last item in the list is {:?}",
            second_to_last
        )), // Check if Some
    };

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_panic() {
        assert_eq!(pop_too_much(), None);
    }
}
