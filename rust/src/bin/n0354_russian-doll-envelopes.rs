use std::cmp::Reverse;

pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
    let mut envelopes = envelopes;
    envelopes.sort_unstable_by_key(|k| (k[0], Reverse(k[1])));
    let mut d = vec![envelopes[0][1]];
    for num in envelopes.into_iter().map(|x| x[1]).skip(1) {
        if num > *d.last().unwrap() {
            d.push(num);
        } else if let Err(index) = d.binary_search(&num) {
            d[index] = num;
        }
    }
    d.len() as i32
}


fn main() {
    let count = max_envelopes(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]]);
    // let count = max_envelopes(vec![vec![1, 1]]);
    println!("{count}");
}
