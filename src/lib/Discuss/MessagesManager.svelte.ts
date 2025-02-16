

export type Message = {
    nick_name: string;
    content: string;
    date: Date;
    highlight: boolean;
};

export class ChattManager {
    public isConnected = $state(true);
    public isUnread: boolean = $state(false);
    public listMessages: Message[] = $state([]);

    pushMessage(inNewMessage: Message, isCurrentChannel : boolean) {
        this.isUnread = !isCurrentChannel;
        let lastMessage: Message | null = null;
        if (this.listMessages.length > 0) {
            lastMessage = this.listMessages[this.listMessages.length - 1];

            if (
                lastMessage?.nick_name === inNewMessage.nick_name &&
                (new Date().getTime() - lastMessage.date.getTime()) / 1000 <
                    3
            ) {
                lastMessage.content += "\n" + inNewMessage.content;
            } else {
                this.listMessages.push(inNewMessage);
            }
        } else {
            this.listMessages.push(inNewMessage);
        }
    }
}
