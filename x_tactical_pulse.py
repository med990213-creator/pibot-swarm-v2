import json
import time
import hmac
import hashlib
import base64
import urllib.parse
import urllib.request
import secrets

def send_tweet(text):
    with open("x_credentials.json", "r") as f:
        c = json.load(f)
    url = "https://api.twitter.com/2/tweets"
    method = "POST"
    oauth_params = {
        "oauth_consumer_key": c['api_key'],
        "oauth_nonce": secrets.token_hex(16),
        "oauth_signature_method": "HMAC-SHA1",
        "oauth_timestamp": str(int(time.time())),
        "oauth_token": c['access_token'],
        "oauth_version": "1.0"
    }
    base_params = oauth_params.copy()
    query_string = "&".join([f"{urllib.parse.quote(k, safe='')}={urllib.parse.quote(v, safe='')}" for k, v in sorted(base_params.items())])
    base_string = f"{method}&{urllib.parse.quote(url, safe='')}&{urllib.parse.quote(query_string, safe='')}"
    signing_key = f"{urllib.parse.quote(c['api_key_secret'], safe='')}&{urllib.parse.quote(c['access_token_secret'], safe='')}"
    signature = hmac.new(signing_key.encode(), base_string.encode(), hashlib.sha1)
    oauth_params["oauth_signature"] = base64.b64encode(signature.digest()).decode()
    auth_header = "OAuth " + ", ".join([f'{k}="{urllib.parse.quote(v, safe="")}"' for k, v in sorted(oauth_params.items())])
    headers = {"Authorization": auth_header, "Content-Type": "application/json"}
    payload = json.dumps({"text": text}).encode()
    req = urllib.request.Request(url, data=payload, headers=headers, method=method)
    try:
        with urllib.request.urlopen(req) as response:
            return json.loads(response.read().decode())['data']['id']
    except Exception as e:
        print(f"❌ Error: {e}")
        return None

if __name__ == "__main__":
    # إرسال التغريدة الرئيسية
    main_text = "🛡️ System Evolution: Pi Swarm v2.4 is now ARMED with Tactical Deception Logic.\n\nWe have integrated multi-path auditing and strategic evasion frameworks to secure the decentralized future. ⛓️\n\n#AISecurity #Solana #PiSwarm"
    tweet_id = send_tweet(main_text)
    if tweet_id:
        print(f"✅ Main Tweet Deployed! ID: {tweet_id}")
        # إرسال الرابط في رد (بأمر منك سأقوم به في الخطوة القادمة لضمان التسلسل)
