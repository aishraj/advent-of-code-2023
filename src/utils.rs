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
