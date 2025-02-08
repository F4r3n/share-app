<script lang="ts">
    import { onMount } from "svelte";
    import MessageRenderer from "./MessageRenderer.svelte";

    type Token = {
        type: string;
        value: any;
    };

    let isASCII = $state(false);
    let {content, onMessageFormatted}:
    {
        content : string;
        onMessageFormatted : ()=>void;
    } = $props();
    let regex: RegExp = /(http:\/\/|https:\/\/){1}(www.)?.[^\s]+/g;

    function isASCIIArt(inLine: string) {
        let isNormal = false;
        for (let c of inLine) {
            const charCode = c.charCodeAt(0);
            isNormal ||= !(
                (charCode >= 32 && charCode <= 47) ||
                (charCode >= 58 && charCode < 64) ||
                (charCode >= 91 && charCode < 96) ||
                (charCode >= 123 && charCode < 126)
            );
        }
        return !isNormal;
    }

    onMount(async () => {
        isASCII = isASCIIArt(content);
    });


    function parseElement(inContent : string) : Token[]
    {
        let match = null;
        let start = 0;
        let tokens: Token[] = [] as Token[];
        let end = inContent.length;

        const content = inContent;
        let hasFoundLink = false;
        let previousStart = 0;
        
        while ((match = regex.exec(content)) != null) {
            hasFoundLink = true;
            start = match.index;
            
            if(previousStart < start)
            {
                tokens.push({ type: "RAW", value: { content: content.substring(
                previousStart,
                start,
            ) } });
            }

            previousStart = start;
            const url = content.substring(
                match.index,
                match.index + match[0].length,
            );
            start = match.index + match[0].length
            tokens.push({
                type: "ATag",
                value: { content: url, href: url },
            });
        }

        if(start < end)
            {
                tokens.push({ type: "RAW", value: { content: content.substring(
                start,
                end,
            ) } });
            }
        return tokens;
    }

    function parse(inContent: string): Token[] {
        let tokens: Token[] = [] as Token[];

        let buffer = "";
        let currentToken = null;
        for (let i = 0; i < inContent.length; ++i) {
            const char = inContent.charCodeAt(i);
            if (char === 3) {
                if (currentToken != null) {
                    currentToken.value.content = buffer;
                    tokens.push(currentToken);
                    buffer = "";
                }
                if (buffer !== "") {
                    const tokensParsed = parseElement(buffer);
                    tokensParsed.forEach(e => tokens.push(e));
                    buffer = "";
                }

                currentToken = null;

                let j = i;
                let value = [0, 0];
                let background_foreground = 0;
                while (j < inContent.length) {
                    j++;
                    const c = inContent.charCodeAt(j);
                    if (c >= 48 && c <= 57) {
                        value[background_foreground] += c - 48;
                    } else if (c === 44) {
                        background_foreground++;
                    } else {
                        break;
                    }
                }
                i = j - 1;
                if (value[0] !== 0 || value[1] !== 0) {
                    currentToken = {
                        type: "ColorTag",
                        value: {
                            color: value[1],
                            background: value[0],
                            content: "",
                        },
                    };
                }
            } else if (char == 15) {
                continue;
            } else {
                buffer += inContent.charAt(i);
            }
        }
        if (currentToken != null) {
            currentToken.value.content = buffer;
            tokens.push(currentToken);
            buffer = "";
        }

        if (buffer !== "") {
            const tokensParsed = parseElement(buffer);
            tokensParsed.forEach(e => tokens.push(e));
            buffer = "";
        }

        return tokens;
    }

    function createTokens(inContent: string): Token[] {
        let tokens: Token[] = [] as Token[];
        tokens = parse(inContent);
        return tokens;
    }
</script>

<div class="message" class:message-ascii={isASCII}>
    <MessageRenderer
        onMessageFormatted={() => {
           onMessageFormatted();
        }}
        tokens={createTokens(content)}
    ></MessageRenderer>
</div>

<style>
    .message {
        padding-left: 20px;
        word-wrap: break-word;
        white-space: break-spaces;
    }

    .message-ascii {
        white-space: pre;
        font-family: monospace;
    }
</style>
