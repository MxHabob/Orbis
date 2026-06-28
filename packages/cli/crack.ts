const bcrypt = require("bcryptjs");

const hash = "$2b$10$uuY6g11NBTp9vI8zVen/FuEZZbecFEpbDWI1IO9bcH5qHHN1I9CyW";
const candidates = [
  "admin123",
  "admin123\r",
  "admin123\n",
  "admin123\r\n",
  "admin123 ",
  " admin123",
  "admin",
  "admin\r",
  "admin\n",
  "admin\r\n",
  "admin ",
  "Mohamed",
  "Mohamed admin",
  "admin@orbis.edu",
  "YER",
  "ar",
  "777777777",
  "Mohamed admin\r",
  "Mohamed admin\n",
  "Mohamed admin\r\n",
  "Mohamed\r",
  "Mohamed\n",
  "Mohamed\r\n",
];

for (const c of candidates) {
  if (bcrypt.compareSync(c, hash)) {
    console.log(
      `>>> MATCH FOUND: "${c.replace(/\r/g, "\\r").replace(/\n/g, "\\n")}"`,
    );
    process.exit(0);
  }
}

console.log("No match found in basic list.");
