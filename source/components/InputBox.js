import React from 'react';
import { Text, Box } from 'ink';
import TextInput from 'ink-text-input';

function InputBox({ value, onChange, placeholder, onSubmit }) {
    return (
        <Box borderStyle="round" borderColor="green" borderDimColor>
            <Box margin={0}>
                <Text color="red">{' > '}</Text>
            </Box>

            <TextInput
                showCursor
                value={value}
                onChange={onChange}
                placeholder={placeholder}
                onSubmit={onSubmit}
            />
        </Box>
    );
}

export default InputBox;