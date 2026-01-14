pub(crate) fn body_snippet(bytes: &[u8], max_len: usize) -> Option<String> {
    if max_len == 0 || bytes.is_empty() {
        return None;
    }

    let slice = if bytes.len() > max_len {
        &bytes[..max_len]
    } else {
        bytes
    };

    let text = String::from_utf8_lossy(slice).to_string();

    if let Ok(mut value) = serde_json::from_str::<serde_json::Value>(&text) {
        redact_json_value(&mut value);
        let out = serde_json::to_string(&value).unwrap_or_else(|_| "<redacted>".to_owned());
        return Some(out);
    }

    let lowered = text.to_ascii_lowercase();
    let looks_sensitive = [
        "accesskey",
        "secret",
        "token",
        "password",
        "authorization",
        "cookie",
    ]
    .iter()
    .any(|k| lowered.contains(k));

    if looks_sensitive {
        return Some("<redacted>".to_owned());
    }

    Some(text)
}

fn redact_json_value(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            for (k, v) in map.iter_mut() {
                if is_sensitive_key(k) {
                    *v = serde_json::Value::String("<redacted>".to_owned());
                } else {
                    redact_json_value(v);
                }
            }
        }
        serde_json::Value::Array(items) => {
            for v in items {
                redact_json_value(v);
            }
        }
        _ => {}
    }
}

fn is_sensitive_key(key: &str) -> bool {
    let k = key.to_ascii_lowercase();
    matches!(
        k.as_str(),
        "accesskeyid"
            | "access_key_id"
            | "accesskeysecret"
            | "access_key_secret"
            | "secret"
            | "token"
            | "securitytoken"
            | "security_token"
            | "password"
            | "authorization"
            | "cookie"
            | "client_secret"
    )
}
