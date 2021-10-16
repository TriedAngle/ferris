#[derive(Debug, Clone)]
pub struct DataService {
    pub client: mongodb::Client,
    pub db: mongodb::Database,
    pub redis: redis::Client,
}
