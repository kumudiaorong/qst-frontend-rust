use crate::rpc;
use crate::ui;
use rpc::{RequestSearch, RequestSubmit, Server};
use ui::FromServer;
use ui::ToServer;
use xlog_rs::log;
pub trait Convert {
    fn convert(self) -> Self;
}

pub async fn convert(ui: ToServer, ser: &mut Server) -> Result<FromServer, ui::Error> {
    match ui {
        ToServer::Search { prompt, content } => {
            if let Some(ext) = ser.get_ext(&prompt).await {
                let mut r = ext
                    .request(RequestSearch { content })
                    .await
                    .map_err(|e| ui::Error::from(e))?;
                Ok(FromServer::Search(
                    r.list
                        .drain(..)
                        .map(|d| ui::Item {
                            obj_id: d.obj_id,
                            name: d.name,
                            arg_hint: d.hint,
                            icon: None,
                        })
                        .collect(),
                ))
            } else {
                Err(ui::Error::from("no such prompt"))
            }
        }
        ToServer::Submit {
            prompt,
            obj_id,
            hint,
        } => {
            if let Some(ext) = ser.get_ext(&prompt).await {
                ext.request(RequestSubmit { obj_id, hint })
                    .await
                    .map_err(|e| ui::Error::from(e))?;
                Ok(FromServer::Submit)
            } else {
                Err(ui::Error::from("no such prompt"))
            }
        }
    }
}
