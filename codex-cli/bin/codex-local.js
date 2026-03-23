#!/usr/bin/env node

process.env.CODEX_LOCAL_FORK = "1";
await import("./codex.js");
