"""
🧠 محرك الذاكرة الرسومي (Memory Graph Engine) - النسخة المصححة
مستوحى من Spacebot Architecture لتعزيز ذكاء السرب
"""

import json
import os
from datetime import datetime
from typing import List, Dict, Optional

class MemoryNode:
    def __init__(self, node_id: str, label: str, properties: Dict):
        self.node_id = node_id
        self.label = label
        self.properties = properties
        self.created_at = datetime.now().isoformat()
    
    def to_dict(self):
        return {
            "node_id": self.node_id,
            "label": self.label,
            "properties": self.properties,
            "created_at": self.created_at
        }

class MemoryEdge:
    def __init__(self, source_id: str, target_id: str, relation: str):
        self.source_id = source_id
        self.target_id = target_id
        self.relation = relation

    def to_dict(self):
        return {
            "source_id": self.source_id,
            "target_id": self.target_id,
            "relation": self.relation
        }

class SovereignGraphMemory:
    def __init__(self, storage_path: str = "swarm_memory.json"):
        self.storage_path = storage_path
        self.nodes: Dict[str, MemoryNode] = {}
        self.edges: List[MemoryEdge] = []
        self.load_memory()

    def add_node(self, node_id: str, label: str, properties: Dict):
        node = MemoryNode(node_id, label, properties)
        self.nodes[node_id] = node
        self.save_memory()
        print(f"🧠 [Memory] New node added: {label} ({node_id})")

    def add_edge(self, source_id: str, target_id: str, relation: str):
        if source_id in self.nodes and target_id in self.nodes:
            edge = MemoryEdge(source_id, target_id, relation)
            self.edges.append(edge)
            self.save_memory()
            print(f"🔗 [Memory] New link: {source_id} --[{relation}]--> {target_id}")

    def query_related(self, node_id: str) -> List[Dict]:
        related = []
        for edge in self.edges:
            if edge.source_id == node_id:
                node = self.nodes.get(edge.target_id)
                related.append({"target": edge.target_id, "relation": edge.relation, "data": node.to_dict() if node else None})
            elif edge.target_id == node_id:
                node = self.nodes.get(edge.source_id)
                related.append({"source": edge.source_id, "relation": edge.relation, "data": node.to_dict() if node else None})
        return related

    def save_memory(self):
        data = {
            "nodes": {k: v.to_dict() for k, v in self.nodes.items()},
            "edges": [e.to_dict() for e in self.edges],
            "updated_at": datetime.now().isoformat()
        }
        with open(self.storage_path, "w", encoding="utf-8") as f:
            json.dump(data, f, indent=2, ensure_ascii=False)

    def load_memory(self):
        if os.path.exists(self.storage_path):
            try:
                with open(self.storage_path, "r", encoding="utf-8") as f:
                    data = json.load(f)
                    for k, v in data.get("nodes", {}).items():
                        node = MemoryNode(v['node_id'], v['label'], v['properties'])
                        node.created_at = v['created_at']
                        self.nodes[k] = node
                    for e in data.get("edges", []):
                        self.edges.append(MemoryEdge(e['source_id'], e['target_id'], e['relation']))
            except Exception as e:
                print(f"⚠️ Error loading memory: {e}")

if __name__ == "__main__":
    mem = SovereignGraphMemory()
    mem.add_node("target_1", "Target", {"ip": "192.168.1.1", "os": "Linux"})
    mem.add_node("vuln_cve_2021", "Vulnerability", {"id": "CVE-2021-44228", "severity": "Critical"})
    mem.add_edge("target_1", "vuln_cve_2021", "AFFECTED_BY")
    
    print("\n📊 Querying relations for target_1:")
    print(json.dumps(mem.query_related("target_1"), indent=2, ensure_ascii=False))
