import { configDir, join, BaseDirectory } from '@tauri-apps/api/path';
import { readTextFile, writeTextFile, createDir } from '@tauri-apps/api/fs';
import { load, dump } from "js-yaml"

type ConnectionConfig = {
    nickName : string;
    server : string;
    channel : string;
    password : string;
}

type Setting = {
    connectionConfig : ConnectionConfig
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
    }
    
    async init() {

    }

    setConnectionSettings(inNickName, inServer, inChannel, inPassword) {
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

    async _getConfigDir() {
        const configDirPath = await configDir();
        return await join(configDirPath, 'share-app');
    }
    
    async _getConfigPath() {
        return await join(await this._getConfigDir(), '.config.txt');
    }
    
    
    getConfig() {
        return this.config;
    }
    
    setConfig(inConfig) {
        this.config = inConfig;
    }
    
    async read() {
        let path = await this._getConfigPath();
        try {
            let content = await readTextFile(path, {dir:BaseDirectory.Config});
            let data = load(content);
            if(data.hasOwnProperty("connectionConfig"))
                this.config.connectionConfig = data["connectionConfig"]
            //this.config = data;
        }catch(e) {
            //console.log(e);
        }
    }
    
    async write() {
        let path = await this._getConfigPath();  
        try {
            createDir(await this._getConfigDir(),  { recursive: true })
            //createDir("share-app/test",  { dir:BaseDirectory.Config, recursive: true })
    
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

