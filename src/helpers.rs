pub fn get_class_concat_with<P: ToString, S: ToString>(prefix_class: P, suffix_class: S) -> String {
    let prefix_class = prefix_class.to_string();

    if !prefix_class.is_empty() {
        format!("{}-{}", prefix_class, suffix_class.to_string())
    } else {
        String::default()
    }
}