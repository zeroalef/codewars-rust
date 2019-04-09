pub fn hor_mirror(s: String) -> String {
    s.split('\n').rev().collect::<Vec<&str>>().join("\n")
}
pub fn vert_mirror(s: String) -> String {
    s.split('\n')
        .map(|s| s.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn oper(fun: fn(String) -> String, s: String) -> String {
    fun(s)
}
