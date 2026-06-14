import * as fs from 'fs';
import * as path from 'path';

const openapi = JSON.parse(fs.readFileSync(path.resolve(__dirname, '../openapi.json'), 'utf8'));
console.log("Schema keys in openapi.json:", Object.keys(openapi.components.schemas));
