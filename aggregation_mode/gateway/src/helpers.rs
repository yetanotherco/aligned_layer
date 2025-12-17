use std::time::{SystemTime, UNIX_EPOCH};

pub(super) fn format_merkle_path(bytes: &[u8]) -> Result<Vec<String>, String> {
    if bytes.is_empty() {
        return Ok(vec![]);
    }

    if bytes.len() % 32 != 0 {
        return Err("merkle path length is not a multiple of 32 bytes".into());
    }

    Ok(bytes
        .chunks(32)
        .map(|chunk| format!("0x{}", hex::encode(chunk)))
        .collect())
}

pub(crate) fn get_time_left_day_formatted() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Error al obtener el tiempo");

    let seconds_remaining = 86400 - (now.as_secs() % 86400);

    let hours = seconds_remaining / 3600;
    let minutes = (seconds_remaining % 3600) / 60;
    let seconds = seconds_remaining % 60;

    format!("{hours}:{minutes}:{seconds} UTC")
}
