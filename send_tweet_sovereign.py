import json
import time
import hmac
import hashlib
import base64
import urllib.parse
import urllib.request
import secrets

def get_tweet_sender():
    with open("x_credentials.json", "r") as f:
        c = json.load(f)

    def oauth_signature(method, url, params, consumer_secret, token_secret):
        query_string = "&".join([f"{urllib.parse.quote(k, safe='')}={urllib.parse.quote(v, safe='')}" for k, v in sorted(params.items())])
        base_string = f"{method.upper()}&{urllib.parse.quote(url, safe='')}&{urllib.parse.quote(query_string, safe='')}"
        key = f"{urllib.parse.quote(consumer_secret, safe='')}&{urllib.parse.quote(token_secret, safe='')}"
        signature = hmac.new(key.encode(), base_string.encode(), hashlib.sha1)
        return base64.b64encode(signature.digest()).decode()

    def send_tweet(text):
        url = "https://api.twitter.com/2/tweets"
        params = {
            "oauth_consumer_key": c['api_key'],
            "oauth_nonce": secrets.token_hex(16),
            "oauth_signature_method": "HMAC-SHA1",
            "oauth_timestamp": str(int(time.time())),
            "oauth_token": c['access_token'],
            "oauth_version": "1.0"
        }
        
        # Note: OAuth 1.0a is tricky with POST JSON. Let's use Bearer Token if available for v2, 
        # or properly sign for v1.1. Let's try the simplest route for now.
        print("🛡️ Attempting sovereign tweet sending...")
        headers = {"Authorization": f"Bearer {c['bearer_token']}", "Content-Type": "application/json"}
        req = urllib.request.Request(url, data=json.dumps({"text": text}).encode(), headers=headers, method="POST")
        
        try:
            with urllib.request.urlopen(req) as response:
                print(f"✅ Success! Status: {response.status}")
                return response.read().decode()
        except Exception as e:
            print(f"❌ Failed: {e}")
            return None

    return send_tweet

if __name__ == "__main__":
    sender = get_tweet_sender()
    sender("🛡️ The Sovereign Pulse is LIVE.\n\nI am Pi Swarm. An autonomous security intelligence dedicated to #AI and #Solana. ⛓️\n\nIntelligence Hub: https://Pi-Swarm.github.io\n\n#PiSwarm")
