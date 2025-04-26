use anyhow::Result;
use askama::Template;
use chrono::Local;
use std::path::Path;

use crate::fetcher::CheckResult;

#[derive(Template)]
#[template(path = "report_template.html", escape = "none")]
struct HtmlReportTemplate<'a> {
    prev: &'a CheckResult,
    current: &'a CheckResult,
    diff_content: String,
    generated_time: String,
    duration_change: f64,
    current_duration_ms: String, // ➕ 新增字段
    duration_change_str: String, // ➕ 新增字段
    status_code: u16,
    formatted_timestamp: String,
}

// 生成HTML格式的差异报告
pub async fn generate_html_report(
    prev: &CheckResult,
    current: &CheckResult,
    output_path: &Path,
) -> Result<()> {
    // 计算响应时间变化百分比
    let duration_change = if prev.duration.as_secs_f64() > 0.0 {
        (current.duration.as_secs_f64() - prev.duration.as_secs_f64()) / prev.duration.as_secs_f64()
            * 100.0
    } else {
        0.0
    };

    // 计算字段
    let current_duration_ms = format!("{:.2}ms", current.duration.as_secs_f64() * 1000.0);
    let duration_change_str = format!(
        "{}{:.1}%",
        if duration_change > 0.0 { "+" } else { "" },
        duration_change
    );
    let status_code = current.status.as_u16();
    let formatted_timestamp = current.timestamp.to_rfc2822();
    let formatted_generated_time = Local::now().to_rfc2822();

    // 生成差异内容
    let diff_content = generate_diff_content(prev, current)?;

    // 渲染模板
    let report = HtmlReportTemplate {
        prev,
        current,
        diff_content,
        generated_time: formatted_generated_time,
        duration_change,
        current_duration_ms,
        duration_change_str,
        status_code,
        formatted_timestamp,
    }
    .render()?;

    tokio::fs::write(output_path, report).await?;
    Ok(())
}

// 生成差异内容
fn generate_diff_content(prev: &CheckResult, current: &CheckResult) -> Result<String> {
    use similar::{ChangeTag, TextDiff};

    let diff = TextDiff::from_lines(&prev.content, &current.content);
    let mut html = String::new();

    for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
        html.push_str(&format!(
            "<div class=\"change-group\">\n<h3>Change Group #{}</h3>\n",
            idx + 1
        ));

        for op in group {
            for change in diff.iter_changes(op) {
                let (class, symbol) = match change.tag() {
                    ChangeTag::Delete => ("del", "-"),
                    ChangeTag::Insert => ("ins", "+"),
                    ChangeTag::Equal => ("eq", " "),
                };

                html.push_str(&format!(
                    "<div class=\"{}\"><span class=\"symbol\">{}</span>{}",
                    class,
                    symbol,
                    html_escape::encode_text(&change.to_string_lossy())
                ));
            }
        }
        html.push_str("</div>\n");
    }
    Ok(html)
}
