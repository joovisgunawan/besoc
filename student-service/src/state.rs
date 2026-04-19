use sqlx::PgPool;
use crate::cache::RedisCache;
use crate::kafka::producer::KafkaProducer;

#[derive(Clone)]
pub struct AppState {
    pub write_db: PgPool,
    pub read_db: PgPool,
    pub cache: RedisCache,
    pub kafka: KafkaProducer,
    pub kafka_topic_student_events: String,
}