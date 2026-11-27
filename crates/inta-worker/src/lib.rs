//! Offline shell behavior implemented in Rust. The published sw.js is its
//! generated WebAssembly host adapter, not a second implementation.
#[cfg(target_arch = "wasm32")]
mod worker {
    use wasm_bindgen::{prelude::*, JsCast};
    use wasm_bindgen_futures::{future_to_promise, JsFuture};
    use web_sys::{
        Cache, ExtendableEvent, FetchEvent, Response, ResponseInit, ServiceWorkerGlobalScope,
    };
    const CACHE: &str = "inta-rust-shell-v2";
    fn scope() -> ServiceWorkerGlobalScope {
        js_sys::global().unchecked_into()
    }
    async fn cache() -> Result<Cache, JsValue> {
        JsFuture::from(scope().caches()?.open(CACHE))
            .await?
            .dyn_into()
    }
    #[wasm_bindgen(start)]
    pub fn start() -> Result<(), JsValue> {
        let install = Closure::<dyn FnMut(ExtendableEvent)>::new(|event: ExtendableEvent| {
            let p = future_to_promise(async {
                let cache = cache().await?;
                let files = js_sys::Array::new();
                for path in [
                    "./",
                    "./index.html",
                    "./manifest.webmanifest",
                    "./inta-app-icon.svg",
                ] {
                    files.push(&path.into());
                }
                JsFuture::from(cache.add_all_with_str_sequence(&files)).await?;
                let _ = scope().skip_waiting();
                Ok(JsValue::UNDEFINED)
            });
            let _ = event.wait_until(&p);
        });
        scope().add_event_listener_with_callback("install", install.as_ref().unchecked_ref())?;
        install.forget();
        let activate = Closure::<dyn FnMut(ExtendableEvent)>::new(|event: ExtendableEvent| {
            let p = future_to_promise(async {
                let storage = scope().caches()?;
                let keys = js_sys::Array::from(&JsFuture::from(storage.keys()).await?);
                for key in keys.iter() {
                    if let Some(key) = key.as_string() {
                        if key.starts_with("inta-") && key != CACHE {
                            JsFuture::from(storage.delete(&key)).await?;
                        }
                    }
                }
                JsFuture::from(scope().clients().claim()).await?;
                Ok(JsValue::UNDEFINED)
            });
            let _ = event.wait_until(&p);
        });
        scope().add_event_listener_with_callback("activate", activate.as_ref().unchecked_ref())?;
        activate.forget();
        let fetch = Closure::<dyn FnMut(FetchEvent)>::new(|event: FetchEvent| {
            let request = event.request();
            if request.method() != "GET" {
                return;
            }
            let Ok(url) = web_sys::Url::new(&request.url()) else {
                return;
            };
            if url.origin() != scope().location().origin() {
                return;
            }
            let navigation = request.mode() == web_sys::RequestMode::Navigate;
            let path = url.pathname();
            let shell = navigation
                || path.ends_with("/index.html")
                || path.ends_with("/manifest.webmanifest")
                || path.ends_with("/inta-app-icon.svg");
            if !shell {
                return;
            }
            let promise = future_to_promise(async move {
                let cache = cache().await?;
                match JsFuture::from(scope().fetch_with_request(&request)).await {
                    Ok(result) => {
                        let response: Response = result.clone().dyn_into()?;
                        if response.ok() {
                            let copy = response.clone()?;
                            let key = if navigation {
                                "./index.html".to_string()
                            } else {
                                request.url()
                            };
                            let _ = JsFuture::from(cache.put_with_str(&key, &copy)).await;
                        }
                        Ok(result)
                    }
                    Err(_) => {
                        let result = JsFuture::from(cache.match_with_request(&request)).await?;
                        if !result.is_undefined() {
                            return Ok(result);
                        }
                        if navigation {
                            let fallback =
                                JsFuture::from(cache.match_with_str("./index.html")).await?;
                            if !fallback.is_undefined() {
                                return Ok(fallback);
                            }
                        }
                        let init = ResponseInit::new();
                        init.set_status(503);
                        let response=Response::new_with_opt_str_and_init(Some("INTA is offline and this page has not been saved on this device. Open a saved offline HTML copy or reconnect once."),&init)?;
                        Ok(response.into())
                    }
                }
            });
            let _ = event.respond_with(&promise);
        });
        scope().add_event_listener_with_callback("fetch", fetch.as_ref().unchecked_ref())?;
        fetch.forget();
        Ok(())
    }
}
