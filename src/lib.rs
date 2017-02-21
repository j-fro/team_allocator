
pub fn allocate_teams<'a>(names: &'a Vec<String>) -> Vec<Vec<Vec<String>>> {
    let inner = Vec::new();
    let middle = vec![inner];
    let outer = vec![middle];
    outer
}

#[cfg(test)]
mod test {
    use allocate_teams;

    #[test]
    fn one_iteration_one_member() {
        assert_eq!(vec![vec![vec!["Bob".to_string()]]],
                   allocate_teams(&vec!["Bob".to_string()]));
    }
}