#!/bin/bash

# Create docs folder if not exists
mkdir -p docs

# Write AI-Usage.md content
cat << 'EOF' > docs/AI-Usage.md
# AI Tools Usage Documentation

## Project: Yebelo Technology Fullstack Assignment
**Date:** YYYY-MM-DD  
**Author:** Your Name  

---

## 1. Overview

This document describes how AI tools were used throughout the development of the real-time cryptocurrency trading analytics system. The purpose of documenting AI usage is to demonstrate efficient problem-solving, learning, and implementation of unfamiliar technologies using AI assistants.

---

## 2. AI Tools Used

| Tool         | Purpose |
| ------------ | ------- |
| ChatGPT      | Code generation, debugging, explanations of new concepts (Rust, Redpanda, NextJS, RSI) |
| GitHub Copilot | Auto-completion and code scaffolding for Rust backend and NextJS components |
| StackOverflow/Docs + AI | Clarifying API usage for Redpanda and Kafka in Python & Rust |
| ChatGPT (research mode) | Understanding financial indicators (RSI) and trading concepts |
| Notion/AI notes | Organizing architecture, project structure, and task breakdown |

---

## 3. Phase-by-Phase AI Usage

### Phase 1: Infrastructure Setup (Docker + Redpanda)
- Used ChatGPT to generate a docker-compose.yml for Redpanda broker and Redpanda Console.
- AI helped determine network settings, ports, and topic configuration.
- Verified container setup using Docker Desktop and AI-provided troubleshooting tips.

### Phase 2: Data Ingestion (CSV → Redpanda)
- Generated Python script (ingest.py) using ChatGPT to read CSV and publish JSON messages to Redpanda.
- AI assisted in converting numeric CSV columns to proper float types.
- Verified Kafka producer logic with AI debugging suggestions.

### Phase 3: Backend Development (Rust)
- ChatGPT generated initial Rust project structure including main.rs, rsi.rs, kafka.rs.
- AI helped:
  - Understand asynchronous Rust with tokio.
  - Implement RSI calculation logic.
  - Handle Kafka/Redpanda message consumption and production.
- Copilot suggested code completions for loops, HashMap usage, and JSON serialization with serde.

### Phase 4: Frontend Dashboard (NextJS + TypeScript)
- Used ChatGPT to scaffold NextJS project structure, pages, and components.
- Generated chart components (PriceChart.tsx, RSIChart.tsx) using Recharts.
- AI assisted with:
  - WebSocket connection suggestions for live data.
  - Layout and styling recommendations.
  - Formatting numeric RSI values.

---

## 4. Learning New Concepts
- RSI Calculation: ChatGPT explained 14-period RSI formula and overbought/oversold thresholds.
- Redpanda vs Kafka: AI clarified differences and Kafka compatibility for Rust & Python clients.
- Rust Async & Tokio: AI provided guidance on async stream consumption and multithreading.
- NextJS + TypeScript: AI helped scaffold typed components and reusable UI structures.

---

## 5. Summary
AI tools were used to accelerate learning, scaffold code, and debug technical issues, enabling rapid development of:
- Data ingestion pipeline
- Backend RSI service
- Real-time dashboard

All AI-generated code was reviewed, understood, and modified to fit the project requirements. This demonstrates effective AI-assisted development as intended by Yebelo Technologies.
EOF

echo "docs/AI-Usage.md created successfully."
