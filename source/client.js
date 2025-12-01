import OpenAI from 'openai';

class OpenAIClient {
    constructor(baseURL, model, apiKey="") {
        this.baseURL = baseURL;
        this.model = model;
        this.apiKey = apiKey;

        this.client = new OpenAI({
            baseURL: this.baseURL,
            apiKey: this.apiKey,
        });
    }

    async send(messages) {
        console.log("send")
        try {
            const stream = await this.client.chat.completions.create({
                model: this.model,
                messages: messages,
                stream: true
            });
            console.log(stream)
        } catch (error) {
            console.log(error)
        }
        
    }
}

export default OpenAIClient;