#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use irc::{client::{prelude::*}};
use irc::proto::colors;
use futures::{prelude::*, lock::Mutex, future::OrElse};

#[derive(Clone, serde::Serialize)]
struct ResponseMessage {
  kind : u16,
  content: Vec<String>
}

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  content: String,
  nick_name: String,
  command: String,
  channel: String,
  response : Option<ResponseMessage>
}
#[derive(Clone, serde::Serialize)]
enum EVENT {
  Quit,
  ErrorConnection
}

#[derive(Clone, serde::Serialize)]
struct Event {
  kind : EVENT
}

#[derive(Clone, serde::Serialize)]
struct User {
  nick_name: String,
  user_mode : u8,
}

pub struct IRCState(pub Mutex<IRC>);
pub struct IRC {
  client : Option<irc::client::Client>,
  channel : String
}

impl IRC {

  pub fn send_message(&self, message : &str, channel :&str) -> Result<(), String> {
    
    let mut current_channel = String::from(channel);
    if current_channel.is_empty() {
      current_channel = self.channel.to_owned();
    }
    match self.client.as_ref().unwrap().send_privmsg(current_channel, 
    String::from(message.to_owned())) {
      Ok(()) => Ok(()),
      Err(e) => Err(e.to_string())
    }
  }

  pub fn get_users(& self)-> Option<Vec<irc::client::data::User>> {
    return self.client.as_ref().unwrap().list_users(&self.channel);
  }
  pub fn send_quit(& self, message : &str)-> Result<(), String> {
    match self.client.as_ref().unwrap().send_quit(message) {
      Ok(()) => Ok(()),
      Err(e) => Err(e.to_string())
    }
  }

  pub fn send_irc_command(& self, command : &str, args : Vec<String>)-> Result<(), String> {    
    match self.client.as_ref().unwrap().send(Command::Raw(String::from(command), args)) {
      Ok(()) => Ok(()),
      Err(e) => Err(e.to_string())
    }
  }
}


pub async fn irc_read(window: tauri::Window, mut stream : irc::client::ClientStream) -> Result<(), irc::error::Error> {
  
  println!("READ");
  while let Some(message) = stream.next().await.transpose()? {
     print!("{}", message);
     let mut pay_load = Payload{content : String::from(""),
                                    nick_name : String::from(""),
                                    command : String::from(""),
                                    channel : String::from(""),
                                    response : None
    };
     match message.command {
      Command::PRIVMSG(ref target, ref msg) => {
        pay_load.command = String::from("PRIVMSG");
        pay_load.channel = String::from(target);
        if let Some(nick_name) = message.source_nickname() {
          pay_load.nick_name = String::from(nick_name);
        }
        pay_load.content = irc::proto::FormattedStringExt::strip_formatting(msg.as_str()).to_string();
      },
      Command::NOTICE(ref _target, ref msg) => {
        pay_load.command = String::from("NOTICE");
        if let Some(nick_name) = message.source_nickname() {
          pay_load.nick_name = String::from(nick_name);
        }
        pay_load.content = irc::proto::FormattedStringExt::strip_formatting(msg.as_str()).to_string();
      },
      Command::Response(response, ref msg) => {
        pay_load.command = String::from("RESPONSE");
        pay_load.response = Some(ResponseMessage{kind:response as u16, content: msg.clone()}); 
      },
      Command::QUIT(ref comment) => {
        pay_load.command = String::from("QUIT");

        if let Some(comment) = comment {
          pay_load.content = comment.to_string();
        }
        
        if let Some(nick_name) = message.source_nickname() {
          pay_load.nick_name = nick_name.to_owned();
        }
        
      },
      Command::JOIN(ref _channel, ref _chan_keys, ref name) => {
        pay_load.command = String::from("JOIN");
        if let Some(name) = name {
          pay_load.content = name.to_owned();
        }
        else {
          if let Some(nick_name) = message.source_nickname() {
            pay_load.nick_name = nick_name.to_owned();
          }
        }
      },
      Command::TOPIC(ref _channel, ref topic) => {
        pay_load.command = String::from("TOPIC");
        if let Some(topic) = topic {
          pay_load.content = topic.to_string();
        }

        if let Some(nick_name) = message.source_nickname() {
          pay_load.nick_name = nick_name.to_owned();
        }
        
      },
      Command::NAMES(_channel, target) => {
      },
      Command::ERROR(err) => {
        pay_load.command = String::from("ERROR");
        pay_load.content = err.to_string();
      },
      _ =>()
     }
     window.emit("irc-recieved", pay_load);
 }

 println!("QUITTTTT");

 Ok(())
}

