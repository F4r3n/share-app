<script lang="ts">
    import { onMount } from "svelte";
    import { createEventDispatcher } from "svelte";
    import MessageRenderer from "./MessageRenderer/MessageRenderer.svelte";
    import { afterUpdate } from "svelte";

    type Token = {
        type: string;
        value: any;
    };
    const dispatch = createEventDispatcher();

    let isASCII = false;
    export let content = "";
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

    afterUpdate(() => {});

    function parseColors(inContent: string): Token[] {
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
                    tokens.push({ type: "RAW", value: { content: buffer } });
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
            tokens.push({ type: "RAW", value: { content: buffer } });
            buffer = "";
        }

        return tokens;
    }

    function createTokens(inContent: string): Token[] {
        let tokens: Token[] = [] as Token[];
        tokens = parseColors(inContent);
        console.log(tokens)
        let match = null;
        let start = 0;
        let end = inContent.length;
        
        for (let index = 0; index < tokens.length; index++) {
            let token = tokens[index];
            if (token.type === "RAW") {
                const content = token.value.content;
                let hasFoundLink = false;
                while ((match = regex.exec(content)) != null) {
                    hasFoundLink = true;
                    start = match.index;
                    const url = content.substring(
                        match.index,
                        match.index + match[0].length,
                    );
                    start = match.index + match[0].length;

                    tokens.push({
                        type: "ATag",
                        value: { content: url, href: url },
                    });
                }

                if(hasFoundLink)
                    token.value=""
            }
        }
        console.log(tokens)

        return tokens;
    }
</script>

<div class="message" class:message-ascii={isASCII}>
    <MessageRenderer
        on:message_formatted={() => {
            dispatch("message_formatted");
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
