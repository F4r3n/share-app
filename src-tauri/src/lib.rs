#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod communication;
mod util;

use crate::communication::share_client;
use base64::Engine;
use futures::lock::Mutex;
use std::fs::OpenOptions;

use util::app_path::create_config_dir;

use util::clipboard;

use settings::Settings;
use util::settings;

use thiserror::Error;

#[derive(Debug, Error)]
enum CommandError {
    #[error("Already locked")]
    Locked,
}

#[derive(Clone, serde::Serialize)]
struct User {
    nick_name: String,
    user_mode: u8,
}

pub struct IRCState(pub Mutex<share_client::IRC>);

#[tauri::command]
fn read_messages(window: tauri::Window, irc: tauri::State<'_, IRCState>) -> Result<(), String> {
    let mut state_guard = irc.0.try_lock().expect("ERROR");
    state_guard
        .read(window)
        .map_err(|e| anyhow::Error::msg(e).to_string())
}

#[tauri::command]
async fn loggin(
    app_handle: tauri::AppHandle,
    nick_name: &str,
    server: &str,
    channel: &str,
    password: &str,
    irc: tauri::State<'_, IRCState>,
) -> Result<(), String> {
    if let Some(state_guard) = irc.0.try_lock().as_mut() {
        println!("Loggin");

        if state_guard.client.is_some() {
            return Ok(());
        }
        let nick_name = nick_name.to_owned();
        let server = server.to_owned();
        let channel = channel.to_owned();
        let password = password.to_owned();
        let str = format!("log_{}_{}.txt", &server, &channel);
    
        if state_guard.client.is_none() {
            println!("Connect");
    
            state_guard.channel = channel.to_owned();
            state_guard.client = match share_client::irc_login(nick_name, server, channel, password).await {
                Ok(client) => Some(client),
                Err(_e) => None,
            }
        }
    
        println!("End connect");
    
        if state_guard.client.is_none() {
            println!("Cannot Connect");
        }
    
        if state_guard.client.is_some() {
            let config_dir = create_config_dir(app_handle);
            match config_dir {
                Ok(file) => {
                    state_guard.log_file = Some(
                        OpenOptions::new()
                            .write(true)
                            .append(true)
                            .create(true)
                            .open(file.join(str))
                            .unwrap(),
                    );
                }
                Err(e) => {
                    return Err(anyhow::Error::msg(e).to_string());
                }
            }
        }
    
        if state_guard.client.is_none() {
            println!("Cannot Connect");
        }
    
        return match &state_guard.client {
            Some(_) => Ok(()),
            _ => Err(String::from("No client")),
        };
    }
    Err(String::from("Already trying to connect"))
}

