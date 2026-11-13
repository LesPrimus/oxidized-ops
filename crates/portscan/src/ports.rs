use std::ops::RangeInclusive;

pub fn parse_ports(s: &str) -> Result<RangeInclusive<u16>, String> {
    let s = s.trim();
    let (start, end) = s.split_once('-').unwrap_or((s, s));

    let parse = |p: &str| -> Result<u16, String> {
        let n: u16 = p.trim().parse().map_err(|_| format!("Invalid port {p}"))?;
        if n == 0 {
            return Err("Invalid port 0".into());
        }
        Ok(n)
    };

    let (start, end) = (parse(start)?, parse(end)?);

    if start > end {
        return Err("Invalid range".into());
    }
    Ok(start..=end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_port() {
        assert_eq!(parse_ports("22"), Ok(22..=22));
    }

    #[test]
    fn range() {
        assert_eq!(parse_ports("20-23"), Ok(20..=23));
    }

    #[test]
    fn whitespace_is_ignored() {
        assert_eq!(parse_ports(" 80 - 81 "), Ok(80..=81));
    }

    #[test]
    fn rejects_bad_input() {
        for bad in ["0", "abc", "70000", "90-80", "1-", "-5", ""] {
            assert!(parse_ports(bad).is_err(), "`{bad}` should be rejected");
        }
    }
}
