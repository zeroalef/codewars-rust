#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct KeyValue {
    key: u32,
    value: String,
}

impl KeyValue {
    pub fn new(key: u32, value: String) -> Self {
        KeyValue { key, value }
    }
}

pub fn order(sentence: &str) -> String {
    if sentence.is_empty() {
        sentence.to_string()
    } else {
        let foo = |s: &str| -> Vec<KeyValue> {
            s.split(' ')
                .map(|s| {
                    KeyValue::new(
                        s.chars().fold(0, |sum, ch| match ch.to_digit(10) {
                            Some(i) => sum + i,
                            None => sum,
                        }),
                        s.to_string(),
                    )
                })
                .collect::<Vec<KeyValue>>()
        };
        let mut tmp = foo(sentence);
        tmp.sort_by(|a, b| a.key.cmp(&b.key));
        tmp.iter()
            .map(|item| item.value.clone())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
