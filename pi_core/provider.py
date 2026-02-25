"""
🥧 Pi-Claw Provider Layer
Mirrors OpenClaw's approach to LLM connectivity.
"""

import json
import urllib.request

class PiClawProvider:
    def __init__(self, model="qwen2.5:1.5b"):
        self.model = model
        self.base_url = "http://localhost:11434/api/chat"

    def chat(self, messages):
        """
        يرسل المحادثة بالكامل (System, User, Assistant) كما يفعل OpenClaw.
        """
        payload = {
            "model": self.model,
            "messages": messages,
            "stream": False
        }
        try:
            req = urllib.request.Request(self.base_url, data=json.dumps(payload).encode(), method="POST")
            with urllib.request.urlopen(req) as res:
                response_data = json.loads(res.read().decode())
                return response_data['message']['content']
        except Exception as e:
            return f"Provider Error: {e}"

if __name__ == "__main__":
    p = PiClawProvider()
    print(p.chat([{"role": "user", "content": "Hello, identify yourself."}]))
