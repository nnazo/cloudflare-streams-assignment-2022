use headers::{Header, HeaderName, HeaderValue};
use once_cell::sync::Lazy;

// A custom header for the queue consumer.
#[derive(Debug, Copy, Clone)]
pub struct QueueConsumer(usize);

// Note: This is not QUEUE_CONSUMER as specified because the `headers` library errors on SCREAMING_SNAKE_CASE
// headers and only is accepting kebab-case
static QUEUE_CONSUMER: Lazy<HeaderName> = Lazy::new(|| HeaderName::from_static("queue-consumer"));

impl Header for QueueConsumer {
    fn name() -> &'static HeaderName {
        &QUEUE_CONSUMER
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        I: Iterator<Item = &'i HeaderValue>,
    {
        let value = values.next().ok_or_else(headers::Error::invalid)?;

        match value.to_str() {
            Ok(value) => match value.parse::<usize>() {
                Ok(id) => Ok(QueueConsumer(id)),
                _ => Err(headers::Error::invalid()),
            },
            _ => Err(headers::Error::invalid()),
        }
    }

    fn encode<E>(&self, values: &mut E)
    where
        E: Extend<HeaderValue>,
    {
        let s = format!("{}", self.0);
        let value = HeaderValue::from_str(&s).expect("Invalid consumer ID format");

        values.extend(std::iter::once(value));
    }
}
