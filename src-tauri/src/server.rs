use super::rpc;
#[derive(std::fmt::Debug, Clone, serde::Serialize)]
pub struct Item {
    pub obj_id: u32,
    pub name: String,
    pub hint: Option<String>,
    pub icon: Option<String>,
}
pub struct Server {
    rpc: rpc::Server,
    prompt: String,
}
use regex::Regex;
impl Server {
    pub async fn new(uri: &str) -> Result<Self, String> {
        Ok(Self {
            rpc: rpc::Server::connect(uri.parse::<tonic::transport::Endpoint>().unwrap())
                .await
                .map_err(|e| format!("connect failed: {}", e))?,
            prompt: String::new(),
        })
    }
    fn extract_prompt(&mut self, s: &str) -> Option<String> {
        static RE: once_cell::sync::Lazy<Regex> = once_cell::sync::Lazy::new(|| {
            Regex::new(r"\|(?P<prompt>\w+)\|(?P<content>.*)").unwrap()
        });
        if let Some(caps) = RE.captures(s) {
            self.prompt = caps.name("prompt").unwrap().as_str().to_string();
            return Some(caps.name("content").unwrap().as_str().to_string());
        }
        return None;
    }
    pub async fn search(&mut self, content: &str) -> Result<Vec<Item>, String> {
        let content = self
            .extract_prompt(content)
            .ok_or_else(|| "invalid prompt")?;
        let service = self
            .rpc
            .get_ext(&self.prompt)
            .await
            .ok_or_else(|| "no service")?;
        service
            .request(rpc::RequestSearch { content })
            .await
            .map(|mut dl| {
                dl.list
                    .drain(..)
                    .map(|d| Item {
                        obj_id: d.obj_id,
                        name: d.name,
                        hint: d.hint,
                        icon: None,
                    })
                    .collect()
            })
            .map_err(|e| format!("search failed: {}", e))
    }
    pub async fn submit(&mut self, obj_id: u32, hint: Option<String>) -> Result<(), String> {
        let service = self
            .rpc
            .get_ext(&self.prompt)
            .await
            .ok_or_else(|| "no service")?;
        service
            .request(rpc::RequestSubmit { obj_id, hint })
            .await
            .map(|_| ())
            .map_err(|e| format!("submit failed: {}", e))
    }
}