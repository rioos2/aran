use api::base::QueryInput;

const DEFAULT_LIMIT: &'static str = "10";

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct LogOutput {
    time: String,
    log: String,
}

impl LogOutput {
    pub fn with(t: &str, o: &str) -> LogOutput {
        LogOutput {
            time: t.to_string(),
            log: o.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct LogQueryBuilder {
    pub input: QueryInput,
}

impl LogQueryBuilder {
    pub fn with(labels: QueryInput) -> LogQueryBuilder {
        LogQueryBuilder { input: labels }
    }

    pub fn get(&self, key: &str) -> String {
        self.input
            .labels
            .get(key)
            .unwrap_or(&"".to_string())
            .to_string()
    }

    pub fn get_limits(&self, key: &str) -> String {
        self.input
            .labels
            .get(key)
            .unwrap_or(&DEFAULT_LIMIT.to_string())
            .to_string()
    }
}
