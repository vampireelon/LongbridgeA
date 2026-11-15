//! Pure parser for Tencent smartbox (`smartbox.gtimg.cn/s3/`) search responses.

use longbridgea_core::SearchHit;

use crate::error::QuoteError;

/// Parse smartbox body into search hits.
///
/// Typical body (GBK or UTF-8):
/// ```text
/// v_hint="sh~600519~贵州茅台~gzmt~GP-A^sz~000001~平安银行~payh~GP-A";
/// ```
/// Optional leading `v_cate_hint="...";` is ignored.
pub fn parse_search_body(body: &str) -> Result<Vec<SearchHit>, QuoteError> {
    let body = body.trim();
    if body.is_empty() {
        return Err(QuoteError::EmptyResponse);
    }

    let payload = extract_v_hint(body).ok_or_else(|| {
        QuoteError::BadFormat(body.chars().take(120).collect())
    })?;

    if payload.is_empty() {
        return Ok(Vec::new());
    }

    let mut hits = Vec::new();
    for row in payload.split('^') {
        let row = row.trim();
        if row.is_empty() {
            continue;
        }
        if let Some(hit) = parse_search_row(row) {
            hits.push(hit);
        }
    }

    Ok(hits)
}

fn extract_v_hint(body: &str) -> Option<&str> {
    // Prefer the v_hint= assignment; ignore v_cate_hint.
    for part in body.split(';') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let lower = part.to_ascii_lowercase();
        // Match v_hint= but not v_cate_hint=
        if lower.starts_with("v_hint=") {
            let after = part.split_once('=')?.1.trim();
            let payload = after.trim_matches('"').trim_matches('\'');
            return Some(payload);
        }
    }

    // Fallback: whole body is the quoted payload (tests / partial dumps).
    let t = body.trim().trim_matches('"').trim_matches('\'');
    if t.contains('~') {
        return Some(t);
    }
    None
}

fn parse_search_row(row: &str) -> Option<SearchHit> {
    let fields: Vec<&str> = row.split('~').collect();
    // market ~ code ~ name ~ pinyin ~ kind  (kind optional on some rows)
    if fields.len() < 3 {
        return None;
    }
    let market_tag = fields[0].trim().to_string();
    let code = fields[1].trim().to_string();
    if market_tag.is_empty() || code.is_empty() {
        return None;
    }
    let name = fields.get(2).map(|s| s.trim().to_string()).unwrap_or_default();
    let pinyin = fields.get(3).map(|s| s.trim().to_string()).unwrap_or_default();
    let kind = fields.get(4).map(|s| s.trim().to_string()).unwrap_or_default();

    Some(SearchHit {
        market_tag,
        code,
        name,
        pinyin,
        kind,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_moutai_single() {
        let body = r#"v_hint=\"sh~600519~贵州茅台~gzmt~GP-A\";"#;
        let hits = parse_search_body(body).unwrap();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].market_tag, "sh");
        assert_eq!(hits[0].code, "600519");
        assert_eq!(hits[0].name, "贵州茅台");
        assert_eq!(hits[0].pinyin, "gzmt");
        assert_eq!(hits[0].kind, "GP-A");
        assert!(hits[0].is_a_share());
        assert_eq!(hits[0].to_symbol().unwrap().tencent_wire(), "sh600519");
    }

    #[test]
    fn parse_multi_with_hk_us() {
        let body = r#"v_hint=\"sh~601318~中国平安~zgpa~GP-A^hk~02318~中国平安~zgpa~GP^us~pngay.ps~PingAn~pingan~GP^sz~000001~平安银行~payh~GP-A\";"#;
        let hits = parse_search_body(body).unwrap();
        assert_eq!(hits.len(), 4);
        assert_eq!(hits[0].code, "601318");
        assert!(hits[0].is_a_share());
        assert!(!hits[1].is_a_share()); // hk
        assert!(!hits[2].is_a_share()); // us
        assert_eq!(hits[3].code, "000001");
        assert_eq!(hits[3].market_tag, "sz");
    }

    #[test]
    fn ignore_cate_hint() {
        let body = r#"v_cate_hint=\"bd010300~煤炭~MT\";
v_hint=\"sh~600519~贵州茅台~gzmt~GP-A^sh~000820~煤炭指数~mtzs~ZS\";"#;
        let hits = parse_search_body(body).unwrap();
        assert_eq!(hits.len(), 2);
        assert_eq!(hits[0].name, "贵州茅台");
        assert_eq!(hits[1].kind, "ZS");
    }

    #[test]
    fn empty_hint_ok() {
        let body = r#"v_hint=\"\";"#;
        let hits = parse_search_body(body).unwrap();
        assert!(hits.is_empty());
    }

    #[test]
    fn raw_payload_without_prefix() {
        let body = "sh~600519~贵州茅台~gzmt~GP-A^sz~000001~平安银行~payh~GP-A";
        let hits = parse_search_body(body).unwrap();
        assert_eq!(hits.len(), 2);
    }
}
