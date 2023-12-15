pub fn transpose<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut result = Vec::new();
    for i in 0..v[0].len() {
        let mut row = Vec::new();
        for j in 0..v.len() {
            row.push(v[j][i].clone());
        }
        result.push(row);
    }
    result
}

pub fn reverse_cols<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    v.iter()
        .map(|row| row.iter().rev().cloned().collect())
        .collect()
}

pub fn reverse_rows<T: Clone>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    v.iter().rev().cloned().collect()
}

#[cfg(test)]
mod test {

    #[test]
    fn verify_reverse_cols_is_commute() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(
            super::reverse_cols(super::reverse_cols(input.clone())),
            input
        );
    }

    #[test]
    fn verify_reverse_rows_is_commute() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(
            super::reverse_rows(super::reverse_rows(input.clone())),
            input
        );
    }

    #[test]
    fn simple_transpose_char_vec_vec() {
        let input = vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']];
        let expected = vec![vec!['a', 'd'], vec!['b', 'e'], vec!['c', 'f']];
        assert_eq!(super::transpose(input), expected);
    }

    #[test]
    fn simple_test_reverse_cols_char_vec_vec() {
        let input = vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']];
        let expected = vec![vec!['c', 'b', 'a'], vec!['f', 'e', 'd']];
        assert_eq!(super::reverse_cols(input), expected);
    }

    #[test]
    fn test_simple_reverse_rows() {
        let input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![7, 8, 9], vec![4, 5, 6], vec![1, 2, 3]];
        assert_eq!(super::reverse_rows(input), expected);
    }
}
