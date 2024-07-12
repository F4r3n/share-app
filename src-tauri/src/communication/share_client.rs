use crate::communication::message::{Payload, ResponseMessage};
use anyhow::Error;
use futures::prelude::*;
use irc::client::prelude::*;
use irc::proto::FormattedStringExt;
use std::fs::File;
use std::io::Write;
use tauri::Manager;

pub struct IRC {
    pub client: Option<irc::client::Client>,
    pub channel: String,
    pub log_file: Option<File>,
}

impl IRC {
    pub fn send_message(&self, message: &str, channel: &str) -> Result<(), anyhow::Error> {
        let mut current_channel = String::from(channel);
        if current_channel.is_empty() {
            current_channel = self.channel.to_owned();
        }

        match self
            .client
            .as_ref()
            .unwrap()
            .send_privmsg(current_channel, String::from(message.to_owned()))
        {
            Ok(()) => Ok(()),
            Err(e) => Err(anyhow::Error::msg(e.to_string())),
        }
    }

    pub fn get_users(&self) -> Option<Vec<irc::client::data::User>> {
        return self.client
        .as_ref()
        .unwrap()
        .list_users(&self.channel);
    }

    pub fn send_quit(&self, message: &str) -> Result<(), String> {
        if let Some(client) = self.client.as_ref() {
            return match client
            .send_quit(message) {
                Ok(()) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        }
        else {
            Err("No client".into())
        }

    }

    pub fn send_irc_command(&self, command: &str, args: Vec<String>) -> Result<(), String> {
        match self.client
        .as_ref()
        .unwrap()
        .send(Command::Raw(String::from(command), args)) {
            Ok(()) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn read(&mut self, window: tauri::Window) -> Result<(), anyhow::Error> {
        let stream: irc::client::ClientStream = self.client
        .as_mut()
        .unwrap()
        .stream()?;

        let mut log: Option<File> = None;
        if let Some(log_file) = &self.log_file {
            log = Some(log_file.try_clone().unwrap());
        }
        tauri::async_runtime::spawn(async {
            match irc_read(&log.unwrap(), window, stream).await {
                Ok(()) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        });

        Ok(())
    }
}

pub fn write_in_log(mut file: &File, nick_name: &str, message: &str) {
    let utc: chrono::DateTime<chrono::Utc> = chrono::Utc::now();
    match write!(
        file,
        "{}",
        format!("{} {}: {}\n", utc, String::from(nick_name), message)
    ) {
        Ok(()) => (),
        Err(_) => (),
    }
}

async fn irc_read(
    file: &File,
    window: tauri::Window,
    mut stream: irc::client::ClientStream,
) -> Result<(), anyhow::Error> {
    loop {
        let result = stream.next().await.transpose();

        let m = match result {
            Ok(m) => m,
            Err(e) => {
                window.emit("irc-recieved", Payload {
                    content: e.to_string(),
                    nick_name: String::from(""),
                    command: String::from("INTERNAL_ERROR"),
                    channel: String::from(""),
                    response: None,
                })?;
                return Err(e.into())},
        };

        let message = match m {
            Some(m) => m,
            None => {
                window.emit("irc-recieved", Payload {
                    content: String::from("No Messages"),
                    nick_name: String::from(""),
                    command: String::from("INTERNAL_ERROR"),
                    channel: String::from(""),
                    response: None,
                })?;
                return Err(Error::msg("No messages"))},
        };

        print!("MESSAGE {}", message);

        let mut pay_load = Payload {
            content: String::from(""),
            nick_name: String::from(""),
            command: String::from(""),
            channel: String::from(""),
            response: None,
        };
        match message.command {
            Command::PING(_, _) => {
                pay_load.command = String::from("PING");
                pay_load.content =
                    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
                        Ok(n) => n.as_secs().to_string(),
                        Err(_e) => String::from("0"),
                    }
            }
            Command::PONG(_, _) => {
                pay_load.command = String::from("PONG");
                pay_load.content =
                    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
                        Ok(n) => n.as_secs().to_string(),
                        Err(_e) => String::from("0"),
                    }
            }
            Command::PRIVMSG(ref target, ref msg) => {
                pay_load.command = String::from("PRIVMSG");
                pay_load.channel = String::from(target);
                if let Some(nick_name) = message.source_nickname() {
                    pay_load.nick_name = String::from(nick_name);
                }
                pay_load.content = msg.as_str().to_string();
                write_in_log(file, pay_load.nick_name.as_str(), pay_load.content.as_str());
            }
            Command::NOTICE(ref _target, ref msg) => {
                pay_load.command = String::from("NOTICE");
                if let Some(nick_name) = message.source_nickname() {
                    pay_load.nick_name = String::from(nick_name);
                }
                pay_load.content = String::from(msg.as_str().to_string().strip_formatting());
            }
            Command::Response(response, ref msg) => {
                pay_load.command = String::from("RESPONSE");
                pay_load.response = Some(ResponseMessage {
                    kind: response as u16,
                    content: msg.clone(),
                });
            }
            Command::QUIT(ref comment) => {
                pay_load.command = String::from("QUIT");

                if let Some(comment) = comment {
                    pay_load.content = comment.to_string();
                }

                if let Some(nick_name) = message.source_nickname() {
                    pay_load.nick_name = nick_name.to_owned();
                }
            }
            Command::PART(ref _channel, ref comment) => {
                pay_load.command = String::from("QUIT");

                if let Some(comment) = comment {
                    pay_load.content = comment.to_string();
                }

                if let Some(nick_name) = message.source_nickname() {
                    pay_load.nick_name = nick_name.to_owned();
                }
            }
            Command::JOIN(ref _channel, ref _chan_keys, ref name) => {
                pay_load.command = String::from("JOIN");
                if let Some(name) = name {
                    pay_load.content = name.to_owned();
                } else {
                    if let Some(nick_name) = message.source_nickname() {
                        pay_load.nick_name = nick_name.to_owned();
                    }
                }
            }
            Command::TOPIC(ref _channel, ref topic) => {
                pay_load.command = String::from("TOPIC");
                if let Some(topic) = topic {
                    pay_load.content = topic.to_string();
                }

                if let Some(nick_name) = message.source_nickname() {
                    pay_load.nick_name = nick_name.to_owned();
                }
            }
            Command::NAMES(ref _channel, ref _target) => {}
            Command::ERROR(ref err) => {
                pay_load.command = String::from("ERROR");
                pay_load.content = err.to_string();
            }
            Command::NICK(ref new_nick_name) => {
                pay_load.command = String::from("NICK");
                if let Some(nick_name) = message.source_nickname() {
                    pay_load.nick_name = nick_name.to_owned();
                }
                pay_load.content = new_nick_name.to_owned();
            }
            _ => (),
        }

        window.emit("irc-recieved", pay_load)?;
    }
}

pub async fn irc_login(
    nick_name: String,
    server: String,
    channel: String,
    password: String,
) -> Result<irc::client::Client, irc::error::Error> {
    let mut split = server.split(":");
    let server = split.next().unwrap_or("127.0.0.1").to_string();
    let port_number = split.last().unwrap_or("6697").parse().unwrap_or(6697);
    println!("Try to connect to {} {} {} {} {}", server, port_number, channel, nick_name, password);
    let config = Config {
        nickname: Some(nick_name),
        server: Some(server),
        channels: vec![channel],
        password: Some(password),
        port: Some(port_number),
        ping_time: Some(30),
        ping_timeout: Some(300),
        use_tls: Some(port_number == 6697),
        ..Default::default()
    };

    let client = Client::from_config(config).await?;

    client.identify()?;

    return Ok(client);
}