#[tauri::command]
fn send_message(
    message: &str,
    channel: &str,
    irc: tauri::State<'_, IRCState>,
) -> Result<(), String> {
    let state_guard = irc.0.try_lock().ok_or(CommandError::Locked.to_string())?;
    let nick_name = state_guard.client.as_ref().unwrap().current_nickname();
    if let Some(log_file) = &state_guard.log_file {
        share_client::write_in_log(log_file, nick_name, message).map_err(|e|e.to_string())?;
    }
    state_guard
        .send_message(message, channel)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_users(irc: tauri::State<'_, IRCState>) -> Result<Vec<User>, String> {
    let state_guard = irc.0.try_lock().ok_or(CommandError::Locked.to_string())?;
    let users = state_guard.get_users().map_err(|e|e.to_string())?;
    let mut js_users: Vec<User> = Vec::new();
    if let Some(users) = users {
        for user in users.iter() {
            js_users.push(User {
                nick_name: user.get_nickname().to_owned(),
                user_mode: (user.highest_access_level() as u8),
            })
        }
    }

    Ok(js_users)
}

#[tauri::command]
fn disconnect(
    message: &str,
    shall_send_message: bool,
    irc: tauri::State<'_, IRCState>,
) -> Result<(), String> {
    let mut client = irc.0.try_lock().ok_or(CommandError::Locked.to_string())?;
    if shall_send_message {
        client.send_quit(message).map_err(|e|e.to_string())?
    }
    client.client = None;
    Ok(())
}

#[tauri::command]
fn send_irc_command(
    command: &str,
    args: Vec<String>,
    irc: tauri::State<'_, IRCState>,
) -> Result<(), String> {
    let client = irc.0.try_lock().ok_or(CommandError::Locked.to_string())?;
    client.send_irc_command(command, args).map_err(|e|e.to_string())
}

#[tauri::command]
fn get_image_clipboard() -> Result<String, String> {
    clipboard::get_image_clipboard().map_err(|e| e.to_string())
}

#[tauri::command]
fn load_settings(app_handle: tauri::AppHandle) -> Result<settings::Settings, String> {
    settings::load_settings(app_handle).map_err(|e| e.to_string())
}

#[tauri::command]
fn save_settings(app_handle: tauri::AppHandle, settings: Settings) -> Result<(), String> {
    settings::save_settings(app_handle, &settings).map_err(|e| e.to_string())
}

#[tauri::command]
fn decode_base64(message: &str) -> Result<Vec<u8>, String> {
    let engine = base64::engine::general_purpose::STANDARD_NO_PAD;
    match engine.decode(message) {
        Ok(vector) => Ok(vector),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn upload_image(endpoint: &str, image_bytes: Vec<u8>) -> Result<String, String> {
    use reqwest::blocking::multipart;

    let form = multipart::Form::new().text("title", "").part(
        "upload",
        multipart::Part::bytes(image_bytes)
            .mime_str("image/png")
            .map_err(|e| e.to_string())?,
    );

    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(endpoint)
        .multipart(form)
        .send()
        .map_err(|e| e.to_string())?;
    dbg!(&resp);
    Ok(resp.text().map_err(|e| e.to_string())?)
}

use scraper::Html;
#[derive(Clone, serde::Serialize)]
struct MetaData {
    image_url: String,
    title: String,
    description: String,
    site: String,
    image_only: bool,
}

impl MetaData {
    pub fn new(in_html: Html) -> Self {
        let mut meta = MetaData {
            image_url: String::from(""),
            title: String::from(""),
            description: String::from(""),
            site: String::from(""),
            image_only: false,
        };
        use scraper::Selector;
        let selector = Selector::parse("head meta").unwrap();
        let s = in_html.select(&selector);
        for element in s {
            if let Some(property) = element.attr("property") {
                if let Some(content) = element.attr("content") {
                    match property {
                        "og:title" => {
                            meta.title = String::from(content);
                        }
                        "og:image" => {
                            meta.image_url = String::from(content);
                        }
                        "og:description" => {
                            meta.description = String::from(content);
                        }
                        "og:site" => {
                            meta.site = String::from(content);
                        }
                        _ => {}
                    }
                }
            }
        }
        meta
    }
}

#[tauri::command]
async fn get_url_preview(endpoint: &str) -> Result<MetaData, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get(endpoint)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let headers = resp.headers();
    let mut has_meta = false;

    if let Some(content) = headers.get("content-type") {
        has_meta = !content
            .to_str()
            .map_err(|e| e.to_string())?
            .starts_with("image");
    }
    let text = resp.text().await.map_err(|e| e.to_string())?;
    let document = Html::parse_document(text.as_str());
    if has_meta {
        Ok(MetaData::new(document))
    } else {
        Ok(MetaData {
            image_url: String::from(endpoint),
            title: String::from(""),
            description: String::from(""),
            site: String::from(""),
            image_only: true,
        })
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            loggin,
            read_messages,
            send_message,
            disconnect,
            send_irc_command,
            get_image_clipboard,
            decode_base64,
            load_settings,
            save_settings,
            get_users,
            upload_image,
            get_url_preview
        ])
        .manage(IRCState(Mutex::new(share_client::IRC {
            client: None,
            channel: String::from(""),
            log_file: None,
        })))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
