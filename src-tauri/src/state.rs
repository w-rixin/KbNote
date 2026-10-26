use std::fs::File;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};

use tokio::sync::{watch, Notify};

use crate::database::Database;
use crate::services::vault::VaultState;

/// 应用全局状态，通过 tauri::State 注入到 Command 中
pub struct AppState {
    pub db: Database,
    /// 数据根目录（默认 = app_data_dir；用户改自定义目录 / KB_DATA_DIR / 便携模式后为对应路径）
    /// 资产/sources/db 都基于此路径
    pub data_dir: PathBuf,
    /// 自动同步调度器唤醒信号：配置变更时 notify_one 重载
    pub sync_scheduler_notify: Arc<Notify>,
    /// V1 多端同步互斥闸门：同一 backend 同时只允许一个 pull/push/双向同步在跑，
    /// 防止并发 pull 互撞 `idx_notes_stable_uuid` UNIQUE、并发 push 互相覆盖远端 manifest
    pub sync_v1_gate: crate::services::sync_v1::lock::SyncGate,
    /// 待办提醒调度器唤醒信号：用户增/改/删/snooze 任务时 notify_one，
    /// 调度器立刻重新计算"下一个最早提醒时刻"并重 sleep。
    /// 这样实现的精度 ~毫秒，且空闲时零 DB 查询（只在事件驱动 + 5min 兜底唤醒时扫）。
    pub reminder_notify: Arc<Notify>,
    /// 启动时 argv 里的 .md 文件路径，等前端 mount 后 take 出来
    pub pending_open_md_path: Mutex<Option<String>>,
    /// T-007 笔记加密保险库：内存中的主密钥（可选），锁定时清空
    pub vault: RwLock<VaultState>,
    /// 单实例守护锁文件句柄（保持存活以维持独占锁，进程退出时自动释放）
    _lock_file: Option<File>,
}

impl AppState {
    pub fn new(db: Database, data_dir: PathBuf, lock_file: Option<File>) -> Self {
        Self {
            db,
            data_dir,
            sync_scheduler_notify: Arc::new(Notify::new()),
            sync_v1_gate: crate::services::sync_v1::lock::SyncGate::new(),
            reminder_notify: Arc::new(Notify::new()),
            pending_open_md_path: Mutex::new(None),
            vault: RwLock::new(VaultState::default()),
            _lock_file: lock_file,
        }
    }
}
