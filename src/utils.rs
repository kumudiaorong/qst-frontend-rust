use crate::rpc;
use crate::ui;
pub trait Convert {
    fn convert(self) -> Self;
}

pub async fn convert(ui: ui::ToServer, ser: &mut rpc::Server) -> Result<ui::FromServer, ui::Error> {
    match ui {
        ui::ToServer::Connect(endpoint) => match ser.connet(endpoint).await {
            Ok(_) => Ok(ui::FromServer::Connected),
            Err(e) => Err(ui::Error::from(e)),
        },
        ui::ToServer::Search { prompt, content } => ser
            .get_ext(&prompt)
            .await
            .map_err(|e| ui::Error::from(e))?
            .request(rpc::extension::Input { content })
            .await
            .map_err(|e| ui::Error::from(e))
            .map(|mut r| {
                ui::FromServer::Search(
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
        ui::ToServer::Submit {
            prompt,
            obj_id,
            hint,
        } => ser
            .get_ext(&prompt)
            .await
            .map_err(|e| ui::Error::from(e))?
            .request(rpc::extension::SubmitHint { obj_id, hint })
            .await
            .map_err(|e| ui::Error::from(e))
            .map(|_| ui::FromServer::Submit),
    }
}
// pub fn convert_server_to_ui(server: rpc::Response) -> ui::FromServer {
//     match server {
//         rpc::Response::Connected => ui::FromServer::Connected,
//         rpc::Response::Search(mut displays) => ui::FromServer::Search(
//             displays
//                 .drain(..)
//                 .map(|d| ui::Item {
//                     obj_id: d.obj_id,
//                     name: d.name,
//                     arg_hint: d.hint,
//                     icon: None,
//                 })
//                 .collect(),
//         ),
//         rpc::Response::Submit => ui::FromServer::Submit,
//         // rpc::Response::FillResult(fill) => {
//         //     ui::FromServer::FillResult(fill)
//         // }
//     }
// }
