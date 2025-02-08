import { invoke } from '@tauri-apps/api/core'
import { theme_mode, theme_name } from './theme.svelte';
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
    triggers: string; //separated by a space
}

export type Theme_Mode = "dark" | "light";
export type Theme_Name = "modern" | "cerberus";

export type Theme = {
    mode: Theme_Mode;
    name: Theme_Name;
}

export type Setting = {
    connection_config: ConnectionConfig;
    upload_image?: UploadImageConfig;
    completion?: CompletionConfig;
    theme?: Theme
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
            triggers: ""
        } as CompletionConfig

        this.config.theme = {
            mode: "light",
            name: "modern"
        } as Theme

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
        this.config.completion = option;
    }

    getConnectionConfig(): ConnectionConfig {
        return this.config.connection_config;
    }

    getUploadImageConfig(): UploadImageConfig | undefined {
        return this.config.upload_image;
    }

    getCompletionConfig(): CompletionConfig | undefined {
        return this.config.completion;
    }

    getConfig() {
        return this.config;
    }

    private _updateTheme() {
        if (this.config.theme) {
            console.log("New theme", this.config.theme.name);
            theme_mode.set(this.config.theme.mode);
            theme_name.set(this.config.theme.name);
        }
    }

    setConfig(inConfig: Setting) {
        this.config = inConfig;
        console.log(this.config)

        this._updateTheme();

    }

    async read() {
        this.config = await invoke("load_settings") as Setting;
        if (!this.config.theme) {
            this.config.theme = {
                mode: "light",
                name: "modern"
            } as Theme
        }
        this._updateTheme();

        console.log("Read", this.config)
    }

    async write() {
        console.log("Write", this.config)
        await invoke("save_settings", { settings: this.config })
    }
}



export const config = new Config()