pub async fn irc_login(nick_name : &str, server : &str, channel : &str, password : &str) -> Result<irc::client::Client, irc::error::Error>{
  let config = Config {
    nickname: Some(nick_name.to_owned()),
    server: Some(server.to_owned()),
    channels: vec![channel.to_owned()],
    password:Some(password.to_owned()),
    port: Some(6697 as u16),
    ..Default::default()
  };


  let client = Client::from_config(config).await?;
  client.identify()?;

  return Ok(client);
}


#[tauri::command]
fn read_messages(window: tauri::Window, irc : tauri::State<'_, IRCState>) -> Result<(), String>{

  let mut state_guard = irc.0.try_lock().expect("ERROR");

  let stream = state_guard.client.as_mut().unwrap().stream();
  match stream {
      Ok(s) => {
        tauri::async_runtime::spawn(async {
          match irc_read(window, s).await {
            Ok(()) => Ok(()),
            Err(e) => Err(e.to_string())
          }
        });
        Ok(())
      },
      Err(e)=>Err(e.to_string())
  }
}

#[tauri::command]
fn loggin(nick_name : &str, server : &str, channel : &str, password : &str, irc : tauri::State<'_, IRCState>)->Result<(), String> {
  let mut state_guard = irc.0.try_lock().expect("ERROR");
  if state_guard.client.is_none() {
    println!("Connect");
    state_guard.channel = channel.to_owned();
    state_guard.client = tauri::async_runtime::block_on(async {
      let client = match irc_login(nick_name, server, channel, password).await {
          Ok(client) => Some(client),
          Err(_e) => None
      };
    return client;
    });
  }

  match &state_guard.client {
    Some(_) => Ok(()),
    _ => Err(String::from("No client"))
  }
}

#[tauri::command]
fn send_message( message : &str, channel : &str, irc : tauri::State<'_, IRCState>) -> Result<(), String> {
  let state_guard = irc.0.try_lock().expect("ERROR");
  state_guard.send_message(message, channel)
}

#[tauri::command]
fn get_users(irc : tauri::State<'_, IRCState>) -> Vec<User> {
  let state_guard = irc.0.try_lock().expect("ERROR");
  let users = state_guard.get_users();
  let mut js_users : Vec<User> = Vec::new();

  if let Some(users) = users {
    for user in users.iter() {
      js_users.push(User{nick_name:user.get_nickname().to_owned(), user_mode:(user.highest_access_level() as u8)})
    }
  }
  return js_users;
}

#[tauri::command]
fn disconnect(window: tauri::Window, message : &str, shall_send_message : bool, wrong_identifier : bool, irc : tauri::State<'_, IRCState>) -> Result<(), String>
{
  let mut client = irc.0.try_lock().expect("ERROR");
  if shall_send_message {
    let result : Result<(), String> = match client.send_quit(message) {
      Ok(())=>Ok(()),
      Err(e)=>Err(e)
    };
  }

  client.client = None;
  if wrong_identifier {
    window.emit("irc-event", Event{kind:EVENT::ErrorConnection});
  }
  else {
    window.emit("irc-event", Event{kind:EVENT::Quit});
  }

  Ok(())
}


#[tauri::command]
fn send_irc_command(command : &str, args : Vec<String>, irc : tauri::State<'_, IRCState>) -> Result<(), String>
{
  let client = irc.0.try_lock().expect("ERROR");
  client.send_irc_command(command, args)
}

fn main() {

  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![
    loggin, 
    read_messages, 
    send_message,
    disconnect,
    send_irc_command,
    get_users])
  .manage(IRCState(Mutex::new(IRC{client : None, channel : String::from("")})))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
