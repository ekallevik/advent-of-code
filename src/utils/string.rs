
pub fn is_numeric(s: &str) -> bool {
    s.trim().chars().all(|c| c.is_numeric())
}