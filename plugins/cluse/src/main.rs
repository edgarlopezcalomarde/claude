use serde_json::Value;
use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};

const LABEL: &str = "154;154;150";
const WHITE: &str = "244;243;238";
const SEP: &str = "90;90;86";
const EMPTY: &str = "58;58;55";

fn paint(rgb: &str, s: &str) -> String {
    format!("\x1b[38;2;{rgb}m{s}\x1b[0m")
}

// Verde bajo 50 %, ámbar desde 50 %, rojo desde 80 %.
fn level_color(p: f64) -> &'static str {
    if p >= 80.0 {
        "255;123;114"
    } else if p >= 50.0 {
        "255;180;84"
    } else {
        "111;214;139"
    }
}

fn bar(p: f64) -> String {
    let filled = if p > 0.0 { ((p / 10.0).round() as usize).clamp(1, 10) } else { 0 };
    paint(level_color(p), &"█".repeat(filled)) + &paint(EMPTY, &"█".repeat(10 - filled))
}

fn window(label: &str, p: f64) -> String {
    let pct = paint(WHITE, &format!("\x1b[1m{:>4}", format!("{p:.0}%")));
    format!("{} {} {}", paint(LABEL, label), bar(p), pct)
}

fn reset_in(v: &Value) -> Option<String> {
    let at = v["rate_limits"]["five_hour"]["resets_at"].as_u64()?;
    let now = SystemTime::now().duration_since(UNIX_EPOCH).ok()?.as_secs();
    let mins = at.saturating_sub(now) / 60;
    Some(format!("{}h{:02}m", mins / 60, mins % 60))
}

fn render(v: &Value) -> String {
    let Some(five) = v["rate_limits"]["five_hour"]["used_percentage"].as_f64() else {
        return "uso n/d".into();
    };
    let mut out = window("5h", five);
    if let Some(r) = reset_in(v) {
        out += &format!(" {}", paint(LABEL, &format!("↻ {r}")));
    }
    if let Some(week) = v["rate_limits"]["seven_day"]["used_percentage"].as_f64() {
        out += &format!(" {} {}", paint(SEP, "│"), window("7d", week));
    }
    out
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).ok();
    let v: Value = serde_json::from_str(&input).unwrap_or(Value::Null);
    println!("{}", render(&v));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plain(s: &str) -> String {
        let mut out = String::new();
        let mut esc = false;
        for c in s.chars() {
            match (esc, c) {
                (_, '\x1b') => esc = true,
                (true, 'm') => esc = false,
                (false, c) => out.push(c),
                _ => {}
            }
        }
        out
    }

    #[test]
    fn shows_both_windows() {
        let v: Value = serde_json::from_str(
            r#"{"rate_limits":{"five_hour":{"used_percentage":68},"seven_day":{"used_percentage":41}}}"#,
        )
        .unwrap();
        assert_eq!(plain(&render(&v)), "5h ██████████  68% │ 7d ██████████  41%");
    }

    #[test]
    fn small_usage_keeps_one_cell() {
        assert!(bar(3.0).contains(&paint(level_color(3.0), "█")));
    }

    #[test]
    fn colors_change_at_thresholds() {
        assert_ne!(level_color(49.0), level_color(50.0));
        assert_ne!(level_color(79.0), level_color(80.0));
    }

    #[test]
    fn missing_data_does_not_panic() {
        assert_eq!(render(&Value::Null), "uso n/d");
    }
}
