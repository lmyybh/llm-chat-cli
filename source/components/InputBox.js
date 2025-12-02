import React from 'react';
import { Text, Box } from 'ink';
import TextInput from './TextInput.js';

function InputBox({ 
    value, onChange, placeholder, onSubmit, 
    borderColor='white', borderDimColor=false, 
    borderTop=true, borderBottom=true, borderLeft=true, borderRight=true,
    identifier = '>', identifierColor = 'red'
}) {
    return (
        <Box 
            borderStyle="round" borderColor={borderColor} borderDimColor={borderDimColor} 
            borderTop={borderTop} borderBottom={borderBottom} borderLeft={borderLeft} borderRight={borderRight}
            flexGrow={1}
        >
            <Box>
                <Text color={identifierColor}>{`${identifier} `}</Text>
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