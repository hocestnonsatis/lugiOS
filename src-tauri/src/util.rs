//! Small shared helpers.

pub fn sanitize_app_id(id: &str) -> Result<String, crate::error::LugosError> {
    let s: String = id
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
        .collect();
    if s.is_empty() {
        return Err(crate::error::LugosError::Msg(
            "app id contains no valid characters".into(),
        ));
    }
    Ok(s)
}
