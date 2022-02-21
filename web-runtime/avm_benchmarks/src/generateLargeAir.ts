import {writeFileSync} from "fs";

export function generateLargeAir(depth: number) : string {
    if (depth <= 1) {
        return "(null)\n";
    }

    let inner = generateLargeAir(depth - 1);
    return `(seq\n${inner}${inner})\n`;
}
for (let i = 1; i <= 20; i++) {
    writeFileSync("seq_null_" + i + ".air", generateLargeAir(i));
}
