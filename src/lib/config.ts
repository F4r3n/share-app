import { invoke } from '@tauri-apps/api/core'

type ConnectionConfig = {
    nick_name : string;
    server : string;
    channel : string;
    password : string;
}

type UploadImageConfig = {
    url_post : string;
    url_get : string
}

type Setting = {
    connection_config : ConnectionConfig;
    upload_image : UploadImageConfig;
}

class Config {

    config : Setting
    constructor() {
        this.config = {} as Setting;
        this.config.connection_config  = {
            nick_name: "",
            server: "",
            channel: "",
            password: ""
        } as ConnectionConfig

        this.config.upload_image = {
            url_post: "",
            url_get: "",
        } as UploadImageConfig

    }

    setConnectionSettings(inNickName : string, inServer : string, inChannel : string, inPassword : string) {
        this.config.connection_config  = {
            nick_name: inNickName,
            server: inServer,
            channel: inChannel,
            password: inPassword
        }
    }

    setUploadImageConfig( option : UploadImageConfig) {
        this.config.upload_image = option;
    }

    getConnectionConfig () : ConnectionConfig{
        return this.config.connection_config; 
    }

    getUploadImageConfig () : UploadImageConfig{
        return this.config.upload_image; 
    }


    
    getConfig() {
        return this.config;
    }
    
    setConfig(inConfig : Setting) {
        this.config = inConfig;
    }
    
    async read() {
        this.config = await invoke("load_settings") as Setting;
        console.log("Read" + this.config) 
    }
    
    async write() {
        console.log("Write" + this.config)

        await invoke("save_settings", {settings: this.config})
    }
}



export const config = new Config()

