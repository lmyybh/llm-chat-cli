import React, { useState } from 'react';
import { Text, Box } from 'ink';
import InputBox from './components/InputBox.js';
import MessageBox from './components/MessageBox.js';
import OpenAIClient, { Message } from './client.js';

const MESSAGE_BOX_COLOR = {
	system: {bubbleBorderColor: "#ff7e5f", bubbleTextColor: "#AE7568"},
	user: {bubbleBorderColor: "#4d9de0", bubbleTextColor: "#2c3e50"},
	assistant: {bubbleBorderColor: "#a663cc", bubbleTextColor: "#3b2f4d"},
}


function App({ url, model }) {
	const [baseURL, setURL] = useState(url);
	const [modelName, setModel] = useState(model);
	const [text, setText] = useState('');
	const [messages, setMessages] = useState([new Message('system', '你是一个非常有帮助的助手')]);
	const [log, setLog] = useState('');
	const [isGenerating, setIsGenerating] = useState(false);

	const handleInputTextSubmit = async (text) => {
		if (isGenerating || text.trim() === '') return;

		// 清空输入框
		setText('');

		// 清空日志
		setLog('');

		// 添加 user 消息
		const newMessages = [...messages, new Message('user', text)];
		setMessages(newMessages);

		// 添加 assistant 消息
		setMessages(prev => [...prev, new Message('assistant', '')]);

		const onStart = () => {
			setIsGenerating(true);
		};

		const onChunk = (full) => {
			setMessages(prev => {
				const updated = [...prev];
				updated[updated.length - 1].setContent(full['content'].trim());
				updated[updated.length - 1].setReasoningContent(full['reasoning_content'].trim());
				return updated;
			});
		};

		const onComplete = (full) => {
			setMessages(prev => {
				const updated = [...prev];
				updated[updated.length - 1].setContent(full['content'].trim());
				updated[updated.length - 1].setReasoningContent(full['reasoning_content'].trim());
				updated[updated.length - 1].updateTime();
				return updated;
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
			setLog(`请求失败: ${error.message}`);
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
				<Text dimColor>{`model: ${modelName}, url: ${baseURL}` + (log ? `, log: ${log}` : "")}</Text>
				<InputBox
					value={text}
					onChange={setText}
					placeholder={isGenerating ? '正在生成回答，请等待...' : ''}
					onSubmit={handleInputTextSubmit}
					borderColor="#6c757d"
					borderLeft={false}
					borderRight={false}
					identifierColor="#cccccc"
				/>
			</Box>
		</>
	);
}

export default App;