use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub trait Domainable: Hash + Eq {}
impl<T: Hash + Eq> Domainable for T {}

#[derive(PartialEq, Eq)]
pub struct Variable<T: Domainable> {
    domain: HashSet<T>,
}

impl<T: Domainable> Hash for &Variable<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(*self, state);
    }
}

impl<T: Domainable> Variable<T> {
    fn new(domain: HashSet<T>) -> Variable<T> {
        Variable { domain: domain }
    }
}

pub struct BinaryConstraint<'a, T: Domainable> {
    var1: &'a Variable<T>,
    var1_value: T,
    var2: &'a Variable<T>,
    var2_value: T,
}

impl<'a, T: Domainable> BinaryConstraint<'a, T> {
    fn new(
        var1: &'a Variable<T>,
        var1_value: T,
        var2: &'a Variable<T>,
        var2_value: T,
    ) -> BinaryConstraint<'a, T> {
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

pub struct Constraints<'a, T: Domainable> {
    constraints: HashSet<&'a BinaryConstraint<'a, T>>,
}

impl<'a, T: Domainable> Constraints<'a, T> {
    fn new(constraints: HashSet<&'a BinaryConstraint<'a, T>>) -> Constraints<'a, T> {
        Constraints {
            constraints: constraints,
        }
    }
}

pub struct State<'a, T: Domainable> {
    assignments: HashMap<&'a Variable<T>, Option<T>>,
}

impl<'a, T: Domainable> State<'a, T> {
    fn new(variables: HashSet<&'a Variable<T>>) -> State<'a, T> {
        let mut assignments = HashMap::new();
        for var in &variables {
            assignments.insert(*var, None);
        }
        State {
            assignments: assignments,
        }
    }

    fn is_consistent(&self, constraints: &Constraints<'a, T>) -> bool {
        for constraint in constraints.constraints.iter() {
            match self.assignments.get(&constraint.var1) {
                None => panic!(),
                Some(Some(v)) => {
                    if *v != constraint.var1_value {
                        return false;
                    }
                }
                _ => (),
            }
            match self.assignments.get(&constraint.var2) {
                None => panic!(),
                Some(Some(v)) => {
                    if *v != constraint.var2_value {
                        return false;
                    }
                }
                _ => (),
            }
        }
        true
    }

    fn is_complete(&self) -> bool {
        for value in self.assignments.values() {
            if *value == None {
                return false;
            }
        }
        true
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
        let binary_constraint = BinaryConstraint::new(&var1, 3, &var2, 4);

        assert_eq!(binary_constraint.var1_value, 3);
        assert_eq!(binary_constraint.var2_value, 4);
        assert!(binary_constraint.var1.domain.contains(&3));
        assert!(binary_constraint.var2.domain.contains(&4));
    }

    #[test]
    #[should_panic]
    fn new_binary_constraint_fail1() {
        let var1 = Variable::new(HashSet::from([1, 2, 3]));
        let var2 = Variable::new(HashSet::from([4, 5, 6]));
        let binary_constraint = BinaryConstraint::new(&var1, 3, &var2, 3);
    }

    #[test]
    #[should_panic]
    fn new_binary_constraint_fail2() {
        let var1 = Variable::new(HashSet::from([1, 2, 3]));
        let var2 = Variable::new(HashSet::from([4, 5, 6]));
        let binary_constraint = BinaryConstraint::new(&var1, 4, &var2, 4);
    }
}
