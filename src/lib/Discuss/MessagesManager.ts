import { Channel } from './channel';
import type { Message } from "./channel";

class MessagesManager {
    private _listMessages: Map<string, Channel>;
    constructor() {
        this._listMessages = new Map<string, Channel>([]);
    }

    putMessageInList(inNewMessage: Message, inChannelName: string) {
        if (!this._listMessages.has(inChannelName)) {
            this._listMessages.set(inChannelName, new Channel());
        }
        let channel: Channel = this._listMessages.get(inChannelName) ?? new Channel();
        let lastMessage = channel.getLast();

        if (
            lastMessage?.nick_name === inNewMessage.nick_name &&
            (new Date().getTime() - lastMessage.date.getTime()) / 1000 < 3
        ) {
            lastMessage.content += "\n" + inNewMessage.content;
            channel.updateLast(lastMessage)
        } else {
            channel.pushMessage(inNewMessage);
        }
    }

    getChannel(inChannelName: string): Channel {
        if (!this._listMessages.has(inChannelName)) {
            this._listMessages.set(inChannelName, new Channel());
        }
        return this._listMessages.has(inChannelName)
            ? this._listMessages!.get(inChannelName)!
            : new Channel();
    }

}

export const messagesManager = new MessagesManager();