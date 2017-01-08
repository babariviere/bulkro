
/// Formatter is a class to format a set of string with number.
///
/// # Example
/// ```
/// use Formatter;
/// let mut formatter = Formatter::new("hello###", vec!["yo", "yey"]);
/// let formatted = Formatter.iter().collect();
/// assert_eq!(formatted, vec!["hello001", "hello002"]);
/// ```
pub struct Formatter {
    format_types: Vec<FormatType>,
    to_format: Vec<String>,
    idx: usize,
}

impl Formatter {
    pub fn new<S: AsRef<str>>(format: S, to_format: &Vec<&str>) -> Formatter {
        let format_types = Formatter::build_format_types(&format);
        Formatter {
            format_types: format_types,
            to_format: to_format.iter().map(|s| s.to_string()).collect(),
            idx: 0,
        }
    }

    /// Build format types for Iterator.
    fn build_format_types<S: AsRef<str>>(format: S) -> Vec<FormatType> {
        let format = format.as_ref();
        let mut ft = Vec::new();
        let mut current: Option<FormatType> = None;
        for c in format.chars() {
            match c {
                '#' => {
                    match current {
                        Some(FormatType::Index(z)) => current = Some(FormatType::Index(z + 1)),
                        Some(current_c) => {
                            ft.push(current_c);
                            current = Some(FormatType::Index(1));
                        }
                        None => current = Some(FormatType::Index(1)),
                    }
                }
                c => {
                    match current {
                        Some(FormatType::Text(s)) => {
                            current = Some(FormatType::Text(format!("{}{}", s, c)))
                        }
                        Some(current_c) => {
                            ft.push(current_c);
                            current = Some(FormatType::Text(c.to_string()));
                        }
                        None => current = Some(FormatType::Text(c.to_string())),
                    }
                }
            }
        }
        ft.push(current.unwrap_or(FormatType::Text("".to_string())));
        ft
    }

    fn add_zeros(num: usize, num_zeros: usize) -> String {
        let mut result = num.to_string();
        while result.len() < num_zeros {
            result.insert(0, '0');
        }
        result
    }
}

impl Iterator for Formatter {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.to_format.len() {
            return None;
        }
        let mut result = String::new();
        for ft in &self.format_types {
            match ft {
                &FormatType::Text(ref s) => result.push_str(s),
                &FormatType::Index(z) => result.push_str(&Formatter::add_zeros(self.idx, z)),
            }
        }
        self.idx += 1;
        Some(result)
    }
}

/// Provide format type to add to string result.
#[derive(Clone, Debug)]
enum FormatType {
    Text(String),
    Index(usize),
}
