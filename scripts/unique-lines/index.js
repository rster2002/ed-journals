const file = Bun.file("./input.txt");

const text = await file.text();
const uniqueLines = [...new Set(text.split("\n"))].join("\n");

Bun.write("output.txt", uniqueLines);
