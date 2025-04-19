use anyhow::Result;
use similar::{ChangeTag, TextDiff};
pub struct ContentDiff;

impl ContentDiff {
    pub fn text_diff(old: &str, new: &str) -> String {
        let diff = TextDiff::from_lines(old, new);

        let mut result = String::new();
        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => "=",
            };
            result.push_str(&format!("{}{}", sign, change));
        }
        result
    }

    pub fn html_diff(old: &str, new: &str) -> Result<String> {
        let diff = TextDiff::from_lines(old, new);
        let mut html = String::from("<div class=\"diff>\">\n");

        for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
            html.push_str(&format!("<h3>Change group {}</h3>\n", idx + 1));
            html.push_str("<pre>\n");

            for op in group {
                for change in diff.iter_inline_changes(op) {
                    let (class, sign) = match change.tag() {
                        ChangeTag::Delete => ("dell", "-"),
                        ChangeTag::Insert => ("ins", "+"),
                        ChangeTag::Equal => ("eq", "="),
                    };
                    html.push_str(&format!("<span class=\"{}\">{}", class, sign));

                    for (emphasized, value) in change.iter_strings_lossy() {
                        if emphasized {
                            html.push_str("<strong>");
                        }
                        html.push_str(&html_escape::encode_text(&value));
                        if emphasized {
                            html.push_str("</strong>");
                        }
                    }
                    html.push_str("</strong>");
                }
            }
            html.push_str("</pre>");
        }
        html.push_str("</div>");
        Ok(html)
    }
}
