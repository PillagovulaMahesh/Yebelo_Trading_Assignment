#!/bin/bash

cat << 'EOF' > README.md
# 📊 Yebelo Assignment – Fullstack Crypto Trading Analytics System

This repository contains my submission for the **Yebelo Technologies Fullstack Developer Assignment**.  
The project demonstrates my ability to rapidly learn and implement **new technologies** using **AI-assisted development**.

---

## 🚀 Tech Stack

- **Containerization:** Docker + Docker Compose  
- **Streaming Platform:** Redpanda (Kafka-compatible)  
- **Backend:** Rust (tokio + rdkafka)  
- **Data Ingestion:** Python (CSV → Redpanda)  
- **Frontend:** Next.js (TypeScript + Recharts)  
- **Styling:** Tailwind CSS  


---

## ⚡ How It Works

### 1️⃣ Phase 1 – Infrastructure Setup  
- Redpanda broker + Console via Docker Compose  
- Topics created: `trade-data` and `rsi-data`

### 2️⃣ Phase 2 – Data Ingestion  
- Python script (`ingest.py`) reads `trades_data.csv`  
- Publishes each trade as JSON → `trade-data` topic  

### 3️⃣ Phase 3 – Backend (Rust RSI Service)  
- Rust service consumes `trade-data`  
- Calculates **14-period RSI** per token  
- Publishes RSI values → `rsi-data` topic  

### 4️⃣ Phase 4 – Frontend (Next.js Dashboard)  
- Token selector for 5 tokens  
- Price chart (line graph)  
- RSI chart with **30/70 thresholds**  
- Current price + RSI shown numerically  

---

## ▶️ Running Locally

### Prerequisites
- Docker & Docker Compose
- Rust (if running backend locally)
- Node.js + npm
- Python 3.x

### Steps

# 1. Clone repo
git clone https://github.com/your-username/yebelo-assignment.git
cd yebelo-assignment

# 2. Start Redpanda + backend
docker-compose up -d

# 3. Ingest sample CSV
cd data_ingestion
npm install   # if needed for dependencies
python ingest.py

# 4. Run frontend
cd ../frontend
npm install
npm run dev
