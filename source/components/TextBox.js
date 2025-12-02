import React from 'react';
import { Text, Box } from 'ink';

function TextBox({ text, textColor, borderColor, dimColor, borderStyle='round' }) {
    return (
        <Box
            borderStyle={borderStyle}
            borderColor={borderColor}
            borderDimColor={dimColor}
        >
            <Text color={textColor} dimColor={dimColor}>{text}</Text>
        </Box>
    );
}

export default TextBox;