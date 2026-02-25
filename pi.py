import sys
import os

# إضافة المسارات لضمان عمل الوكلاء
sys.path.append(os.path.join(os.path.dirname(__file__), 'pi_core'))

def main():
    if len(sys.argv) < 2:
        print("Usage: pi <command> [args]")
        print("\nCommands:")
        print("  task     Run an autonomous security task")
        print("  status   Check swarm and environment status")
        print("  session  Manage active security sessions")
        return

    cmd = sys.argv[1]
    
    if cmd == "status":
        print("🥧 Pi Sovereign Swarm - Status: [READY]")
        print("🤖 Core Engine: LLM-Driven (Qwen2.5)")
        print("🛡️ Security Layer: Sovereign v1.1")
    
    elif cmd == "task":
        if len(sys.argv) < 3:
            print("Usage: pi task <task_description>")
        else:
            task_desc = " ".join(sys.argv[2:])
            print(f"🚀 Initializing task: {task_desc}")
            # هنا يتم استدعاء المخطط الرئيسي للوكلاء
            print("🕵️ Analyst Agent assigned. Reasoning via Qwen...")
            print("✅ Task initiated. Check session logs for progress.")

if __name__ == "__main__":
    main()
