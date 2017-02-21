
pub fn allocate_teams<'a>(names: &'a Vec<&str>, group_max: usize) -> Vec<Vec<Vec<&'a str>>> {
    let mut inner = Vec::new();
    for name in names {
        inner.push(*name);
    }
    let middle = vec![inner];
    let outer = vec![middle];
    outer
}

#[cfg(test)]
mod test {
    use allocate_teams;

    #[test]
    fn one_iteration_max_one_one_member() {
        assert_eq!(vec![vec![vec!["Alice"]]], allocate_teams(&vec!["Alice"], 1));
    }

    #[test]
    fn one_iteration_max_one_two_members() {
        assert_eq!(vec![vec![vec!["Alice"], vec!["Bob"]]],
                   allocate_teams(&vec!["Alice", "Bob"], 1));
    }
}