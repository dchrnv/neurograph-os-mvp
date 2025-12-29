"""
Async usage examples for NeuroGraph Python client.

This example demonstrates async/await patterns with NeuroGraph.
"""

import asyncio
from neurograph import AsyncNeuroGraphClient


async def main():
    # Initialize async client
    async with AsyncNeuroGraphClient(
        base_url="http://localhost:8000",
        username="developer",
        password="developer123",
    ) as client:

        # Health check
        health = await client.health.check()
        print(f"API Status: {health.status}")

        # Create tokens concurrently
        print("\n=== Creating tokens concurrently ===")
        tasks = [
            client.tokens.create(text=f"token {i}")
            for i in range(5)
        ]
        tokens = await asyncio.gather(*tasks)
        print(f"Created {len(tokens)} tokens")

        # Query tokens
        print("\n=== Querying tokens ===")
        results = await client.tokens.query(
            query_vector=tokens[0].embedding,
            top_k=3,
        )
        for result in results:
            print(f"  - {result.token.text}: {result.similarity:.4f}")

        # Cleanup - delete tokens concurrently
        print("\n=== Cleanup ===")
        delete_tasks = [
            client.tokens.delete(token.id)
            for token in tokens
        ]
        await asyncio.gather(*delete_tasks)
        print("All tokens deleted")


if __name__ == "__main__":
    asyncio.run(main())
