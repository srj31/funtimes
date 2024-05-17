import { Buffer } from "node:buffer";

const input = "0x1234567890abcdef1234";
const buf = Buffer.from(input.slice(2), "hex");

console.log(buf);
console.log("String version");
console.log(buf.toString());
console.log(buf.toString("hex"));
// Prints: 68656c6c6f20776f726c64
// Prints: aGVsbG8gd29ybGQ=

// Prints: <Buffer 66 00 68 00 71 00 77 00 68 00 67 00 61 00 64 00 73 00>
//

// Define chunkBytesInput function
const chunkBytesInput = (input) => {
  const result = [];
  for (let i = 0; i < input.length; i += 8) {
    result.push(input.slice(i, i + 8));
  }
  return result;
};

function hexToDecimal(hex) {
  const hexDigits = "0123456789ABCDEF";
  let decimal = BigInt(0); // Use BigInt to handle large integers
  // Iterate over each digit of the hexadecimal number
  for (let i = 0; i < hex.length; i++) {
    const digit = BigInt(hexDigits.indexOf(hex[i].toUpperCase()));
    decimal = decimal * BigInt(16) + digit;
  }
  return decimal;
}

class Data {
  constructor(rawBytes) {
    this.rawBytes = rawBytes;
  }

  toInts() {
    const chunked = chunkBytesInput(this.rawBytes);
    console.log("chunked", chunked);
    const intsArray = chunked.map(function (chunk) {
      console.log("chunked", chunk.toString("hex"));
      return hexToDecimal(chunk.toString("hex"));
    });
    return {
      values: intsArray,
      length: this.rawBytes.length,
    };
  }

  toHex() {
    return "0x" + this.rawBytes.toString("hex");
  }

  static fromHex(input) {
    return new Data(Buffer.from(input.slice(2), "hex"));
  }
}

const data = Data.fromHex(input);
console.log(data.rawBytes);
console.log("toInts()");
console.log(data.toInts());
console.log("toHex()");
console.log(data.toHex());
