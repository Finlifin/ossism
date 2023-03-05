// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use async_openai::Client;

use async_openai::types::{
    ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role,
};

#[tauri::command]
fn greet(name: String) -> String {
    String::from("was invoked")
    // format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn request_gpt(question: String, sys_role: String) -> String {
    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content(sys_role)
                .build()
                .unwrap(),
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(question)
                .build()
                .unwrap(),
        ])
        .build()
        .unwrap();

    let response = client.chat().create(request).await.unwrap();
    println!("{:#?}", response);
    response.choices.get(0).unwrap().message.content.to_owned()
    // for choice in response.choices {
    //     println!("{:#?}", choice);
    // }
    // "你好".to_string()
}

// #[tauri::command]
// async fn just(q: &str, s: &str) {
//     request_gpt(q, s).await;
// }

#[tokio::main]
async fn main() {
    println!("{}", request_gpt("你好".to_owned(), "你是中枢Ordis".to_owned()).await);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, request_gpt])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
