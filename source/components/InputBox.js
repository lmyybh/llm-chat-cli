import React, { useState, useEffect } from 'react';
import { Text, Box, useInput } from 'ink';
import TextInput from './TextInput.js';
import CommandManager from '../commands/CommandManager.js';

function InputBox({
    value, onChange, placeholder, onSubmit,
    borderColor='white', borderDimColor=false,
    borderTop=true, borderBottom=true, borderLeft=true, borderRight=true,
    identifier = '>', identifierColor = 'red',
    onCommand
}) {
    const [suggestions, setSuggestions] = useState([]);
    const [showSuggestions, setShowSuggestions] = useState(false);
    const [selectedSuggestion, setSelectedSuggestion] = useState(0);
    const commandManagerRef = React.useRef(new CommandManager());

    // Handle keyboard shortcuts
    useInput((_, key) => {
        if (value.startsWith('/')) {
            // Command mode shortcuts
            if (key.tab) {
                const commandPart = value.slice(1).split(' ')[0];
                const suggested = commandManagerRef.current.suggestCommand(commandPart);

                if (Array.isArray(suggested) && suggested.length > 0) {
                    setShowSuggestions(true);
                    setSelectedSuggestion(0);
                } else if (typeof suggested === 'string' && suggested) {
                    onChange('/' + suggested + ' ');
                    setShowSuggestions(false);
                }
            } else if (key.upArrow && showSuggestions) {
                setSelectedSuggestion(prev =>
                    prev > 0 ? prev - 1 : suggestions.length - 1
                );
            } else if (key.downArrow && showSuggestions) {
                setSelectedSuggestion(prev =>
                    prev < suggestions.length - 1 ? prev + 1 : 0
                );
            } else if (key.return && showSuggestions) {
                const selected = suggestions[selectedSuggestion];
                onChange('/' + selected + ' ');
                setShowSuggestions(false);
                setSelectedSuggestion(0);
            }
        }
    });

    const handleSubmit = (text) => {
        if (text.trim().startsWith('/')) {
            // Command mode - pass a callback to handle success/failure
            onCommand?.(text, () => {
                // On success: clear input
                onChange('');
                setShowSuggestions(false);
                setSelectedSuggestion(0);
            }, () => {
                // On failure: keep input (do nothing)
            });
        } else {
            // Normal chat mode
            onSubmit?.(text);
            onChange('');
            setShowSuggestions(false);
            setSelectedSuggestion(0);
        }
    };

    // Update suggestions based on current input
    useEffect(() => {
        if (value && value.startsWith('/')) {
            const commandPart = value.slice(1).split(' ')[0];
            const suggested = commandManagerRef.current.suggestCommand(commandPart);

            if (Array.isArray(suggested)) {
                setSuggestions(suggested);
                setShowSuggestions(suggested.length > 0);
            } else if (typeof suggested === 'string' && suggested) {
                setSuggestions([suggested]);
                setShowSuggestions(true);
            } else {
                setSuggestions([]);
                setShowSuggestions(false);
            }
        } else if (value === '/') {
            // Show all commands when just slash is typed
            const allCommands = commandManagerRef.current.getAllCommands().map(cmd => cmd.name);
            setSuggestions(allCommands);
            setShowSuggestions(true);
        } else {
            setSuggestions([]);
            setShowSuggestions(false);
        }
    }, [value]);

    // Dynamic identifier based on mode
    const displayIdentifier = value.startsWith('/') ? '$' : identifier;
    const displayIdentifierColor = value.startsWith('/') ? '#00ff00' : identifierColor;
    const displayPlaceholder = value.startsWith('/')
        ? 'Enter command (TAB for suggestions)'
        : placeholder;

    return (
        <Box flexDirection="column">
            <Box
                borderStyle="round"
                borderColor={value.startsWith('/') ? '#00ff00' : borderColor}
                borderDimColor={borderDimColor}
                borderTop={borderTop}
                borderBottom={borderBottom}
                borderLeft={borderLeft}
                borderRight={borderRight}
                flexGrow={1}
            >
                <Box>
                    <Text color={displayIdentifierColor}>{`${displayIdentifier} `}</Text>
                </Box>

                <TextInput
                    showCursor
                    value={value}
                    onChange={onChange}
                    placeholder={displayPlaceholder}
                    onSubmit={handleSubmit}
                />
            </Box>

            {/* Command suggestions */}
            {showSuggestions && suggestions.length > 0 && (
                <Box flexDirection="column" marginLeft={2}>
                    {suggestions.map((suggestion, index) => (
                        <Box key={suggestion}>
                            <Text
                                color={index === selectedSuggestion ? "#00ff00" : "#666"}
                                dimColor={index !== selectedSuggestion}
                            >
                                {index === selectedSuggestion ? '▶ ' : '  '}
                                /{suggestion}
                            </Text>
                        </Box>
                    ))}
                    <Text color="#666" dimColor>
                        Use ↑↓ to navigate, TAB to complete
                    </Text>
                </Box>
            )}
        </Box>
    );
}

export default InputBox;