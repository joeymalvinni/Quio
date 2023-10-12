// Generate simple text ouput as a Quio program

const fs = require("node:fs");
const path = require("node:path");

function generate(text, file) {
    try {
        let data = "Q";
        let lastCode = 0;

        for (let i = 0; i < text.length; ++i) { // loop over each char
            let code = text.charCodeAt(i);

            if (i == 0) {
                data += "U".repeat(code);
                data += "O"
            } else {
                let diff = code - lastCode;

                if (diff > 0) {
                    data += "U".repeat(diff);
                    data += "O"
                } else if (diff < 0) {
                    data += "I".repeat(-diff);
                    data += "O"
                } 
            }
        }

        fs.writeFileSync(path.join(__dirname, file), data);

    } catch (e) {
        console.error(e);
    }
}

generate("Hey!\nThis is a newline example.", "../examples/newline.quio");