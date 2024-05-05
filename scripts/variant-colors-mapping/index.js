const file = Bun.file("./input.txt");

const text = await file.text();

const colorSet = new Set();
const sourceSet = new Set();

const starClass = [
    "O",
    "B",
    "A",
    "F",
    "G",
    "K",
    "M",
    "L",
    "T",
    "Y",
    "TTS",
    "Ae",
    "Be",
    "AeBe",
    "W",
    "WN",
    "WNC",
    "WC",
    "WO",
    "CS",
    "C",
    "CN",
    "CJ",
    "CH",
    "CHd",
    "D",
    "DA",
    "DAB",
    "DAO",
    "DAZ",
    "DAV",
    "DB",
    "DBZ",
    "DBV",
    "DO",
    "DOV",
    "DQ",
    "DC",
    "DCV",
    "DX",
    "N",
    "H",
    "X",
    "SupermassiveBlackHole",
    "ABlueWhiteSuperGiant",
    "FWhiteSuperGiant",
    "MRedSuperGiant",
    "KOrangeGiant",
    "RoguePlanet",
    "Nebula",
    "StellarRemnantNebula",
    "MS",
    "S",
];

const targetGenus = false;
const lines = text.split("\n")
    .map(line => line.split("\t"))
    .filter(line => (line[1] ?? "").startsWith("$Codex"))
    .map(([variant, codex]) => {
        const [speciesString, color] = variant.split(" - ");
        const [genus, species] = speciesString.split(" ");

        const sourceString = codex.split("_")[4];

        sourceSet.add(sourceString);

        const source = starClass.includes(sourceString)
            ? `VariantSource::StarClass(StarClass::${sourceString})`
            : `VariantSource::Material(Material::${sourceString})`;

        return [genus, species, color, source];
    })
    .filter(([genus, species, color, source]) => {
        if (colorSet.has(color)) {
            return false;
        }

        colorSet.add(color);

        return true;
    })
    .map(([genus, species, color, source]) => {
        return targetGenus
            ? `(Genus::${genus}, _, ${source}) => VariantColor::${color},`
            : `(_, Species::${genus}${species}, ${source}) => VariantColor::${color},`;
    })
    .join("\n");

console.log(sourceSet);
Bun.write("./output.txt", lines);
