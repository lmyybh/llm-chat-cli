#!/usr/bin/env node
import React from 'react';
import {render} from 'ink';
import meow from 'meow';
import App from './app.js';

const cli = meow(
	`
		Usage
		  $ llm-chat-cli
	`,
	{
		importMeta: import.meta,
	},
);

console.clear();

render(<App url={cli.flags.url} model={cli.flags.model} />);
