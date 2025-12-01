import React, { useState } from 'react';
import { Text, Box } from 'ink';
import InputBox from './components/InputBox.js';
import TextBox from './components/TextBox.js';
import OpenAIClient from './client.js';

function App({ name = 'Stranger' }) {
	const [text, setText] = useState('');
	const [messages, setMessages] = useState([]);

	const client = new OpenAIClient(
		"http://localhost:32110/v1",
		"Qwen/Qwen3-8B"
	);

	const handleInputTextSubmit = async (text) => {
		const newMessages = [...messages, {role: 'user', content: text}];
		setMessages(newMessages);

		await client.send(
			newMessages.map((m) => ({role: m.role, content: m.content}))
		);

		// stream.on('content', (delta, snapshot) => {
		// 	console.log('delta', delta);
		// });

		// 清空输入框
		setText('');
	};

	return (
		<>
			<Box flexDirection="column" >
				{messages.map((msg, index) => (
					<TextBox key={index} text={msg.content} />
				))}
			</Box>
			<InputBox 
				value={text} 
				onChange={setText} 
				placeholder={'请输入文本'} 
				onSubmit={handleInputTextSubmit}
			/>
		</>
	);
}

export default App;