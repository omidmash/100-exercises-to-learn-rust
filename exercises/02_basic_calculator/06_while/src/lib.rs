// Rewrite the factorial function using a `while` loop.
pub fn factorial(mut n: u32) -> u32 {
    // The `todo!()` macro is a placeholder that the compiler
    // interprets as "I'll get back to this later", thus
    // suppressing type errors.
    // It panics at runtime.
    let mut fact = 1;
    while n > 1 {
        fact *= n;
        n -= 1;
    }
    fact
}

// 1. facto 0, n 5, facto 20, n 4
// 2 facto 20 + 12 = 32, n 3
// 3. facto 32 + 6 +

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
