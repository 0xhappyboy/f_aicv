//! Minimal XML helpers for manifest serialization.
pub(crate) fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
/// Unescapes XML entities.
pub(crate) fn unescape_xml(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&amp;", "&")
}
/// Extracts the value of an attribute from an XML element.
pub(crate) fn extract_attr(xml: &str, element: &str, attr: &str) -> Option<String> {
    let open = format!("<{}", element);
    let start = xml.find(&open)?;
    let tag_end = xml[start..].find('>')? + start;
    let tag_str = &xml[start..tag_end];
    let needle = format!("{}=\"", attr);
    let a = tag_str.find(&needle)? + needle.len();
    let b = tag_str[a..].find('"')? + a;
    Some(unescape_xml(&tag_str[a..b]))
}
/// Extracts the inner content of an XML element.
pub(crate) fn extract_inner(xml: &str, element: &str) -> Option<String> {
    let open = format!("<{}", element);
    let close = format!("</{}>", element);
    let start = xml.find(&open)?;
    let after = xml[start..].find('>')? + start + 1;
    let end_rel = xml[after..].find(&close)?;
    Some(xml[after..after + end_rel].to_string())
}
