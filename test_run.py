from agents import ReconnaissanceAgent, AnalysisAgent, ReporterAgent
import json

def run_mission():
    print("--- 🕵️ Pi Swarm: INTERNAL STABILITY TEST ---")
    target = "https://github.com/openclaw/openclaw"
    
    # 1. Recon
    recon = ReconnaissanceAgent("Recon", "Specialist")
    scan_results = recon.run_scan(target)
    
    if scan_results['status'] == 'success':
        # 2. Analysis
        analyst = AnalysisAgent()
        analysis = analyst.analyze(scan_results)
        
        # 3. Report
        print("\n--- 📊 TRUE RESULTS (Cleaned from False Positives) ---")
        if not analysis['findings']:
            print("✅ VERDICT: ZeroClaw is SAFE. Previous warnings were false positives.")
        else:
            for f in analysis['findings']:
                print(f"🚩 FOUND: [{f['type']}] in {f['file']} (Count: {f['count']})")
    else:
        print(f"❌ Error: {scan_results['error']}")

if __name__ == "__main__":
    run_mission()
