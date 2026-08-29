import { cp, mkdir, rm } from 'node:fs/promises';
import { resolve } from 'node:path';

const root = resolve(import.meta.dirname, '..');
const src = resolve(root, 'src');
const dist = resolve(root, 'dist');

await rm(dist, { recursive: true, force: true });
await mkdir(dist, { recursive: true });
await cp(src, dist, { recursive: true });
console.log('Frontend copied to dist/.');
