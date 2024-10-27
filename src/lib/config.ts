import { invoke } from '@tauri-apps/api/core'

type ConnectionConfig = {
    nick_name: string;
    server: string;
    channel: string;
    password: string;
}

export type UploadImageConfig = {
    url_post: string;
    url_get: string
}

export type CompletionConfig = {
    url: string;
    token: string;
    triggers: string[];
}

type Setting = {
    connection_config: ConnectionConfig;
    upload_image?: UploadImageConfig;
    completion?: CompletionConfig;
}

class Config {

    config: Setting
    constructor() {
        this.config = {} as Setting;
        this.config.connection_config = {
            nick_name: "",
            server: "",
            channel: "",
            password: ""
        } as ConnectionConfig

        this.config.upload_image = {
            url_post: "",
            url_get: "",
        } as UploadImageConfig

        this.config.completion = {
            url: "",
            token: "",
            triggers: []
        } as CompletionConfig

    }

    setConnectionSettings(inNickName: string, inServer: string, inChannel: string, inPassword: string) {
        this.config.connection_config = {
            nick_name: inNickName,
            server: inServer,
            channel: inChannel,
            password: inPassword
        }
    }

    setUploadImageConfig(option: UploadImageConfig) {
        this.config.upload_image = option;
    }
    setCompletionConfig(option: CompletionConfig) {
        console.log(option)
        this.config.completion = option;
    }

    getConnectionConfig(): ConnectionConfig {
        return this.config.connection_config;
    }

    getUploadImageConfig(): UploadImageConfig|undefined {
        return this.config.upload_image;
    }

    getCompletionConfig(): CompletionConfig|undefined {
        return this.config.completion;
    }

    getConfig() {
        return this.config;
    }

    setConfig(inConfig: Setting) {
        this.config = inConfig;
    }

    async read() {
        this.config = await invoke("load_settings") as Setting;
        console.log("Read",this.config)
    }

    async write() {
        console.log("Write",this.config)
        await invoke("save_settings", { settings: this.config })
    }
}



export const config = new Config()

