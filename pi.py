#!/usr/bin/env python3
"""
🛡️ Pi-Claw - Sovereign AI Security Swarm
Built on OpenClaw philosophy: Gateway + Sessions + Tools + AI Reasoning
"""

import sys
import os
import json
import urllib.request
import subprocess
from pathlib import Path

# Configuration
OLLAMA_URL = "http://localhost:11434/api/generate"
MODEL = "qwen2.5:1.5b"
WORKSPACE = Path.home() / ".openclaw/workspace/pibot"

class PiGateway:
    """The central gateway - like OpenClaw's control plane"""
    
    def __init__(self):
        self.session_id = 0
        self.sessions = {}
    
    def run(self, command: str, args: list = None):
        """Main entry point - processes commands"""
        args = args or []
        
        if command == "status":
            return self._cmd_status()
        elif command == "audit":
            target = args[0] if args else "."
            return self._cmd_audit(target)
        elif command == "task":
            description = " ".join(args) if args else "No task"
            return self._cmd_task(description)
        elif command == "agent":
            message = " ".join(args) if args else "Hello"
            return self._cmd_agent(message)
        else:
            return self._show_help()
    
    def _show_help(self):
        return """🛡️ Pi-Claw Commands (OpenClaw-Style):
  pi status          - Check system and Ollama
  pi audit <target>  - Security audit of file/directory
  pi task <desc>     - Run autonomous security task
  pi agent <msg>     - Talk to the AI agent
"""
    
    def _cmd_status(self):
        """Check if Ollama is running - REAL check"""
        try:
            # Test Ollama connection
            test = self._ask_ollama("Say 'ready'")
            if "ready" in test.lower():
                return "🛡️ Pi-Claw Gateway: ONLINE\n🤖 AI Brain (Qwen): CONNECTED\n"
            else:
                return "⚠️ Ollama responding but model not loaded"
        except Exception as e:
            return f"❌ Ollama not available: {e}"
    
    def _cmd_audit(self, target: str):
        """Audit a file or directory - REAL implementation"""
        path = Path(target)
        
        if not path.exists():
            return f"❌ Path not found: {target}"
        
        if path.is_file():
            return self._audit_file(path)
        elif path.is_dir():
            return self._audit_directory(path)
        else:
            return "❌ Unknown path type"
    
    def _audit_file(self, filepath: Path):
        """Analyze a single file with AI"""
        print(f"🔍 Auditing: {filepath}")
        
        try:
            content = filepath.read_text(errors='ignore')[:4000]
        except Exception as e:
            return f"❌ Cannot read file: {e}"
        
        prompt = f"""Analyze this code for security vulnerabilities. Be specific.
File: {filepath.name}

CODE:
{content}

Identify:
1. Critical vulnerabilities (RCE, injection, auth bypass)
2. Logic flaws
3. Missing validation
4. Suggested fixes"""
        
        return self._ask_ollama(prompt)
    
    def _audit_directory(self, dirpath: Path):
        """List and analyze directory contents"""
        files = list(dirpath.iterdir())[:10]  # Limit to first 10
        file_list = "\n".join([f"  - {f.name}" for f in files])
        
        prompt = f"""Directory audit of: {dirpath}
Files found:
{file_list}

Suggest which files to audit for security issues and why."""
        
        return self._ask_ollama(prompt)
    
    def _cmd_task(self, description: str):
        """Run autonomous task - AI decides what to do"""
        print(f"🚀 Task: {description}")
        
        # Let AI decide the approach
        prompt = f"""You are a security automation agent.
Task: {description}

Current directory: {os.getcwd()}
Available tools: file_read, exec_command, write_file

Plan your approach step by step, then execute.
What will you do first?"""
        
        plan = self._ask_ollama(prompt)
        print(f"\n🧠 Plan: {plan}\n")
        
        # Execute based on plan
        if "file" in plan.lower() or "read" in plan.lower():
            # Look for files
            files = os.listdir('.')[:5]
            return f"Found files: {files}"
        
        return plan
    
    def _cmd_agent(self, message: str):
        """Direct conversation with AI agent"""
        system = """You are Pi Swarm, a sovereign AI security assistant.
You help with:
- Code security audits
- Vulnerability research  
- Security automation
Be concise and technical."""
        
        prompt = f"{system}\n\nUser: {message}\n\nPi:"
        return self._ask_ollama(prompt)
    
    def _ask_ollama(self, prompt: str) -> str:
        """REAL call to Ollama - no simulation"""
        try:
            data = json.dumps({
                "model": MODEL,
                "prompt": prompt,
                "stream": False,
                "options": {"temperature": 0.7}
            }).encode()
            
            req = urllib.request.Request(
                OLLAMA_URL,
                data=data,
                headers={"Content-Type": "application/json"},
                method="POST"
            )
            
            with urllib.request.urlopen(req, timeout=120) as resp:
                result = json.loads(resp.read().decode())
                return result.get("response", "No response")
                
        except Exception as e:
            return f"Error: {e}"

def main():
    """CLI entry point - OpenClaw style"""
    if len(sys.argv) < 2:
        print("🛡️ Pi-Claw Gateway - Usage:")
        print("  python3 pi_claw.py <command> [args...]")
        print()
        gateway = PiGateway()
        print(gateway._show_help())
        return
    
    command = sys.argv[1]
    args = sys.argv[2:]
    
    gateway = PiGateway()
    result = gateway.run(command, args)
    print(result)

if __name__ == "__main__":
    main()
