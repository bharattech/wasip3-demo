use crate::wasi::logging::logging;


wit_bindgen::generate!({
    world: "component",
    generate_all
});

struct Component;

impl exports::wasmcloud::p3_multi_comps_demo::middleware::Guest for Component {
    fn invoke() -> Result<String, String> {
        
        logging::log(logging::Level::Debug, "middleware", "invoke");
        println!("middleware call : receiver::invoke()");
        let result = wasmcloud::p3_multi_comps_demo::receiver::invoke();
        
        match result {
            Ok(text) => Ok(text),
            Err(e) => Err(e),
        }

    }
}

export!(Component);