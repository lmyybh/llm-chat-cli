import React from 'react';
import { Text, Box } from 'ink';

function TextBox({ text, textColor, borderColor, dimColor, borderStyle='round' }) {
    return (
        <Box
            borderStyle={borderStyle}
            borderColor={borderColor}
            borderDimColor={dimColor}
            flexGrow={1}
            width="100%"
        >
            <Text
                color={textColor}
                dimColor={dimColor}
                wrap="wrap"
            >
                {text || ' '}
            </Text>
        </Box>
    );
}

export default TextBox;