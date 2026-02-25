"""
🥧 Pi Sovereign Core - Built on OpenClaw Philosophy
Usage: python3 pi.py [task|status|audit|fix]
"""

import sys
import json
import os
from agents import AnalysisAgent, ReconnaissanceAgent, ReporterAgent

class PiClaw:
    def __init__(self):
        self.workspace = os.getcwd()
        self.version = "3.1-Sovereign"

    def status(self):
        print(f"🛡️ Pi-Claw System Status: [ONLINE]")
        print(f"📊 Version: {self.version}")
        print(f"📂 Workspace: {self.workspace}")

    def audit(self, target):
        print(f"🕵️ Starting Sovereign Audit on: {target}")
        recon = ReconnaissanceAgent("Recon", "Specialist")
        scan = recon.run_scan(target)
        if scan['status'] == 'success':
            analyst = AnalysisAgent()
            # هنا نستخدم عقل Qwen لإصلاح الكود مباشرة
            patch = analyst.analyze_and_patch(os.path.join(scan['dir'], 'programs/kadeshx/src/lib.rs')) # مثال
            print(f"✅ Audit Complete. Patch generated in memory.")
            return patch
        return None

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 pi.py [status|audit <target>]")
        return

    cmd = sys.argv[1]
    pi = PiClaw()

    if cmd == "status":
        pi.status()
    elif cmd == "audit":
        target = sys.argv[2] if len(sys.argv) > 2 else ""
        pi.audit(target)
    else:
        print(f"Unknown command: {cmd}")

if __name__ == "__main__":
    main()
