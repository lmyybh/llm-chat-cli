import React from 'react';
import { Text, Box } from 'ink';
import TextBox from './TextBox.js';

function MessageBox({ message, bubbleTextColor, bubbleBorderColor }) {
    return (
        <Box flexDirection="column" alignItems="flex-start">
            <Text dimColor>
                {
                    message.role + " "
                    + message.timeStr() + " "
                    + (message.content ? "" : "reasoning...") + " "
                    + (message.duration() > 0 ? (message.duration() + "s") : "")
                }
            </Text>
            <TextBox
                text={message.content || message.reasoningContent}
                textColor={bubbleTextColor}
                borderColor={bubbleBorderColor}
                dimColor={!message.content}
                borderStyle={message.content ? 'round' : 'classic'}
            />
        </Box>
    )
}

export default MessageBox;