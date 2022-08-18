#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use irc::{client::{prelude::*}};
use futures::{prelude::*, lock::Mutex};

#[derive(Clone, serde::Serialize)]
struct Response {
  kind : u8,
  content: Vec<String>
}

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  content: String,
  nick_name: String,
  command: String,
  response : Option<Response>
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

  pub fn send_message(&mut self, message : &str) {
    self.client.as_mut().unwrap().send_privmsg(&self.channel, String::from(message.to_owned()));
  }

  pub fn get_users(& mut self)-> Option<Vec<irc::client::data::User>> {
    return self.client.as_mut().unwrap().list_users(&self.channel);
  }

}


pub async fn irc_read(window: tauri::Window, mut stream : irc::client::ClientStream) -> Result<(), irc::error::Error> {
  while let Some(message) = stream.next().await.transpose()? {
     print!("{}", message);

     let mut pay_load = Payload{content : String::from(""),
                                    nick_name : String::from(""),
                                    command : String::from(""), 
                                    response : None
    };
     match message.command {
      Command::PRIVMSG(ref _target, ref msg) => {
        pay_load.command = String::from("PRIVMSG");
        if let Some(nick_name) = message.source_nickname() {
          pay_load.nick_name = String::from(nick_name);
        }
        pay_load.content = msg.to_owned();

      },
      Command::NOTICE(ref _target, ref msg) => {
        pay_load.command = String::from("NOTICE");
        pay_load.content = msg.to_owned();

      },
      Command::Response(ref response, ref msg) => {
        pay_load.command = String::from("RESPONSE");
        pay_load.response = Some(Response{kind:response.to_owned() as u8, content: msg.clone()});

      },
      Command::QUIT(Some(comment)) => {
        pay_load.command = String::from("QUIT");
        pay_load.content = comment;
      },
      Command::JOIN(ref _channel, _chanKeys, Some(name) ) => {
        pay_load.command = String::from("JOIN");
        pay_load.content = name;
      },
      Command::TOPIC(_channel, Some(topic)) => {
        pay_load.command = String::from("TOPIC");
        pay_load.content = topic;
        
      },
      Command::NAMES(_channel, target) => {
        print!("{}", target.unwrap());
      },
      _ =>()
     }
     window.emit("irc-recieved", pay_load).unwrap();

 }

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

#[derive(Clone, serde::Serialize)]
struct TEST {
  message: String,
}

#[tauri::command]
fn connect(window: tauri::Window, irc : tauri::State<'_, IRCState>) {

  let mut state_guard = irc.0.try_lock().expect("ERROR");

  let stream = state_guard.client.as_mut().unwrap().stream();
  match stream {
      Ok(s) => {
        tauri::async_runtime::spawn(async {
          irc_read(window, s).await;
      
        });
      },
      Err(e) => ()
  }

  
}

#[tauri::command]
fn loggin(nick_name : &str, server : &str, channel : &str, password : &str, irc : tauri::State<'_, IRCState>)->Result<(), ()> {
  let mut state_guard = irc.0.try_lock().expect("ERROR");
  println!("{}{}{}", nick_name, server, channel);
  state_guard.channel = channel.to_owned();
  state_guard.client = tauri::async_runtime::block_on(async {
    let client = match irc_login(nick_name, server, channel, password).await {
        Ok(client) => Some(client),
        Err(e) => panic!("{}", e),
    };
    return client;
  });

  Ok(())
}

#[tauri::command]
fn send_message( message : &str, irc : tauri::State<'_, IRCState>) {
  let mut state_guard = irc.0.try_lock().expect("ERROR");
  state_guard.send_message(message);
}

#[tauri::command]
fn get_users(irc : tauri::State<'_, IRCState>) -> Vec<User> {
  let mut state_guard = irc.0.try_lock().expect("ERROR");
  let users = state_guard.get_users();
  let mut js_users : Vec<User> = Vec::new();
  if let Some(users) = users {
    for user in users.iter() {
      js_users.push(User{nick_name:user.get_nickname().to_owned(), user_mode:(user.highest_access_level() as u8)})
    }
  }
  println!("SEND");
  return js_users;
}



fn main() {

  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![loggin, connect, send_message, get_users])
  .manage(IRCState(Mutex::new(IRC{client : None, channel : String::from("")})))
  .setup(|app| {

    Ok(())
  })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
