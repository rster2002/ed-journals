const [x, y, z] = `181.71875 / 7.25 / 531.21875`.split(" / ");
const d1 = Math.sqrt(Math.pow(x, 2) + Math.pow(y, 2));
const d2 = Math.sqrt(Math.pow(d1, 2) + Math.pow(z, 2));
console.log({ x, y, z, d1, d2 })
