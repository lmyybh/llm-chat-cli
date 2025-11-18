use crate::model::openai::{ChatCompletionChunk, ChatCompletionRequest, Message, SamplingParams};
use futures_util::StreamExt;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use std::sync::mpsc;

const OPENAI_API_CONTENT_TYPE: &str = "application/json";
const SSE_DATA_PREFIX: &str = "data: ";

pub fn stream_completion(
    api_url: String,
    api_key: Option<String>,
    model: String,
    messages: Vec<Message>,
    sender: mpsc::Sender<String>,
) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async move {
            let client = reqwest::Client::new();

            let request_body = ChatCompletionRequest {
                model,
                messages,
                stream: true,
                sampling_params: SamplingParams { 
                    temperature: Some(0.7), 
                    top_p: Some(1.0), 
                    max_tokens: Some(1000), 
                    stop: None 
                },
            };

            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, HeaderValue::from_static(OPENAI_API_CONTENT_TYPE));
            if let Some(key) = api_key {
                headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", key)).unwrap());
            }

            let res = client
                .post(&api_url)
                .headers(headers)
                .json(&request_body)
                .send()
                .await;

            let response = match res {
                Ok(resp) => {
                    if !resp.status().is_success() {
                        eprintln!("LLM API error: {}", resp.status());
                        let _ = sender.send("__ERROR__".to_string());
                        return;
                    }
                    resp
                }
                Err(e) => {
                    eprintln!("Failed to send request: {:?}", e);
                    let _ = sender.send("__ERROR__".to_string());
                    return;
                }
            };

            // 获取字节流
            let mut stream = response.bytes_stream();

            let mut buffer = String::new();

            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(bytes) => {
                        // 将新字节追加到缓冲区
                        buffer.push_str(&String::from_utf8_lossy(&bytes));

                        // 按行处理（SSE 是按行发送的）
                        let lines: Vec<&str> = buffer.lines().collect();
                        let mut new_buffer = String::new();

                        // 如果最后一行不完整（没有 \n），保留在 buffer 中
                        if !buffer.ends_with('\n') && !buffer.ends_with("\r") {
                            if let Some(last) = lines.last() {
                                new_buffer.push_str(last);
                            }
                        }

                        // 处理完整行
                        for line in lines.iter().take(lines.len() - (if new_buffer.is_empty() { 0 } else { 1 })) {
                            let line = line.trim();
                            if line.is_empty() {
                                continue;
                            }

                            if line.starts_with(SSE_DATA_PREFIX) {
                                let data = &line[SSE_DATA_PREFIX.len()..];
                                if data == "[DONE]" {
                                    let _ = sender.send("__DONE__".to_string());
                                    return;
                                }

                                if let Ok(chunk) = serde_json::from_str::<ChatCompletionChunk>(data) {
                                    if let Some(content) = chunk.choices.get(0).and_then(|c| c.delta.reasoning_content.as_ref()) {
                                        let _ = sender.send(content.clone());
                                    }

                                    if let Some(content) = chunk.choices.get(0).and_then(|c| c.delta.content.as_ref()) {
                                        let _ = sender.send(content.clone());
                                    }
                                }
                            }
                            // 忽略其他行（如 event:, id:, retry: 等）
                        }

                        buffer = new_buffer;
                    }
                    Err(e) => {
                        eprintln!("Stream error: {:?}", e);
                        break;
                    }
                }
            }

            // 如果连接意外关闭但没收到 [DONE]
            let _ = sender.send("__DONE__".to_string());
        });
    });
}