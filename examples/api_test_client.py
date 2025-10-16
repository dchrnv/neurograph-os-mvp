"""
REST API test client for NeuroGraph OS.
"""

import httpx
import asyncio
from typing import Dict, Any
import json


class NeuroGraphAPIClient:
    """HTTP client for NeuroGraph OS REST API."""
    
    def __init__(self, base_url: str = "http://localhost:8000"):
        self.base_url = base_url
        self.client = httpx.AsyncClient(base_url=base_url, timeout=30.0)
    
    async def close(self):
        """Close the client."""
        await self.client.aclose()
    
    # System endpoints
    
    async def health_check(self) -> Dict[str, Any]:
        """Check system health."""
        response = await self.client.get("/health")
        response.raise_for_status()
        return response.json()
    
    async def get_system_stats(self) -> Dict[str, Any]:
        """Get system statistics."""
        response = await self.client.get("/api/v1/system/stats")
        response.raise_for_status()
        return response.json()
    
    async def get_system_info(self) -> Dict[str, Any]:
        """Get system information."""
        response = await self.client.get("/api/v1/system/info")
        response.raise_for_status()
        return response.json()
    
    # Token endpoints
    
    async def create_token(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """Create a token."""
        response = await self.client.post("/api/v1/tokens/", json=data)
        response.raise_for_status()
        return response.json()
    
    async def list_tokens(self, limit: int = 10, offset: int = 0) -> Dict[str, Any]:
        """List tokens."""
        response = await self.client.get(
            "/api/v1/tokens/",
            params={"limit": limit, "offset": offset}
        )
        response.raise_for_status()
        return response.json()
    
    async def get_token(self, token_id: str) -> Dict[str, Any]:
        """Get a token by ID."""
        response = await self.client.get(f"/api/v1/tokens/{token_id}")
        response.raise_for_status()
        return response.json()
    
    async def update_token(self, token_id: str, data: Dict[str, Any]) -> Dict[str, Any]:
        """Update a token."""
        response = await self.client.put(f"/api/v1/tokens/{token_id}", json=data)
        response.raise_for_status()
        return response.json()
    
    async def delete_token(self, token_id: str):
        """Delete a token."""
        response = await self.client.delete(f"/api/v1/tokens/{token_id}")
        response.raise_for_status()
    
    async def spatial_search(self, **kwargs) -> Dict[str, Any]:
        """Search tokens in spatial region."""
        response = await self.client.post("/api/v1/tokens/search/spatial", json=kwargs)
        response.raise_for_status()
        return response.json()
    
    async def count_tokens(self, token_type: str = None) -> Dict[str, Any]:
        """Count tokens."""
        params = {"token_type": token_type} if token_type else {}
        response = await self.client.get("/api/v1/tokens/count/total", params=params)
        response.raise_for_status()
        return response.json()
    
    # Graph endpoints
    
    async def create_connection(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """Create a graph connection."""
        response = await self.client.post("/api/v1/graph/connections", json=data)
        response.raise_for_status()
        return response.json()
    
    async def list_connections(self, limit: int = 10, offset: int = 0) -> Dict[str, Any]:
        """List connections."""
        response = await self.client.get(
            "/api/v1/graph/connections",
            params={"limit": limit, "offset": offset}
        )
        response.raise_for_status()
        return response.json()
    
    async def get_neighbors(self, token_id: str, direction: str = "both") -> Dict[str, Any]:
        """Get token neighbors."""
        response = await self.client.get(
            f"/api/v1/graph/tokens/{token_id}/neighbors",
            params={"direction": direction}
        )
        response.raise_for_status()
        return response.json()
    
    async def get_degree(self, token_id: str) -> Dict[str, Any]:
        """Get token degree."""
        response = await self.client.get(f"/api/v1/graph/tokens/{token_id}/degree")
        response.raise_for_status()
        return response.json()
    
    async def find_path(self, source_id: str, target_id: str, max_depth: int = 5) -> Dict[str, Any]:
        """Find path between tokens."""
        response = await self.client.get(
            "/api/v1/graph/path",
            params={"source_id": source_id, "target_id": target_id, "max_depth": max_depth}
        )
        response.raise_for_status()
        return response.json()
    
    async def get_graph_stats(self) -> Dict[str, Any]:
        """Get graph statistics."""
        response = await self.client.get("/api/v1/graph/stats")
        response.raise_for_status()
        return response.json()
    
    # Experience endpoints
    
    async def create_experience_event(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """Create an experience event."""
        response = await self.client.post("/api/v1/experience/events", json=data)
        response.raise_for_status()
        return response.json()
    
    async def list_experience_events(self, limit: int = 10, offset: int = 0) -> Dict[str, Any]:
        """List experience events."""
        response = await self.client.get(
            "/api/v1/experience/events",
            params={"limit": limit, "offset": offset}
        )
        response.raise_for_status()
        return response.json()


async def demo():
    """Run API demo."""
    client = NeuroGraphAPIClient()
    
    try:
        print("╔══════════════════════════════════════════════╗")
        print("║    NeuroGraph OS - REST API Demo            ║")
        print("╚══════════════════════════════════════════════╝\n")
        
        # 1. Health check
        print("1️⃣  Health Check")
        health = await client.health_check()
        print(f"   Status: {health['status']}")
        print(f"   WS Connections: {health['websocket_connections']}\n")
        
        # 2. System stats
        print("2️⃣  System Statistics")
        stats = await client.get_system_stats()
        print(f"   Tokens: {stats['tokens']}")
        print(f"   Connections: {stats['connections']}")
        print(f"   Events: {stats['experience_events']}\n")
        
        # 3. Create tokens
        print("3️⃣  Creating Tokens")
        token1_data = {
            "type": "demo",
            "coordinates": {
                "x": [1.0, 0, 0, 0, 0, 0, 0, 0],
                "y": [0, 0, 0, 0, 0, 0, 0, 0],
                "z": [0, 0, 0, 0, 0, 0, 0, 0]
            },
            "weight": 1.0,
            "metadata": {"description": "Demo token 1"}
        }
        
        token1 = await client.create_token(token1_data)
        token1_id = token1['id']
        print(f"   ✓ Token 1: {token1_id[:8]}...")
        
        token2_data = {
            "type": "demo",
            "coordinates": {
                "x": [2.0, 0, 0, 0, 0, 0, 0, 0],
                "y": [1.0, 0, 0, 0, 0, 0, 0, 0],
                "z": [0, 0, 0, 0, 0, 0, 0, 0]
            },
            "weight": 1.5
        }
        
        token2 = await client.create_token(token2_data)
        token2_id = token2['id']
        print(f"   ✓ Token 2: {token2_id[:8]}...\n")
        
        # 4. List tokens
        print("4️⃣  Listing Tokens")
        token_list = await client.list_tokens(limit=5)
        print(f"   Total: {token_list['total']}")
        print(f"   Showing: {len(token_list['tokens'])}")
        for token in token_list['tokens'][:3]:
            print(f"     • {token['id'][:8]}... ({token['type']})")
        print()
        
        # 5. Get token details
        print("5️⃣  Get Token Details")
        token_details = await client.get_token(token1_id)
        print(f"   ID: {token_details['id']}")
        print(f"   Type: {token_details['type']}")
        print(f"   Weight: {token_details['weight']}")
        print(f"   Coordinates: ({token_details['coordinates']['x'][0]:.1f}, "
              f"{token_details['coordinates']['y'][0]:.1f}, "
              f"{token_details['coordinates']['z'][0]:.1f})\n")
        
        # 6. Update token
        print("6️⃣  Update Token")
        updated = await client.update_token(token1_id, {"weight": 2.0})
        print(f"   Weight updated: {updated['weight']}\n")
        
        # 7. Create connection
        print("7️⃣  Create Graph Connection")
        connection_data = {
            "source_id": token1_id,
            "target_id": token2_id,
            "connection_type": "spatial",
            "weight": 0.8
        }
        connection = await client.create_connection(connection_data)
        print(f"   ✓ Connection: {token1_id[:8]}... → {token2_id[:8]}...")
        print(f"   Type: {connection['connection_type']}")
        print(f"   Weight: {connection['weight']}\n")
        
        # 8. Get neighbors
        print("8️⃣  Get Token Neighbors")
        neighbors = await client.get_neighbors(token1_id)
        print(f"   Token: {token1_id[:8]}...")
        print(f"   Neighbors: {neighbors['count']}")
        for conn in neighbors['neighbors']:
            print(f"     → {conn['target_id'][:8]}... ({conn['connection_type']})")
        print()
        
        # 9. Get degree
        print("9️⃣  Get Token Degree")
        degree = await client.get_degree(token1_id)
        print(f"   In-degree: {degree['in_degree']}")
        print(f"   Out-degree: {degree['out_degree']}")
        print(f"   Total: {degree['total_degree']}\n")
        
        # 10. Graph stats
        print("🔟 Graph Statistics")
        graph_stats = await client.get_graph_stats()
        print(f"   Nodes: {graph_stats['total_nodes']}")
        print(f"   Edges: {graph_stats['total_edges']}")
        print(f"   Avg Degree: {graph_stats['avg_degree']:.2f}")
        print(f"   Density: {graph_stats['density']:.4f}\n")
        
        # 11. Spatial search
        print("1️⃣1️⃣  Spatial Search")
        search_results = await client.spatial_search(
            min_x=0, min_y=0, min_z=0,
            max_x=3, max_y=3, max_z=3,
            level=0
        )
        print(f"   Found: {search_results['total']} token(s) in region")
        for token in search_results['tokens'][:3]:
            coords = token['coordinates']
            print(f"     • {token['id'][:8]}... at ({coords['x'][0]:.1f}, "
                  f"{coords['y'][0]:.1f}, {coords['z'][0]:.1f})")
        print()
        
        # 12. Create experience event
        print("1️⃣2️⃣  Create Experience Event")
        event_data = {
            "event_type": "demo_action",
            "token_id": token1_id,
            "state_before": {"value": 0},
            "state_after": {"value": 1},
            "action": {"type": "increment"},
            "reward": 1.0,
            "metadata": {"demo": True}
        }
        event = await client.create_experience_event(event_data)
        print(f"   ✓ Event: {event['id'][:8]}...")
        print(f"   Type: {event['event_type']}")
        print(f"   Reward: {event['reward']}\n")
        
        # 13. List experience events
        print("1️⃣3️⃣  List Experience Events")
        events = await client.list_experience_events(limit=3)
        print(f"   Total: {events['total']}")
        for evt in events['events'][:3]:
            print(f"     • {evt['event_type']} (reward: {evt['reward']})")
        print()
        
        # 14. Count tokens
        print("1️⃣4️⃣  Count Tokens")
        count = await client.count_tokens()
        print(f"   Total tokens: {count['count']}")
        
        count_demo = await client.count_tokens(token_type="demo")
        print(f"   Demo tokens: {count_demo['count']}\n")
        
        # 15. Delete token
        print("1️⃣5️⃣  Delete Token")
        await client.delete_token(token2_id)
        print(f"   ✓ Deleted: {token2_id[:8]}...\n")
        
        print("✨ Demo completed successfully!")
    
    except httpx.HTTPError as e:
        print(f"\n❌ HTTP Error: {e}")
    except Exception as e:
        print(f"\n❌ Error: {e}")
    finally:
        await client.close()


async def interactive():
    """Interactive API client."""
    client = NeuroGraphAPIClient()
    
    print("\n╔══════════════════════════════════════════════╗")
    print("║   NeuroGraph OS - Interactive API Client    ║")
    print("╚══════════════════════════════════════════════╝\n")
    
    print("Commands:")
    print("  health              - Check health")
    print("  stats               - System stats")
    print("  create-token        - Create a token")
    print("  list-tokens         - List tokens")
    print("  create-connection   - Create connection")
    print("  graph-stats         - Graph statistics")
    print("  quit                - Exit\n")
    
    try:
        while True:
            cmd = input(">>> ").strip().lower()
            
            if not cmd:
                continue
            
            if cmd == "quit":
                break
            
            try:
                if cmd == "health":
                    result = await client.health_check()
                    print(json.dumps(result, indent=2))
                
                elif cmd == "stats":
                    result = await client.get_system_stats()
                    print(json.dumps(result, indent=2))
                
                elif cmd == "create-token":
                    data = {
                        "type": "interactive",
                        "coordinates": {
                            "x": [1.0] + [0] * 7,
                            "y": [0] * 8,
                            "z": [0] * 8
                        }
                    }
                    result = await client.create_token(data)
                    print(f"Created: {result['id']}")
                
                elif cmd == "list-tokens":
                    result = await client.list_tokens(limit=5)
                    print(f"Total: {result['total']}")
                    for token in result['tokens']:
                        print(f"  • {token['id'][:8]}... ({token['type']})")
                
                elif cmd == "graph-stats":
                    result = await client.get_graph_stats()
                    print(json.dumps(result, indent=2))
                
                else:
                    print(f"Unknown command: {cmd}")
            
            except Exception as e:
                print(f"Error: {e}")
    
    except KeyboardInterrupt:
        print("\nExiting...")
    finally:
        await client.close()


if __name__ == "__main__":
    import sys
    
    mode = sys.argv[1] if len(sys.argv) > 1 else "demo"
    
    if mode == "demo":
        asyncio.run(demo())
    elif mode == "interactive":
        asyncio.run(interactive())
    else:
        print(f"Unknown mode: {mode}")
        print("Usage: python api_test_client.py [demo|interactive]")