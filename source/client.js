import OpenAI from 'openai';
import { format } from 'date-fns';

const ROLES = ["system", "user", "assistant"];

export class Message {
    constructor(role, content, reasoning_content = "") {
        console.assert(ROLES.includes(role), "Invalid role: " + role);

        this.role = role;
        this.content = content;
        this.reasoningContent = reasoning_content;
        this.startTime = Date.now();
        this.stopTime = this.startTime
    }

    setContent(content) {
        this.content = content;
    }

    setReasoningContent(reasoning_content) {
        this.reasoningContent = reasoning_content;
    }

    toOpenAI() {
        return {
            role: this.role,
            content: this.content,
        };
    }

    duration() {
        return Math.floor((this.stopTime - this.startTime) / 1000);
    }

    updateTime() {
        this.stopTime = Date.now();
    }

    timeStr() {
        return format(this.stopTime, 'yyyy-MM-dd HH:mm:ss');
    }
}

class OpenAIClient {
    constructor(baseURL, model, apiKey = "") {
        this.baseURL = baseURL;
        this.model = model;
        this.apiKey = apiKey;

        this.client = new OpenAI({
            baseURL: this.baseURL,
            apiKey: this.apiKey,
            timeout: 5000, // 10s
            maxRetries: 3,
        });
    }

    async send(messages, onStart, onChunk, onComplete, onError) {
        try {
            onStart?.()

            const stream = await this.client.chat.completions.create({
                model: this.model,
                messages: messages,
                stream: true
            });

            let fullContent = { "reasoning_content": "", "content": "" }
            for await (const chunk of stream) {
                const delta = chunk.choices[0]?.delta
                fullContent['reasoning_content'] += delta?.reasoning_content || '';
                fullContent['content'] += delta?.content || '';

                onChunk?.(fullContent);
            }
            onComplete?.(fullContent)

        } catch (error) {
            onError?.(error);
        }
    }
}

export default OpenAIClient;