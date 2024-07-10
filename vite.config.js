import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import process from "process";
import path from "path";

const isDev = process.env.NODE_ENV === "development";
const isProd = !isDev;

export default defineConfig({
    plugins: [react({ minify: isProd })],
    build: {
        minify: isProd ? "esbuild" : false,
        sourcemap: true,
        emptyOutDir: true,
        outDir: path.join(process.env.OUT_DIR, "static"),
    },
});
