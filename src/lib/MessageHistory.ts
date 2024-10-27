export type Navigation = "up" | "down";
function clamp(num: number, min: number, max: number) {
    return num <= min ? min : num >= max ? max : num;
}

export class MessageHistory {
    private messageHistory: string[] = [];
    private indexHistory = 0;
    private hasReachedStartHistory = true;
    append(inMessage: string) {
        this.messageHistory.push(inMessage);
        this.indexHistory = this.messageHistory.length;
    }

    navigate(nav: Navigation): boolean {
        if (this.messageHistory.length == 0) return false;
        switch (nav) {
            case "up": {
                this.indexHistory -= 1;
                break;
            }
            case "down": {
                this.indexHistory += 1;
                break;
            }
        }
        let canContinue = true;
        if (this.indexHistory < 0 
        || this.indexHistory >= this.messageHistory.length)
        {
            canContinue = false;
        }
        if(this.indexHistory >= this.messageHistory.length)
        {
            this.hasReachedStartHistory = true;
        }
        else{
            this.hasReachedStartHistory = false;
        }
        this.indexHistory = clamp(
            this.indexHistory,
            0,
            this.messageHistory.length,
        );
        return canContinue;
    }

    getMessage(): string {
        return this.messageHistory[this.indexHistory];
    }

    isStart(): boolean {
        return this.hasReachedStartHistory;
    }
}