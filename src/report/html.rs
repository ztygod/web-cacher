use anyhow::Result;
use askama::Template;
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

    // 生成差异内容
    let diff_content = 
}

// 生成差异内容
fn generate_diff_content(prev: &CheckResult,current: &CheckResult) -> Result<String>{
    use similar::{TextDiff,ChangeTag};

    let 
}
