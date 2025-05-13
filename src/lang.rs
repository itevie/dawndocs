pub fn kebab_to_english<T: Into<String>>(value: T) -> String {
    value
        .into()
        .split("-")
        .collect::<Vec<&str>>()
        .iter()
        .map(|e| {
            format!(
                "{}{}",
                &e.to_string().chars().nth(0).unwrap().to_uppercase(),
                &e.to_string()[1..]
            )
        })
        .collect::<Vec<String>>()
        .join(" ")
}
