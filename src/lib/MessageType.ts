
type Response = {
    kind : number,
    content : string[];
}


export type MessageFromIRC = {
    nick_name: string;
    content: string;
    command : string;
    channel : string;
    response?: Response
}
