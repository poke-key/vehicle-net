#!/bin/bash

cd web

if command -v bun &> /dev/null; then
    bun install
    bun dev
else
    npm install
    npm run dev
fi
