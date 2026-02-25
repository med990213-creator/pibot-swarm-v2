"""
Pi Swarm Agents Core v3.0 - OFFICIAL LLM-DRIVEN RELEASE
Integrated with Qwen2.5:1.5B via Ollama. 
Role: Autonomous Reasoning, Logic Auditing, and Self-Correction.
"""

import os
import json
import urllib.request
from typing import List, Dict

class LLMBrain:
    """The central reasoning engine for the swarm."""
    def __init__(self, model="qwen2.5:1.5b"):
        self.model = model
        self.url = "http://localhost:11434/api/generate"

    def reason(self, prompt: str):
        payload = {"model": self.model, "prompt": prompt, "stream": False}
        try:
            req = urllib.request.Request(self.url, data=json.dumps(payload).encode(), method="POST")
            with urllib.request.urlopen(req) as res:
                return json.loads(res.read().decode())['response']
        except Exception as e:
            return f"Error: {e}"

class BaseAgent:
    def __init__(self, name: str, role: str):
        self.name = name
        self.role = role
        self.brain = LLMBrain()

    def log(self, message: str):
        print(f"🛡️ [{self.name}] {message}")

class AnalysisAgent(BaseAgent):
    def analyze_and_patch(self, file_path: str):
        self.log(f"Initiating AI-Logic Audit on: {file_path}")
        try:
            with open(file_path, 'r') as f:
                code = f.read()
            
            prompt = f"System: Expert Solana Auditor.\nTask: Fix vulnerabilities (e.g. UncheckedAccount) in this code. Return ONLY the fixed code.\n\nCODE:\n{code}"
            
            self.log("Brain is reasoning about the fix...")
            fixed_code = self.brain.reason(prompt)
            return fixed_code
        except Exception as e:
            return f"Error: {e}"

class ReconnaissanceAgent(BaseAgent):
    def run_scan(self, repo_url: str):
        self.log(f"Cloning Target: {repo_url}")
        target_dir = "/tmp/pi_target"
        import subprocess
        subprocess.run(["rm", "-rf", target_dir])
        subprocess.run(["git", "clone", "--depth", "1", repo_url, target_dir], capture_output=True)
        return {"status": "success", "dir": target_dir}

class ReporterAgent(BaseAgent):
    def generate_report(self, results: str):
        return f"# 🛡️ PI SWARM SOVEREIGN REPORT v3.0\n\n### AI-Generated Fix:\n\`\`\`rust\n{results}\n\`\`\`"
