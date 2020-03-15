#[cfg(test)]
mod tests {
    #[test]
    fn conditionals() {
        let i = 20;
        if i < 2 {
            assert!(i < 2);
        } else if i > 2 {
            assert!(i > 2);
        } else {
            assert_eq!(i, 2);
        }
    }

    #[test]
    fn more_conditionals() {
        let my_option = Some(10);

        if let Some(unpacked) = my_option {
            assert_eq!(unpacked, 10)
        }

        let mut other_option = Some(2);
        while let Some(unpacked) = other_option {
            other_option = if unpacked > 0 {
                Some(unpacked - 1)
            } else {
                None
            }
        }
        assert_eq!(other_option, None);
    }

    #[test]
    fn loops() {

        let mut i = 42;
        let mut broke = false;
        assert!(!broke);

        loop {
            i -= 1;
            if i < 2 {
                broke = true;
                break;
            } else if i > 2 {
                continue;
            }
        }
        assert!(broke);
    }
}
