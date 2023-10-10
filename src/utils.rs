use crate::rpc;
use crate::ui;

pub fn convert_ui_to_server(ui: ui::ToServer) -> rpc::Request {
    match ui {
        ui::ToServer::Connect(endpoint) => rpc::Request::Connect(endpoint),
        ui::ToServer::Search { prompt, content } => rpc::Request::Search {
            prompt,
            input: rpc::Input { content },
        },
        ui::ToServer::Submit {
            prompt,
            obj_id,
            hint,
        } => rpc::Request::Submit {
            prompt,
            hint: rpc::SubmitHint { obj_id, hint },
        },
    }
}
pub fn convert_server_to_ui(server: rpc::Response) -> ui::FromServer {
    match server {
        rpc::Response::Connected => ui::FromServer::Connected,
        rpc::Response::Search(mut displays) => ui::FromServer::Search(
            displays
                .drain(..)
                .map(|d| ui::Item {
                    obj_id: d.obj_id,
                    name: d.name,
                    arg_hint: d.hint,
                    icon: None,
                })
                .collect(),
        ),
        rpc::Response::Submit => ui::FromServer::Submit,
        // rpc::Response::FillResult(fill) => {
        //     ui::FromServer::FillResult(fill)
        // }
    }
}
