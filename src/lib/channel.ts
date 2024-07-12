import { writable, get } from 'svelte/store';
import type { Writable } from 'svelte/store';

export type Message = {
    nick_name: string;
    content: string;
    date: Date;
    highlight: boolean;
}


export class Channel {
    _messages: Writable<Message[]>;
    constructor(inMessage?: Message) {
        this._messages = writable([] as Message[])
        if (inMessage)
            this.pushMessage(inMessage)
    }

    pushMessage(inMessage: Message) {
        this._messages.update(v => [...v, inMessage]);
    }

    updateLast(inMessage: Message) {
        this._messages.update(v => {
            v[v.length - 1] = inMessage;
            return v;
        });
    }

    getLast(): Message | undefined {
        let lastMessage: Message | undefined;
        let messages = get(this._messages);
        if (messages.length > 0) {
            lastMessage = messages[messages.length - 1];
        }
        return lastMessage;
    }

    public getListMessages(): Writable<Message[]> {
        return this._messages;
    }

}

