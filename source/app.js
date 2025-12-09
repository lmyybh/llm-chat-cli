import React, { useState, useEffect, useRef, useCallback } from 'react';
import { Text, Box, useApp } from 'ink';
import InputBox from './components/InputBox.js';
import MessageBox from './components/MessageBox.js';
import OpenAIClient, { Message } from './client.js';
import CommandManager from './commands/CommandManager.js';

const MESSAGE_BOX_COLOR = {
	system: {bubbleBorderColor: "#ff7e5f", bubbleTextColor: "#AE7568"},
	user: {bubbleBorderColor: "#4d9de0", bubbleTextColor: "#2c3e50"},
	assistant: {bubbleBorderColor: "#a663cc", bubbleTextColor: "#3b2f4d"},
}


const LOG_LEVELS = {
	info: { color: "#00ff00", prefix: "ℹ" },
	warning: { color: "#ffaa00", prefix: "⚠" },
	error: { color: "#ff6b6b", prefix: "✗" },
	success: { color: "#00d4ff", prefix: "✓" }
};

function App({ url, model }) {
	const [baseURL, setURL] = useState(url);
	const [modelName, setModel] = useState(model);
	const [text, setText] = useState('');
	const [messages, setMessages] = useState([new Message('system', '你是一个非常有帮助的助手')]);
	const [log, setLog] = useState({ message: '', level: 'info' });
	const [isGenerating, setIsGenerating] = useState(false);
	const { exit } = useApp();
	const commandManagerRef = React.useRef(new CommandManager());

	const handleInputTextSubmit = async (text) => {
		if (isGenerating || text.trim() === '') return;

		// 清空输入框
		setText('');

		// 清空日志
		setLog({ message: '', level: 'info' });

		// Use refs to track the current assistant message
		const assistantMessageRef = useRef(null);

		// 添加 user 消息
		const newMessages = [...messages, new Message('user', text)];
		setMessages(newMessages);

		// 添加 assistant 消息
		const newAssistantMsg = new Message('assistant', '');
		setMessages(prev => [...prev, newAssistantMsg]);
		// Store reference to the message for streaming updates
		assistantMessageRef.current = newAssistantMsg;

		const onStart = () => {
			setIsGenerating(true);
		};

		// Debounce update function to reduce flickering
		const updateMessage = useCallback((content, reasoningContent) => {
			if (assistantMessageRef.current) {
				assistantMessageRef.current.setContent(content);
				assistantMessageRef.current.setReasoningContent(reasoningContent);
				// Force re-render by creating a new array reference
				setMessages(prev => [...prev]);
			}
		}, []);

		const onChunk = (full) => {
			const content = full['content'].trim();
			const reasoningContent = full['reasoning_content'].trim();
			updateMessage(content, reasoningContent);
		};

		const onComplete = (full) => {
			const content = full['content'].trim();
			const reasoningContent = full['reasoning_content'].trim();
			if (assistantMessageRef.current) {
				assistantMessageRef.current.setContent(content);
				assistantMessageRef.current.setReasoningContent(reasoningContent);
				assistantMessageRef.current.updateTime();
			}
			// Final update to ensure the message is properly saved
			setMessages(prev => {
				// Create a new message to ensure proper state update
				const newMsg = assistantMessageRef.current;
				assistantMessageRef.current = null;
				return [...prev.slice(0, -1), newMsg];
			});
			setIsGenerating(false);
		};

		const onError = (error) => {
			setIsGenerating(false);

			// 删除最后的 user 和 assistant 消息
			setMessages(prev => prev.slice(0, -2));

			// 回退用户消息到输入框
			setText(text);

			// 显示错误信息
			setLog({ message: `请求失败: ${error.message}`, level: 'error' });
		};

		// 发送请求
		const client = new OpenAIClient(baseURL, modelName);

		await client.send(
			newMessages.map((m) => m.toOpenAI()),
			onStart,
			onChunk,
			onComplete,
			onError
		);
	};

	// Handle command execution
	const handleCommand = async (commandText, onSuccess, onFailure) => {
		try {
			const context = {
				messages,
				baseURL,
				model: modelName,
				setURL,
				setModel,
				setMessages,
				clearMessages: () => setMessages([new Message('system', '你是一个非常有帮助的助手')]),
				setSystemPrompt: (prompt) => {
					setMessages(prev => {
						const updated = [...prev];
						const systemIndex = updated.findIndex(m => m.role === 'system');
						if (systemIndex >= 0) {
							updated[systemIndex] = new Message('system', prompt);
						} else {
							updated.unshift(new Message('system', prompt));
						}
						return updated;
					});
				},
				exit: () => exit()
			};

			const result = await commandManagerRef.current.parseAndExecute(commandText, context);

			if (result.shouldExit) {
				onSuccess?.();
				return;
			}

			if (result.message) {
				// Display command result in the output area with appropriate level
				const level = result.success ? 'info' : 'error';
				setLog({ message: result.message, level });
			}

			// Check if command was successful
			if (result.success) {
				onSuccess?.();
			} else {
				onFailure?.();
			}
		} catch (error) {
			// Handle any unexpected errors during command execution
			setLog({ message: `Error: ${error.message}`, level: 'error' });
			onFailure?.();
		}
	};

	return (
		<>
			<Box flexDirection="column" alignItems="flex-start">
				{messages.map((msg, index) => (
					<MessageBox
						key={index}
						message={msg}
						bubbleBorderColor={MESSAGE_BOX_COLOR[msg.role].bubbleBorderColor}
						bubbleTextColor={MESSAGE_BOX_COLOR[msg.role].bubbleTextColor}
					/>
				))}
			</Box>

			<Box marginTop={1} flexDirection="column">
				{/* Output display area - above input box */}
				{log.message && (
					<Box marginBottom={1}>
						<Text color={LOG_LEVELS[log.level].color} dimColor>
							{LOG_LEVELS[log.level].prefix} {log.message}
						</Text>
					</Box>
				)}
				<InputBox
					value={text}
					onChange={setText}
					placeholder={isGenerating ? '正在生成回答，请等待...' : '输入消息或 / 命令...'}
					onSubmit={handleInputTextSubmit}
					onCommand={handleCommand}
					borderColor="#6c757d"
					borderLeft={false}
					borderRight={false}
					identifierColor="#cccccc"
				/>
				<Text dimColor>{`model: ${modelName}, url: ${baseURL}`}</Text>
			</Box>
		</>
	);
}

export default App;