use cgl::Builder;

#[cfg(test)]
mod tests {
    use super::*;

    // Example 1: f(x) = x^2 + x + 5
    #[test]
    fn example_1() {
        let mut builder = Builder::new();
        let x = builder.init();
        let x_squared = builder.mul(&x, &x);
        let x_squared_plus_x = builder.add(&x_squared, &x);
        let five = builder.constant(5);
        let _y = builder.add(&x_squared_plus_x, &five);

        let inputs = vec![Some(3)];
        builder.fill_nodes(inputs);
        assert!(builder.check_constraints());
    }

    // Example 2: f(a) = (a+1) / 8
    #[test]
    fn example_2() {
        let mut builder = Builder::new();
        let a = builder.init();
        let one = builder.constant(1);
        let b = builder.add(&a, &one);

        let c = builder.hint(|values| values[0] / 8, vec![b]);
        let eight = builder.constant(8);
        let c_times_8 = builder.mul(&c, &eight);
        builder.assert_equal(b, c_times_8);

        let inputs = vec![Some(7)];
        builder.fill_nodes(inputs);
        assert!(builder.check_constraints());
    }

    // Example 3: f(x) = sqrt(x+7)
    #[test]
    fn example_3() {
        let mut builder = Builder::new();
        let x = builder.init();
        let seven = builder.constant(7);
        let x_plus_7 = builder.add(&x, &seven);

        let sqrt_x_plus_7 = builder.hint(|values| (values[0] as f64).sqrt() as u32, vec![x_plus_7]);
        let computed_sq = builder.mul(&sqrt_x_plus_7, &sqrt_x_plus_7);
        builder.assert_equal(computed_sq, x_plus_7);

        let inputs = vec![Some(9)];
        builder.fill_nodes(inputs);
        assert!(builder.check_constraints());
    }

    // Edge Test 1: Test with no operations, just a constant node
    #[test]
    fn edge_test_constant_only() {
        let mut builder = Builder::new();
        let _five = builder.constant(5);

        let inputs = vec![None; 1]; // No inputs needed for constants
        builder.fill_nodes(inputs);
        assert!(builder.check_constraints());
    }

    // Edge Test 2: Test with an operation where input nodes are both zero
    #[test]
    fn edge_test_zero_inputs() {
        let mut builder = Builder::new();
        let zero_a = builder.constant(0);
        let zero_b = builder.constant(0);
        let sum = builder.add(&zero_a, &zero_b);

        let inputs = vec![None; 2];
        builder.fill_nodes(inputs);
        builder.assert_equal(sum, zero_a);
        assert!(builder.check_constraints());
    }

    // Edge Test 3: Test with hinting a square root of a non-perfect square
    #[test]
    fn edge_test_non_perfect_square() {
        let mut builder = Builder::new();
        let x = builder.constant(10);
        let _sqrt_x = builder.hint(|values| (values[0] as f64).sqrt() as u32, vec![x]);

        let inputs = vec![None; 1];
        builder.fill_nodes(inputs);
        // There's no constraint to check for this non-perfect square hint
    }

    // Edge Test 4: Test with multiple operations leading to the same result
    #[test]
    fn edge_test_multiple_operations() {
        let mut builder = Builder::new();
        let two = builder.constant(2);
        let three = builder.constant(3);
        let six = builder.mul(&two, &three);
        let six_alt = builder.add(&three, &three);
        builder.assert_equal(six, six_alt);

        let inputs = vec![None; 2];
        builder.fill_nodes(inputs);
        assert!(builder.check_constraints());
    }
}
