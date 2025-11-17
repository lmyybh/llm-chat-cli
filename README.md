# llm-chat-cli

```
llm-chat-tui/
├── Cargo.toml
├── src/
│   ├── main.rs                  # 程序入口：初始化终端、事件循环、主 App 结构驱动
│   ├── app.rs                   # 核心应用状态：包含当前选中会话、所有会话列表、输入框内容等
│   ├── ui/
│   │   ├── mod.rs               # UI 模块入口，导出各组件渲染函数
│   │   ├── layout.rs            # 定义整体布局（侧边栏 + 聊天区 + 输入框）
│   │   ├── sidebar.rs           # 渲染左侧会话列表（可滚动）
│   │   ├── chat_view.rs         # 渲染聊天气泡区域（用户 vs LLM）
│   │   └── input.rs             # 渲染底部输入框（支持编辑、回车发送）
│   ├── event.rs                 # 处理键盘、鼠标、终端 resize 等事件
│   ├── model/
│   │   ├── mod.rs
│   │   ├── message.rs           # 定义消息结构（角色、内容、时间戳等）
│   │   └── conversation.rs      # 定义会话结构（ID、标题、消息列表等）
│   ├── llm/
│   │   ├── mod.rs
│   │   └── client.rs            # 封装 LLM API 调用（如 OpenAI、Ollama 等）
│   └── state/
│       ├── mod.rs
│       └── persistence.rs       # （可选）本地持久化会话（如保存到 ~/.llm-chat/）
└── README.md
```