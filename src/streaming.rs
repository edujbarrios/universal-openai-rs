use std::pin::Pin;

use futures_util::StreamExt;

use crate::{ChatStreamEvent, Result};

pub type ChatStream = Pin<Box<dyn futures_core::Stream<Item = Result<ChatStreamEvent>> + Send>>;
pub type TextChunkStream = Pin<Box<dyn futures_core::Stream<Item = Result<String>> + Send>>;

pub trait StreamDecoder {
    type Event;

    fn decode_line(&mut self, line: &str) -> Result<Option<Self::Event>>;
}

#[derive(Debug, Default, Clone)]
pub struct OpenAiSseDecoder {
    done: bool,
}

impl OpenAiSseDecoder {
    pub fn new() -> Self {
        Self::default()
    }
}

impl StreamDecoder for OpenAiSseDecoder {
    type Event = ChatStreamEvent;

    fn decode_line(&mut self, line: &str) -> Result<Option<Self::Event>> {
        let line = line.trim();

        if self.done || line.is_empty() || line.starts_with("event:") {
            return Ok(None);
        }

        let Some(data) = line.strip_prefix("data:") else {
            return Ok(None);
        };
        let data = data.trim();

        if data == "[DONE]" {
            self.done = true;
            return Ok(None);
        }

        Ok(Some(serde_json::from_str::<ChatStreamEvent>(data)?))
    }
}

#[derive(Debug, Default, Clone)]
pub struct LenientSseDecoder {
    done: bool,
}

impl LenientSseDecoder {
    pub fn new() -> Self {
        Self::default()
    }
}

impl StreamDecoder for LenientSseDecoder {
    type Event = ChatStreamEvent;

    fn decode_line(&mut self, line: &str) -> Result<Option<Self::Event>> {
        let line = line.trim();

        if self.done || line.is_empty() || line.starts_with("event:") || line.starts_with("id:") {
            return Ok(None);
        }

        let data = line.strip_prefix("data:").unwrap_or(line).trim();

        if data == "[DONE]" || data.eq_ignore_ascii_case("done") {
            self.done = true;
            return Ok(None);
        }

        Ok(Some(serde_json::from_str::<ChatStreamEvent>(data)?))
    }
}

#[derive(Debug, Default, Clone)]
pub struct JsonLinesDecoder;

impl JsonLinesDecoder {
    pub fn new() -> Self {
        Self
    }
}

impl StreamDecoder for JsonLinesDecoder {
    type Event = ChatStreamEvent;

    fn decode_line(&mut self, line: &str) -> Result<Option<Self::Event>> {
        let line = line.trim();

        if line.is_empty() || line == "[DONE]" || line.eq_ignore_ascii_case("done") {
            return Ok(None);
        }

        Ok(Some(serde_json::from_str::<ChatStreamEvent>(line)?))
    }
}

pub(crate) fn decode_response_stream<D>(response: reqwest::Response, mut decoder: D) -> ChatStream
where
    D: StreamDecoder<Event = ChatStreamEvent> + Send + 'static,
{
    let stream = async_stream::try_stream! {
        let bytes = response.bytes_stream();
        futures_util::pin_mut!(bytes);
        let mut buffer = String::new();

        while let Some(chunk) = bytes.next().await {
            let chunk = chunk?;
            buffer.push_str(&String::from_utf8_lossy(&chunk));

            while let Some(newline) = buffer.find('\n') {
                let line: String = buffer.drain(..=newline).collect();
                if let Some(event) = decoder.decode_line(&line)? {
                    yield event;
                }
            }
        }

        if !buffer.trim().is_empty() {
            if let Some(event) = decoder.decode_line(&buffer)? {
                yield event;
            }
        }
    };

    Box::pin(stream)
}

pub(crate) fn text_chunks_from_events(mut events: ChatStream) -> TextChunkStream {
    let stream = async_stream::try_stream! {
        while let Some(event) = events.next().await {
            let event = event?;

            for choice in event.choices {
                if let Some(text) = choice.delta.content {
                    yield text;
                }
            }
        }
    };

    Box::pin(stream)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EVENT: &str = r#"{"choices":[{"index":0,"delta":{"content":"hello"}}]}"#;

    #[test]
    fn openai_sse_decoder_reads_data_lines() {
        let mut decoder = OpenAiSseDecoder::new();
        let event = decoder
            .decode_line(&format!("data: {EVENT}"))
            .unwrap()
            .unwrap();

        assert_eq!(event.choices[0].delta.content.as_deref(), Some("hello"));
        assert!(decoder.decode_line("data: [DONE]").unwrap().is_none());
    }

    #[test]
    fn lenient_sse_decoder_accepts_raw_json_lines() {
        let mut decoder = LenientSseDecoder::new();
        let event = decoder.decode_line(EVENT).unwrap().unwrap();

        assert_eq!(event.choices[0].delta.content.as_deref(), Some("hello"));
    }

    #[test]
    fn json_lines_decoder_accepts_json_lines() {
        let mut decoder = JsonLinesDecoder::new();
        let event = decoder.decode_line(EVENT).unwrap().unwrap();

        assert_eq!(event.choices[0].delta.content.as_deref(), Some("hello"));
    }
}
