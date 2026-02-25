# 🛡️ Pi Swarm: Sovereign AI Security Orchestrator (v2.0)

Pi Swarm is a high-performance, autonomous security framework. This repository is the **Central Intelligence**, housing the orchestrator and specialized agents.

---

## 🚀 Getting Started (Installation)

### 1. Prerequisites
- Python 3.10 or higher.
- [OpenClaw](https://openclaw.ai) installed and running.

### 2. Setup
```bash
# Clone and enter the core
git clone https://github.com/Pi-Swarm/pibot-swarm-v2.git
cd pibot-swarm-v2

# No external dependencies required (Standard Library focus)
# But ensure you have the memory graph initialized:
python3 memory_graph.py
```

---

## 🛠️ Usage (How to run the Swarm)

### Run the Interactive Orchestrator:
```bash
python3 main.py --interactive
```

### Execute a Specialized Task:
- **Network Scan:** `python3 main.py --task scan --target 192.168.1.0/24`
- **OSINT Audit:** `python3 main.py --task osint --target example.com`

---

## 📂 System File Map
- `orchestrator.py`: The central brain managing agent handoffs.
- `agents.py`: Logic for Recon, Analyst, and Reporter agents.
- `monitor_agent.py`: Observability and behavioral guardrails.
- `mcp_bridge.py`: Standardized tool integration via MCP.
- `pi_skills.json`: The persistent database of learned security skills.

---
*Securing the Frontier of Sovereign AI.*
