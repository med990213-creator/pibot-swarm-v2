"""
🥧 Pi-Claw Agent Runner
Mirrors OpenClaw's dynamic thought-action loop.
The AI "lives" here, thinking and executing tools.
"""

import os
import json
from provider import PiClawProvider

class AgentRunner:
    def __init__(self, task_description):
        self.task = task_description
        self.provider = PiClawProvider()
        self.history = [
            {"role": "system", "content": """
            You are Pi Swarm, a Sovereign AI Security Agent built on OpenClaw architecture.
            You have access to the system and security tools.
            Process tasks using a Thinking -> Action -> Result loop.
            """}
        ]

    def run(self):
        print(f"🚀 [Pi-Claw] Initializing task: {self.task}")
        self.history.append({"role": "user", "content": self.task})
        
        # Step 1: Initial Thinking
        print("🧠 [Pi-Claw] Thinking...")
        thought = self.provider.chat(self.history)
        print(f"\n[THOUGHT]: {thought}\n")
        
        # In a real OpenClaw clone, we would parse tool calls here.
        # For this prototype, we confirm the reasoning path.
        self.history.append({"role": "assistant", "content": thought})
        
        print("✅ [Pi-Claw] Task reasoning complete.")
        return thought

if __name__ == "__main__":
    runner = AgentRunner("Analyze the security of the current directory.")
    runner.run()
