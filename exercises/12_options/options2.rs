fn main() {
    let range = 10;

    let mut optional_integers: Vec<Option<i8>> = vec![None];

    for i in 1..=range {
        optional_integers.push(Some(i));
    }

    println!("{:?}", optional_integers);
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // pop values from Vec until the popped value is a None
        // which means that there is nothing left in the vec.
        // If the value the vec pops is a none ex: [None].pop
        // it will actuallu pop Option(None). Thats why the
        // additional if let unwrapping is needed.
        while let Some(val) = optional_integers.pop() {
            // moves the cursor only if the value that got popped
            // was a Some
            if let Some(integer) = val {
                assert_eq!(integer, cursor);
                cursor -= 1;
            }
        }

        // a better way is to use nested pattern matching
        // while let Some(Some(integer)) = optional_integers.pop() {
        //     assert_eq!(integer, cursor);
        //     cursor -= 1;
        // }

        assert_eq!(cursor, 0);
    }
}
