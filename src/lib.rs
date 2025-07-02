/// Email's suffix Checker
///
/// # Examples
///
/// ## Execute Function of Script File
///
/// ```rust
/// let white = vec!["@062e.com"];
/// let checker = EmailChecker::form_vec(white);
/// println!(checker.check("adad@062e.com"));
/// ```
pub struct EmailChecker {
    pub available_suffix: Vec<String>,
}

impl EmailChecker {
    /// create a check with some available suffix
    pub fn form_vec(available_suffix: &[impl AsRef<str>]) -> Self {
        Self {
            available_suffix: available_suffix
                .iter()
                .map(|s| s.as_ref().to_string())
                .collect(),
        }
    }

    /// Check if the suffix of a email's suffix matches the available_suffix and return a bool
    pub fn check(&self, email: impl Into<String>) -> bool {
        let email = email.into();
        if let Some(index) = email.find('@') {
            let suffix = &email[(index + 1)..].to_string();
            self.available_suffix.contains(suffix)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let white = vec!["062e.com"];
        let checker = EmailChecker::form_vec(&white);

        assert_eq!(true, checker.check("adad@062e.com"));
        assert_eq!(false, checker.check("adad@062ess.com"))
    }
}
