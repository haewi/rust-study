struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count:0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn calling_next_directly() {
        let mut count = Counter::new();

        assert_eq!(count.next(), Some(1));
        assert_eq!(count.next(), Some(2));
        assert_eq!(count.next(), Some(3));
        assert_eq!(count.next(), Some(4));
        assert_eq!(count.next(), Some(5));
        assert_eq!(count.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1)) // pair with another counter that skiped the first value
            .map(|(a, b)| a*b) // multiply each pair together (1*2, 2*3, 3*4, 4*5)
            .filter(|x| x % 3 == 0) // keep results that are divisible by 3 (6, 12)
            .sum(); // add them up 6+12
        assert_eq!(18, sum);
    }
}
