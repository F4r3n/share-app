import { configDir, join, BaseDirectory } from '@tauri-apps/api/path';
import { readTextFile, writeTextFile, createDir } from '@tauri-apps/api/fs';
import { invoke } from '@tauri-apps/api'
import { load, dump } from "js-yaml";

type ConnectionConfig = {
    nickName : string;
    server : string;
    channel : string;
    password : string;
}

type UploadImageConfig = {
    url_post : string;
    url_get : string
}

type Setting = {
    connectionConfig : ConnectionConfig;
    uploadImage : UploadImageConfig;
}

class Config {

    config : Setting
    constructor() {
        this.config = {} as Setting;
        this.config.connectionConfig  = {
            nickName: "",
            server: "",
            channel: "",
            password: ""
        } as ConnectionConfig

        this.config.uploadImage = {
            url_post: "",
            url_get: "",
        } as UploadImageConfig

    }
    
    async init() {

    }

    setConnectionSettings(inNickName : string, inServer : string, inChannel : string, inPassword : string) {
        this.config.connectionConfig  = {
            nickName: inNickName,
            server: inServer,
            channel: inChannel,
            password: inPassword
        }
    }

    getConnectionConfig () : ConnectionConfig{
        return this.config.connectionConfig;
    }

    async _getConfigDir() : Promise<string> {
        return invoke("get_config_dir_command")
    }
    
    async _getConfigPath() {
        return await join(await this._getConfigDir(), '.config.txt');
    }
    
    
    getConfig() {
        return this.config;
    }
    
    setConfig(inConfig : Setting) {
        this.config = inConfig;
    }
    
    async read() {
        let path = await this._getConfigPath();
        try {
            let content = await readTextFile(path, {dir:BaseDirectory.Config});
            let data = load(content);
            if(data.hasOwnProperty("connectionConfig"))
                this.config.connectionConfig = data["connectionConfig"]
            if(data.hasOwnProperty("uploadImage"))
                this.config.uploadImage = data["uploadImage"]
        }catch(e) {
            console.error(e);
        }
    }
    
    async write() {
        let path = await this._getConfigPath();  
        try {
            createDir(await this._getConfigDir(),  { recursive: true })
    
            await writeTextFile(path, dump( this.config, {
                flowLevel: 3,
                styles: {
                  '!!int'  : 'hexadecimal',
                  '!!null' : 'camelcase'
                }
              }));
        }catch(e) {
            console.log(e);
        }
    
        return ""
    }
}



export const config = new Config()

