use crate::rpc;
use crate::ui;
use rpc::{Input, Server, SubmitHint};
use ui::FromServer;
use ui::ToServer;
pub trait Convert {
    fn convert(self) -> Self;
}

pub async fn convert(ui: ToServer, ser: &mut Server) -> Result<FromServer, ui::Error> {
    match ui {
        ToServer::Connect(endpoint) => match ser.connet(endpoint).await {
            Ok(_) => Ok(FromServer::Connected),
            Err(e) => Err(ui::Error::from(e)),
        },
        ToServer::Search { prompt, content } => ser
            .get_ext(&prompt)
            .await
            .map_err(|e| ui::Error::from(e))?
            .request(Input { content })
            .await
            .map_err(|e| ui::Error::from(e))
            .map(|mut r| {
                FromServer::Search(
                    r.drain(..)
                        .map(|d| ui::Item {
                            obj_id: d.obj_id,
                            name: d.name,
                            arg_hint: d.hint,
                            icon: None,
                        })
                        .collect(),
                )
            }),
        ToServer::Submit {
            prompt,
            obj_id,
            hint,
        } => ser
            .get_ext(&prompt)
            .await
            .map_err(|e| ui::Error::from(e))?
            .request(SubmitHint { obj_id, hint })
            .await
            .map_err(|e| ui::Error::from(e))
            .map(|_| FromServer::Submit),
    }
}
