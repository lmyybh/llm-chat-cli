# LLM Chat CLI - 项目概览

## 项目简介
这是一个基于 React Ink 框架构建的命令行聊天应用，支持与任何兼容 OpenAI API 的大语言模型进行交互对话。

## 技术栈
- **UI 框架**: React + Ink (命令行 React)
- **AI SDK**: OpenAI
- **构建工具**: Babel
- **代码规范**: XO + Prettier
- **测试框架**: AVA

## 项目结构
```
llm-chat-cli/
├── source/                    # 源代码目录
│   ├── app.js                # 主应用组件 (App)
│   ├── cli.js                # CLI 入口文件
│   ├── client.js             # OpenAI 客户端封装
│   └── components/           # UI 组件
│       ├── InputBox.js       # 输入框组件
│       ├── MessageBox.js     # 消息显示组件
│       ├── TextBox.js        # 文本显示组件
│       └── TextInput.js      # 文本输入组件
├── dist/                     # 编译输出目录
├── package.json              # 项目配置
└── readme.md                 # 项目说明
```

## 核心功能

### 1. 聊天界面
- 提供美观的命令行聊天 UI
- 支持不同角色的消息气泡（系统、用户、助手）
- 实时显示模型和连接信息

### 2. 消息管理
- 支持 system、user、assistant 三种角色
- 消息历史记录管理
- 每条消息包含时间戳

### 3. 流式响应
- 实时显示 AI 生成的响应内容
- 支持 `reasoning_content` 字段（用于显示 AI 思考过程）
- 响应过程中显示"正在生成"提示

### 4. 错误处理
- 网络请求失败时的错误提示
- 自动回滚失败的消息
- 保持用户输入不丢失

## 使用方式

### 安装
```bash
npm install --global llm-chat-cli
```

### 运行
```bash
# 使用默认配置
llm-chat-cli

# 自定义 URL 和模型
llm-chat-cli --url=<your-api-url> --model=<model-name>
```

## 配置选项
- `--url`: API 服务器地址
- `--model`: 使用的模型名称
- `--name`: 用户名称（未在代码中实现）

## 扩展性
虽然项目名为 LLM Chat CLI，但通过配置可以连接到任何兼容 OpenAI API 的服务，包括：
- OpenAI GPT 系列
- Anthropic Claude
- 本地部署的 LLM 服务
- 其他兼容 OpenAI API 的服务

## 开发命令
- `npm run build`: 构建项目
- `npm run dev`: 开发模式（监听文件变化）
- `npm test`: 运行测试（代码检查 + 测试）

## 代码规范
- 使用 XO 进行 ESLint 检查
- 使用 Prettier 进行代码格式化
- 支持 ES modules
- 最低 Node.js 版本: 16

## 注意事项
1. 项目目前处于早期开发阶段（版本 0.0.0）
2. CLI 参数解析使用了 meow，但部分参数（如 --name）可能尚未完全实现
3. 默认系统提示为中文："你是一个非常有帮助的助手"
4. 超时时间设置为 5 秒，最大重试次数为 3 次