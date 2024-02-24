use i18n_embed::{WebLanguageRequester, fluent::fluent_language_loader};
use i18n_embed_fl::fl;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(RustEmbed)]
#[folder = "i18n"]
struct Localizations;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let language_loader = fluent_language_loader!();
    let requested_languages = WebLanguageRequester::requested_languages();
    let _result = i18n_embed::select(
        &language_loader, &Localizations, &requested_languages);

    let greet_input_ref = use_node_ref();

    let name = use_state(|| String::new());

    let greet_msg = use_state(|| String::new());
    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with(name2,
            move |_| {
                spawn_local(async move {
                    if name.is_empty() {
                        return;
                    }

                    let args = to_value(&GreetArgs { name: &*name }).unwrap();
                    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
                    let new_msg = invoke("greet", args).await.as_string().unwrap();
                    greet_msg.set(new_msg);
                });
            }
        );
    }

    let greet = {
        let name = name.clone();
        let greet_input_ref = greet_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            name.set(
                greet_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <main class="container">
            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt={
                        fl!(language_loader, "tauri-logo")
                    }/>
                </a>
                <a href="https://yew.rs" target="_blank">
                    <img src="public/yew.png" class="logo yew" alt={
                        fl!(language_loader, "yew-logo")
                    }/>
                </a>
            </div>

            <p>{fl!(language_loader, "logo-info")}</p>

            <p>
                {fl!(language_loader, "recommended-ide-setup")}{" "}
                <a href="https://code.visualstudio.com/" target="_blank">{
                    fl!(language_loader, "vs-code")
                }</a>
                {" "}{fl!(language_loader, "plus")}{" "}
                <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">{
                    fl!(language_loader, "tauri")
                }</a>
                {" "}{fl!(language_loader, "plus")}{" "}
                <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">{
                    fl!(language_loader, "rust-analyzer")
                }</a>
            </p>

            <form class="row" onsubmit={greet}>
                <input id="greet-input" ref={greet_input_ref} placeholder={
                    fl!(language_loader, "enter-a-name")
                } />
                <button type="submit">{fl!(language_loader, "greet")}</button>
            </form>

            <p><b>{ &*greet_msg }</b></p>
        </main>
    }
}
