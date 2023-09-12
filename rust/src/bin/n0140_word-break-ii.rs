pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    backtrace(&s, 0, &word_dict, &mut vec![], &mut result);
    result
}
fn backtrace(s: &str, i: usize, word_dict: &Vec<String>, t: &mut Vec<String>, result: &mut Vec<String>) {
    if i > s.len() {
        return;
    }
    if i == s.len() {
        result.push(t.join(" "));
        return;
    }
    for word in word_dict {
        if i + word.len() > s.len() {
            continue;
        }
        if &s[i..i + word.len()] == word {
            t.push(word.clone());
            backtrace(s, i + word.len(), word_dict, t, result);
            t.pop();
        }
    }
}

fn main() {
    let result = word_break(
        "catsanddog".to_string(),
        vec![
            "cat".to_string(),
            "cats".to_string(),
            "and".to_string(),
            "sand".to_string(),
            "dog".to_string(),
        ],
    );
    println!("{result:?}");
}
