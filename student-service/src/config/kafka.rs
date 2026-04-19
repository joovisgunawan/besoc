use crate::config::helpers::get_env;

#[derive(Debug, Clone)]
pub struct KafkaConfig {
    pub broker: String,
    pub topic_student_events: String,
    pub topic_supervision_events: String,
}

impl KafkaConfig {
    pub fn from_env() -> Self {
        Self {
            broker: get_env("KAFKA_BROKER", "localhost:9092"),
            topic_student_events: get_env("KAFKA_TOPIC_STUDENT_EVENTS", "student-events"),
            topic_supervision_events: get_env(
                "KAFKA_TOPIC_SUPERVISION_EVENTS",
                "supervision-events",
            ),
        }
    }
}