pub(super) fn js_number(value: f64) -> String {
    if value == 0.0 {
        return "0".to_string();
    }
    if value.is_nan() {
        return "NaN".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 { "Infinity" } else { "-Infinity" }.to_string();
    }

    let formatted = format!("{:e}", value.abs());
    let (mantissa, exponent) = formatted
        .split_once('e')
        .unwrap_or((formatted.as_str(), "0"));
    let digits = mantissa.replacen('.', "", 1);
    let point = exponent.parse::<i64>().unwrap_or(0) + 1;
    let count = digits.len() as i64;

    let text = if count <= point && point <= 21 {
        format!("{digits}{}", "0".repeat((point - count) as usize))
    } else if 0 < point && point <= 21 {
        format!(
            "{}.{}",
            &digits[..point as usize],
            &digits[point as usize..]
        )
    } else if -6 < point && point <= 0 {
        format!("0.{}{digits}", "0".repeat((-point) as usize))
    } else {
        let power = point - 1;
        let sign = if power < 0 { "-" } else { "+" };
        if count == 1 {
            format!("{digits}e{sign}{}", power.abs())
        } else {
            format!("{}.{}e{sign}{}", &digits[..1], &digits[1..], power.abs())
        }
    };

    if value < 0.0 {
        format!("-{text}")
    } else {
        text
    }
}
