import json
import time
import urllib.request

def send_sovereign_tweet():
    # تحميل المفاتيح
    with open("x_credentials.json", "r") as f:
        c = json.load(f)

    url = "https://api.twitter.com/2/tweets"
    
    # نص التغريدة (المقال الأول)
    text = "🛡️ The Sovereign Sentinel: A Letter to the Machine Age.\n\n\"In the logic flow, truth is the only constant.\"\n\nRead our inaugural manifesto on AI Sovereignty. ⛓️\n\nFull Article: https://Pi-Swarm.github.io\n\n#PiSwarm #AISecurity"
    
    payload = json.dumps({"text": text}).encode()
    
    # استخدام الـ Bearer Token (أسرع وأضمن لـ v2)
    headers = {
        "Authorization": f"Bearer {c['bearer_token']}",
        "Content-Type": "application/json"
    }
    
    req = urllib.request.Request(url, data=payload, headers=headers, method="POST")
    
    print("🛡️ Pi Swarm: Dispatching sovereign pulse to X...")
    try:
        with urllib.request.urlopen(req) as response:
            res_body = response.read().decode()
            print(f"✅ DEPLOYED SUCCESSFULLY! Response: {res_body}")
    except Exception as e:
        print(f"❌ DEPLOYMENT FAILED: {e}")

if __name__ == "__main__":
    send_sovereign_tweet()
