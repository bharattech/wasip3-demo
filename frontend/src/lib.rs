mod bindings {
    wit_bindgen::generate!({
        path: "../wit",
        world: "frontend",
        generate_all,
    });
}

use bindings::exports::wasi::http::handler::Guest as Handler;
use bindings::wasi::http::types::{ErrorCode, Fields, Request, Response};

struct Component;

impl Handler for Component {
    async fn handle(_request: Request) -> Result<Response, ErrorCode> {
        //println!("get_path_with_query: {:?}", request.get_path_with_query());

        println!("fronend call : middleware::invoke()");

        let headers = Fields::new();
        //let body_bytes = b"hello from p3".to_vec();
        // Call the middleware component through the linked interface
        let result = bindings::wasmcloud::p3_multi_comps_demo::middleware::invoke();

        let (status, body_bytes) = match result {
            Ok(test) => (200u16, test.into_bytes()),
            Err(e) => (500, format!("middleware error: {e}").into_bytes()),
        };

        let (mut tx, rx) = bindings::wit_stream::new();
        let (trailers_tx, trailers_rx) = bindings::wit_future::new(|| todo!());

        wit_bindgen::spawn(async move {
            tx.write_all(body_bytes).await;
            drop(tx);
            let _ = trailers_tx.write(Ok(None)).await;
        });

        let (response, _result) = Response::new(headers, Some(rx), trailers_rx);
        response
            .set_status_code(status)
            .map_err(|()| ErrorCode::InternalError(Some("failed to set status".into())))?;
        Ok(response)
    }
}

bindings::export!(Component with_types_in bindings);
