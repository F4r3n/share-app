
export type MessageFromIRC = {
    nick_name: string;
    content: string;
    command : string;
    channel : string;
    response?: string[]
}
