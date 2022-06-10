use std::collections::HashSet;
use std::hash::Hash;

pub struct Variable<T> {
    domain: HashSet<T>,
}

impl<T> Variable<T> {
    fn new(domain: HashSet<T>) -> Variable<T> {
        Variable { domain: domain }
    }
}

pub struct BinaryConstraint<'a, T, U> {
    var1: &'a Variable<T>,
    var1_value: T,
    var2: &'a Variable<U>,
    var2_value: U,
}

impl<'a, T, U> BinaryConstraint<'a, T, U>
where
    T: Eq + Hash,
    U: Eq + Hash,
{
    fn new(
        var1: &'a Variable<T>,
        var1_value: T,
        var2: &'a Variable<U>,
        var2_value: U,
    ) -> BinaryConstraint<'a, T, U> {
        if var1.domain.contains(&var1_value) && var2.domain.contains(&var2_value) {
            BinaryConstraint {
                var1: var1,
                var1_value: var1_value,
                var2: var2,
                var2_value: var2_value,
            }
        } else {
            // variable value not in domain, panic!
            panic!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_variable_domain() {
        let variable = Variable::new(HashSet::from([1, 2, 3, 4]));

        for i in 1..4 {
            assert!(variable.domain.contains(&i));
        }
    }

    #[test]
    fn new_binary_constraint_success() {
        let var1 = Variable::new(HashSet::from([1, 2, 3]));
        let var2 = Variable::new(HashSet::from([4, 5, 6]));
        let binaryConstraint = BinaryConstraint::new(&var1, 3, &var2, 4);

        assert_eq!(binaryConstraint.var1_value, 3);
        assert_eq!(binaryConstraint.var2_value, 4);
        assert!(binaryConstraint.var1.domain.contains(&3));
        assert!(binaryConstraint.var2.domain.contains(&4));
    }

    #[test]
    #[should_panic]
    fn new_binary_constraint_fail1() {
        let var1 = Variable::new(HashSet::from([1, 2, 3]));
        let var2 = Variable::new(HashSet::from([4, 5, 6]));
        let binaryConstraint = BinaryConstraint::new(&var1, 3, &var2, 3);
    }

    #[test]
    #[should_panic]
    fn new_binary_constraint_fail2() {
        let var1 = Variable::new(HashSet::from([1, 2, 3]));
        let var2 = Variable::new(HashSet::from([4, 5, 6]));
        let binaryConstraint = BinaryConstraint::new(&var1, 4, &var2, 4);
    }
}
