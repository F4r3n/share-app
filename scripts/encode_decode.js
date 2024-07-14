import * as fs from "fs";
import readline from "node:readline"
const argv = process.argv

function getArg(inWhatArg) {
    const argIndex = argv.indexOf(inWhatArg);
    if (argIndex > 0) {
        return argv[argIndex + 1];
    }
    return undefined;
}

function hasArg(inWhatArg) {
    return argv.indexOf(inWhatArg);
}

const rl = readline.createInterface({
    input: process.stdin,
});

let pipedData = undefined
rl.on("line", (line) => {
    pipedData += line;
});


let result = ""
if (hasArg("--encode") > 0) {
    let data = pipedData;
    if(!pipedData && getArg("--encode"))
        data = fs.readFileSync(getArg("--encode"));
    result = data.toString("base64")
}
else if (hasArg("--decode") > 0) {
    let data = pipedData;
    if(!pipedData && getArg("--decode"))
        data = fs.readFileSync(getArg("--decode")).toString()
    result = Buffer.from(data, "base64");
}

const output = getArg("--out")
if (output) {
    fs.writeFileSync(output, result)
}

process.exit();