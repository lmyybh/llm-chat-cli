#!/usr/bin/env node
import React from 'react';
import {render} from 'ink';
import meow from 'meow';
import App from './app.js';

const cli = meow(
	`
		Usage
		  $ llm-chat-cli --url=http://localhost:8000/v1 --model=Qwen/Qwen3-8B
	`,
	{
		importMeta: import.meta,
	},
);

console.clear();

render(<App url={cli.flags.url} model={cli.flags.model} />);
