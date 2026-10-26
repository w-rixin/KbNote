//! 旧 AI 厂商配置规范化 —— **仅供数据库迁移 v61 → v62 使用**。
//!
//! AI 功能本身已随删减移除（services/ai.rs 等全删）；但数据库迁移链必须完整 ——
//! 老库升级时会逐版本跑到这条迁移，缺了函数整条链就断。
//! 新库上 `ai_models` 表为空，迁移到这里无实际操作，纯形状兼容。

/// 旧规则对「版本段」的判定：末段以 `v` 开头、后面全是数字或小数点。
///
/// 与新版只认全数字的口径不同 —— 修正要复现的是**本项目当初怎么解释用户数据**。
fn legacy_has_version_segment(base: &str) -> bool {
    base.rsplit('/').next().is_some_and(|seg| {
        seg.starts_with('v')
            && seg.len() > 1
            && seg[1..].chars().all(|c| c.is_ascii_digit() || c == '.')
    })
}

/// 把旧版 `api_url` 规范化：旧拼接会自动补 `/v1`，这里按旧规则把会补的那段写进地址。
pub fn normalize_legacy_api_url(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    if trimmed.ends_with('#') {
        return trimmed.to_string();
    }
    let base = trimmed.trim_end_matches('/');
    if base.ends_with("/chat/completions") || legacy_has_version_segment(base) {
        return base.to_string();
    }
    format!("{base}/v1")
}

/// 旧厂商 id → 新预置 key。原实现的兜底是 `ai_profile::preset::CUSTOM_PRESET_KEY`，
/// 该 crate 已随 AI 删减移除，这里直接写字面量（值即 "custom"）。
pub fn legacy_provider_key(old: &str) -> &'static str {
    match old.trim() {
        "kimi" => "moonshot",
        "doubao" => "volcengine_ark",
        "openai" => "openai_official",
        "deepseek" => "deepseek",
        "zhipu" => "zhipu",
        "qwen" => "qwen",
        "siliconflow" => "siliconflow",
        "minimax" => "minimax",
        "qianfan" => "qianfan",
        "stepfun" => "stepfun",
        "baichuan" => "baichuan",
        "mimo" => "mimo",
        "gemini" => "gemini",
        "xai" => "xai",
        "groq" => "groq",
        "together" => "together",
        "openrouter" => "openrouter",
        "ollama" => "ollama",
        "lmstudio" => "lmstudio",
        "vllm" => "vllm",
        _ => "custom",
    }
}
