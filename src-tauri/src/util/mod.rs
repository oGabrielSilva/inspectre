#[cfg(windows)]
pub fn format_cim_date(raw: &str) -> Option<String> {
    if raw.len() < 8 {
        return None;
    }
    let year = raw.get(0..4)?;
    let month = raw.get(4..6)?;
    let day = raw.get(6..8)?;
    Some(format!("{year}-{month}-{day}"))
}
