export class Command {
    constructor(name, description, usage = '', handler = null) {
        this.name = name;
        this.description = description;
        this.usage = usage;
        this.handler = handler;
    }

    async execute(args, context) {
        if (this.handler) {
            return await this.handler(args, context);
        }
        return `Command ${this.name} not implemented`;
    }
}

export class CommandManager {
    constructor() {
        this.commands = new Map();
        this.registerDefaultCommands();
    }

    registerDefaultCommands() {
        // Help command
        this.register(new Command(
            'help',
            'Show available commands',
            '/help [command]',
            async (args, context) => {
                if (args.length > 0) {
                    const commandName = args[0].toLowerCase();
                    const command = this.getCommand(commandName);
                    if (command) {
                        return [
                            `Command: /${command.name}`,
                            `Description: ${command.description}`,
                            `Usage: ${command.usage}`,
                        ].join('\n');
                    }
                    return `Unknown command: ${commandName}`;
                }

                const commandList = Array.from(this.commands.values())
                    .map(cmd => `  /${cmd.name.padEnd(15)} ${cmd.description}`)
                    .join('\n');

                return [
                    'Available commands:',
                    commandList,
                    '\nType /help <command> for more information about a specific command.',
                ].join('\n');
            }
        ));

        // Clear command
        this.register(new Command(
            'clear',
            'Clear the conversation history',
            '/clear',
            async (args, context) => {
                context.clearMessages();
                return 'Conversation history cleared.';
            }
        ));

        // Exit command
        this.register(new Command(
            'exit',
            'Exit the application',
            '/exit',
            async (args, context) => {
                context.exit();
                return null; // Return null to indicate exit
            }
        ));

        // Model command
        this.register(new Command(
            'model',
            'Change or view the current model',
            '/model [model-name]',
            async (args, context) => {
                try {
                    if (args.length === 0) {
                        return `Current model: ${context.model}`;
                    }
                    const newModel = args.join(' ');
                    if (!newModel || newModel.trim() === '') {
                        throw new Error('Model name cannot be empty');
                    }
                    context.setModel(newModel);
                    return `Model changed to: ${newModel}`;
                } catch (error) {
                    throw new Error(`Failed to change model: ${error.message}`);
                }
            }
        ));

        // URL command
        this.register(new Command(
            'url',
            'Change or view the current API URL',
            '/url [api-url]',
            async (args, context) => {
                try {
                    if (args.length === 0) {
                        return `Current URL: ${context.baseURL}`;
                    }
                    const newURL = args.join(' ');
                    if (!newURL || newURL.trim() === '') {
                        throw new Error('URL cannot be empty');
                    }
                    // Basic URL validation
                    try {
                        new URL(newURL);
                    } catch (e) {
                        throw new Error('Invalid URL format');
                    }
                    context.setURL(newURL);
                    return `URL changed to: ${newURL}`;
                } catch (error) {
                    throw new Error(`Failed to change URL: ${error.message}`);
                }
            }
        ));

        // System prompt command
        this.register(new Command(
            'system',
            'View or change the system prompt',
            '/system [prompt]',
            async (args, context) => {
                try {
                    if (args.length === 0) {
                        const systemMsg = context.messages.find(m => m.role === 'system');
                        return systemMsg ? `Current system prompt: ${systemMsg.content}` : 'No system prompt set';
                    }
                    const newPrompt = args.join(' ');
                    context.setSystemPrompt(newPrompt);
                    return `System prompt updated`;
                } catch (error) {
                    throw new Error(`Failed to update system prompt: ${error.message}`);
                }
            }
        ));
    }

    register(command) {
        this.commands.set(command.name.toLowerCase(), command);
    }

    getCommand(name) {
        return this.commands.get(name.toLowerCase());
    }

    getAllCommands() {
        return Array.from(this.commands.values());
    }

    async parseAndExecute(input, context) {
        const trimmed = input.trim();

        // Remove leading slash
        if (!trimmed.startsWith('/')) {
            return { success: false, message: null };
        }

        const parts = trimmed.slice(1).split(/\s+/);
        const commandName = parts[0]?.toLowerCase();

        if (!commandName) {
            return { success: true, message: this.getCommand('help').execute([], context) };
        }

        const command = this.getCommand(commandName);
        if (!command) {
            return {
                success: false,
                message: `Unknown command: /${commandName}. Type /help for available commands.`
            };
        }

        try {
            const args = parts.slice(1);
            const result = command.execute(args, context);
            return { success: true, message: result, shouldExit: commandName === 'exit' };
        } catch (error) {
            return {
                success: false,
                message: `Error executing command: ${error.message}`
            };
        }
    }

    suggestCommand(partial) {
        const partialLower = partial.toLowerCase();

        // If no partial input, return all commands
        if (!partial || partial === '') {
            return Array.from(this.commands.keys());
        }

        const suggestions = Array.from(this.commands.keys())
            .filter(name => name.startsWith(partialLower));

        if (suggestions.length === 1) {
            return suggestions[0];
        }
        return suggestions;
    }
}

export default CommandManager;