//! 工具栏配置模块
//!
//! 该模块定义了工具栏按钮的配置数据结构和静态配置表，
//! 供工具栏组件和主窗口使用，以保持配置的一致性。

use adw::glib;
use once_cell::sync::Lazy;

/// 工具栏按钮配置结构体
#[derive(Debug, Clone)]
pub struct ToolbarButton {
    /// 图标名称
    pub icon_name: &'static str,
    /// 提示文本
    pub tooltip: &'static str,
    /// 动作类型
    pub action: ToolbarAction,
    /// 内容动作类型
    pub content_action: Option<ContentAction>,
    /// 信号标志（仅对信号类型动作有效）
    pub signal_flags: Option<glib::SignalFlags>,
}

/// 工具栏按钮动作类型枚举
#[derive(Debug, Clone)]
pub enum ToolbarAction {
    /// 发送信号，包含信号名称
    Signal(&'static str),
    /// 切换选中状态
    Toggle,
}

/// 内容动作类型枚举
#[derive(Debug, Clone)]
pub enum ContentAction {
    /// 显示AI聊天页面
    ShowAIChat,
    /// 显示欢迎页面
    ShowWelcome,
    /// 截图
    ScreenShot,
    ///
    EntityList,
}

/// 工具栏按钮配置表
pub static TOOLBAR_BUTTONS: Lazy<Vec<ToolbarButton>> = Lazy::new(|| {
    vec![
        ToolbarButton {
            icon_name: "document-new-symbolic",
            tooltip: "新建/AI聊天",
            action: ToolbarAction::Signal("show-ai-chat"),
            content_action: Some(ContentAction::ShowAIChat),
            signal_flags: Some(glib::SignalFlags::RUN_LAST | glib::SignalFlags::ACTION),
        },
        ToolbarButton {
            icon_name: "document-open-symbolic",
            tooltip: "截图工具",
            action: ToolbarAction::Signal("show-screen-shot"),
            content_action: Some(ContentAction::ScreenShot),
            signal_flags: Some(glib::SignalFlags::RUN_LAST | glib::SignalFlags::ACTION),
        },
        ToolbarButton {
            icon_name: "document-save-symbolic",
            tooltip: "保存",
            action: ToolbarAction::Signal("show-welcome"),
            content_action: Some(ContentAction::ShowWelcome),
            signal_flags: Some(glib::SignalFlags::RUN_LAST | glib::SignalFlags::ACTION),
        },
        ToolbarButton {
            icon_name: "document-save-symbolic",
            tooltip: "保存",
            action: ToolbarAction::Signal("show-entity-list"),
            content_action: Some(ContentAction::EntityList),
            signal_flags: Some(glib::SignalFlags::RUN_LAST | glib::SignalFlags::ACTION),
        },
    ]
});
