import { useMemo } from 'react';
import { Text, Box } from 'ink';
import TextBox from './TextBox.js';

function MessageBox({ message, bubbleTextColor, bubbleBorderColor }) {
    // Memoize the metadata to prevent unnecessary recalculations
    const metadata = useMemo(() => {
        const hasContent = Boolean(message.content);
        const duration = message.duration();
        return {
            timeStr: message.timeStr(),
            hasContent,
            durationStr: duration > 0 ? ` ${duration}s` : '',
            statusStr: hasContent ? '' : ' reasoning...'
        };
    }, [message]);

    // Memoize the text content
    const textContent = useMemo(() => {
        return message.content || message.reasoningContent || '';
    }, [message.content, message.reasoningContent]);

    return (
        <Box flexDirection="column" alignItems="flex-start" marginBottom={1}>
            <Text dimColor>
                {message.role} {metadata.timeStr}{metadata.statusStr}{metadata.durationStr}
            </Text>
            <Box marginTop={0} minHeight={3}>
                <TextBox
                    text={textContent}
                    textColor={bubbleTextColor}
                    borderColor={bubbleBorderColor}
                    dimColor={!metadata.hasContent}
                    borderStyle={metadata.hasContent ? 'round' : 'classic'}
                />
            </Box>
        </Box>
    )
}

export default MessageBox;