#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod communication;
mod util;

use crate::communication::share_client;
use base64::Engine;
use communication::share_client::IrcClient;
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
    #[error("No client connected")]
    NoClient,
}

#[derive(Clone, serde::Serialize)]
struct User {
    nick_name: String,
    user_mode: u8,
}

pub struct IRCState(pub Mutex<share_client::IrcState>);

#[tauri::command]
fn read_messages(window: tauri::Window, irc: tauri::State<'_, IRCState>) -> Result<(), String> {
    let mut state = irc.0.try_lock().unwrap();
    if let Some(client) = &mut state.client {
        client
            .read(window)
            .map_err(|e| anyhow::Error::msg(e).to_string())?
    }
    Ok(())
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

        if state_guard.client.is_none() {
            println!("Connect");

            let channel = channel.to_owned();
            let file_format = format!("log_{}_{}.txt", &server, &channel);

            channel.clone_into(&mut state_guard.channel);
            let client = share_client::irc_login(
                nick_name.to_owned(),
                server.to_owned(),
                channel,
                password.to_owned(),
            )
            .await
            .map_err(|e| e.to_string())?;
            let config_dir = create_config_dir(app_handle);
            let log_file = config_dir
                .map(|file| {
                    OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(file.join(file_format))
                })
                .map_err(|e| e.to_string())?
                .map_err(|e| anyhow::anyhow!("Error opening log file: {}", e).to_string())?;
            state_guard.client = Some(IrcClient { client, log_file });
        }

        println!("End connect");
        return Ok(());
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
    if let Some(client) = &state_guard.client {
        let nick_name = client.current_nickname();
        share_client::write_in_log(&client.log_file, nick_name, message)
            .map_err(|e| e.to_string())?;

        return client
            .send_message(message, channel)
            .map_err(|e| e.to_string());
    }
    Err(CommandError::NoClient.to_string())
}

#[tauri::command]
fn get_users(irc: tauri::State<'_, IRCState>) -> Result<Vec<User>, String> {
    let state_guard = irc.0.try_lock().ok_or(CommandError::Locked.to_string())?;
    let mut js_users: Option<Vec<User>> = None;

    if let Some(client) = &state_guard.client {
        js_users = client.get_users(&state_guard.channel).map(|users| {
            users
                .iter()
                .map(|user| User {
                    nick_name: user.get_nickname().to_owned(),
                    user_mode: (user.highest_access_level() as u8),
                })
                .collect::<Vec<User>>()
        });
    }

    Ok(js_users.unwrap_or_default())
}

#[tauri::command]
fn disconnect(
    message: &str,
    shall_send_message: bool,
    irc: tauri::State<'_, IRCState>,
) -> Result<(), String> {
    let mut state = irc.0.try_lock().ok_or(CommandError::Locked.to_string())?;
    if shall_send_message {
        state
            .client
            .as_ref()
            .map(|client| client.send_quit(message))
            .transpose()
            .map_err(|e| e.to_string())?;
    }
    state.client = None;
    Ok(())
}

#[tauri::command]
fn send_irc_command(message: &str, irc: tauri::State<'_, IRCState>) -> Result<(), String> {
    let state = irc.0.try_lock().ok_or(CommandError::Locked.to_string())?;
    if let Some(client) = &state.client {
        return client
            .send_irc_command(message, &state.channel)
            .map_err(|e| e.to_string());
    }
    Err(CommandError::NoClient.to_string())
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
    resp.text().map_err(|e| e.to_string())
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

#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_updater::UpdaterExt;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
async fn update(app: tauri::AppHandle) -> anyhow::Result<()> {
    if let Some(update) = app.updater()?.check().await? {
      let mut downloaded = 0;
  
      // alternatively we could also call update.download() and update.install() separately
      update.download_and_install(|chunk_length, _content_length| {
        downloaded += chunk_length;
      }, || {
      }).await?;
  
      println!("update installed");
      app.restart();
    }
  
    Ok(())
  }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
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
        .manage(IRCState(Mutex::new(share_client::IrcState {
            client: None,
            channel: String::from(""),
        })))
        .setup(|app| {
            #[cfg(not(any(target_os = "android", target_os = "ios")))]
            {
                let handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                  let _ = update(handle).await;
                });
            }
            Ok(())
          })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
