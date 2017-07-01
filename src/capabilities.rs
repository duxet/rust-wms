#[derive(Debug, Deserialize)]
pub struct Capabilities {
    service: Service
}

#[derive(Debug, Deserialize)]
struct Service {
    name: String,
    title: String
}
