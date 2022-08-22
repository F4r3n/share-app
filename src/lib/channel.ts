export type Message = {
    nick_name: string;
    content: string;
    date: Date;
    highlight : boolean;
}


export class Channel {
    messages : Message[];
    constructor(inMessage? : Message) {
        this.messages = [] as Message[]
        if(inMessage)
            this.pushMessage(inMessage)
    }


    pushMessage(inMessage : Message) {
        this.messages.push(inMessage);
    }
}

