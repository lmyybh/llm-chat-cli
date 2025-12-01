import React from 'react';
import { Text, Box } from 'ink';

function TextBox({ text }) {
    return (
        <Box borderStyle="round" borderColor="blue" borderDimColor padding={1}>
            <Text color="red">{text}</Text>
        </Box>
    );
}

export default TextBox;