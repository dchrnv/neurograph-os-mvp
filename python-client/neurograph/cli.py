"""
NeuroGraph CLI - Command-line interface for NeuroGraph API.

Usage:
    neurograph-cli health
    neurograph-cli token create "hello world"
    neurograph-cli token get 123
    neurograph-cli token list --limit 10
    neurograph-cli token query --text "hello" --top-k 5
    neurograph-cli apikey create --name "My Key" --expires 30
    neurograph-cli apikey list
"""

import sys
import json
from typing import Optional
import argparse

from .client import NeuroGraphClient
from .exceptions import NeuroGraphError
from .logging import setup_logging, get_logger
import logging


def create_client(args) -> NeuroGraphClient:
    """Create client from CLI arguments."""
    return NeuroGraphClient(
        base_url=args.base_url,
        username=args.username,
        password=args.password,
        api_key=args.api_key,
    )


def cmd_health(args):
    """Health check command."""
    client = create_client(args)
    try:
        health = client.health.check()
        print(json.dumps({
            "status": health.status,
            "version": health.version,
        }, indent=2))
    finally:
        client.close()


def cmd_status(args):
    """System status command."""
    client = create_client(args)
    try:
        status = client.health.status()
        print(json.dumps({
            "api_version": status.api_version,
            "runtime_version": status.runtime_version,
            "tokens_count": status.tokens_count,
            "uptime_seconds": status.uptime_seconds,
            "memory_usage_mb": status.memory_usage_mb,
        }, indent=2))
    finally:
        client.close()


def cmd_token_create(args):
    """Create token command."""
    client = create_client(args)
    try:
        token = client.tokens.create(text=args.text, metadata=args.metadata)
        print(json.dumps({
            "id": token.id,
            "text": token.text,
            "embedding": token.embedding if args.show_embedding else f"[{len(token.embedding)} dimensions]",
            "metadata": token.metadata,
        }, indent=2))
    finally:
        client.close()


def cmd_token_get(args):
    """Get token command."""
    client = create_client(args)
    try:
        token = client.tokens.get(token_id=args.token_id)
        print(json.dumps({
            "id": token.id,
            "text": token.text,
            "embedding": token.embedding if args.show_embedding else f"[{len(token.embedding)} dimensions]",
            "metadata": token.metadata,
        }, indent=2))
    finally:
        client.close()


def cmd_token_list(args):
    """List tokens command."""
    client = create_client(args)
    try:
        tokens = client.tokens.list(limit=args.limit, offset=args.offset)
        for token in tokens:
            print(json.dumps({
                "id": token.id,
                "text": token.text,
                "metadata": token.metadata,
            }))
    finally:
        client.close()


def cmd_token_delete(args):
    """Delete token command."""
    client = create_client(args)
    try:
        client.tokens.delete(token_id=args.token_id)
        print(f"Token {args.token_id} deleted")
    finally:
        client.close()


def cmd_token_query(args):
    """Query tokens command."""
    client = create_client(args)
    try:
        # Get query vector from text
        if args.text:
            query_token = client.tokens.create(text=args.text)
            query_vector = query_token.embedding
            # Clean up temporary token
            client.tokens.delete(query_token.id)
        else:
            print("Error: --text is required for query", file=sys.stderr)
            sys.exit(1)

        # Query
        results = client.tokens.query(
            query_vector=query_vector,
            top_k=args.top_k,
        )

        for result in results:
            print(json.dumps({
                "token_id": result.token.id,
                "text": result.token.text,
                "similarity": result.similarity,
            }))
    finally:
        client.close()


def cmd_apikey_create(args):
    """Create API key command."""
    client = create_client(args)
    try:
        api_key = client.api_keys.create(
            name=args.name,
            scopes=args.scopes,
            expires_in_days=args.expires,
        )
        print(json.dumps({
            "key_id": api_key.key_id,
            "api_key": api_key.api_key,
            "name": api_key.name,
            "scopes": api_key.scopes,
        }, indent=2))
        print("\n⚠️  Save this key! It won't be shown again.", file=sys.stderr)
    finally:
        client.close()


def cmd_apikey_list(args):
    """List API keys command."""
    client = create_client(args)
    try:
        keys = client.api_keys.list()
        for key in keys:
            print(json.dumps({
                "key_id": key.key_id,
                "name": key.name,
                "key_prefix": key.key_prefix,
                "is_active": key.is_active,
                "scopes": key.scopes,
            }))
    finally:
        client.close()


def cmd_apikey_revoke(args):
    """Revoke API key command."""
    client = create_client(args)
    try:
        client.api_keys.revoke(key_id=args.key_id)
        print(f"API key {args.key_id} revoked")
    finally:
        client.close()


def main():
    """Main CLI entry point."""
    parser = argparse.ArgumentParser(
        description="NeuroGraph CLI - Command-line interface for NeuroGraph API"
    )

    # Global arguments
    parser.add_argument(
        "--base-url",
        default="http://localhost:8000",
        help="API base URL (default: http://localhost:8000)",
    )
    parser.add_argument("--username", help="Username for authentication")
    parser.add_argument("--password", help="Password for authentication")
    parser.add_argument("--api-key", help="API key for authentication")
    parser.add_argument("--debug", action="store_true", help="Enable debug logging")

    subparsers = parser.add_subparsers(dest="command", help="Command to run")

    # Health command
    parser_health = subparsers.add_parser("health", help="Check API health")
    parser_health.set_defaults(func=cmd_health)

    # Status command
    parser_status = subparsers.add_parser("status", help="Get system status")
    parser_status.set_defaults(func=cmd_status)

    # Token commands
    parser_token = subparsers.add_parser("token", help="Token operations")
    token_subparsers = parser_token.add_subparsers(dest="token_command")

    # token create
    parser_token_create = token_subparsers.add_parser("create", help="Create token")
    parser_token_create.add_argument("text", help="Text for token")
    parser_token_create.add_argument("--metadata", type=json.loads, help="Metadata (JSON)")
    parser_token_create.add_argument("--show-embedding", action="store_true", help="Show full embedding")
    parser_token_create.set_defaults(func=cmd_token_create)

    # token get
    parser_token_get = token_subparsers.add_parser("get", help="Get token")
    parser_token_get.add_argument("token_id", type=int, help="Token ID")
    parser_token_get.add_argument("--show-embedding", action="store_true", help="Show full embedding")
    parser_token_get.set_defaults(func=cmd_token_get)

    # token list
    parser_token_list = token_subparsers.add_parser("list", help="List tokens")
    parser_token_list.add_argument("--limit", type=int, default=10, help="Limit (default: 10)")
    parser_token_list.add_argument("--offset", type=int, default=0, help="Offset (default: 0)")
    parser_token_list.set_defaults(func=cmd_token_list)

    # token delete
    parser_token_delete = token_subparsers.add_parser("delete", help="Delete token")
    parser_token_delete.add_argument("token_id", type=int, help="Token ID")
    parser_token_delete.set_defaults(func=cmd_token_delete)

    # token query
    parser_token_query = token_subparsers.add_parser("query", help="Query similar tokens")
    parser_token_query.add_argument("--text", required=True, help="Query text")
    parser_token_query.add_argument("--top-k", type=int, default=10, help="Top K results (default: 10)")
    parser_token_query.set_defaults(func=cmd_token_query)

    # API Key commands
    parser_apikey = subparsers.add_parser("apikey", help="API key operations")
    apikey_subparsers = parser_apikey.add_subparsers(dest="apikey_command")

    # apikey create
    parser_apikey_create = apikey_subparsers.add_parser("create", help="Create API key")
    parser_apikey_create.add_argument("--name", required=True, help="Key name")
    parser_apikey_create.add_argument("--scopes", nargs="+", help="Scopes")
    parser_apikey_create.add_argument("--expires", type=int, help="Expiration in days")
    parser_apikey_create.set_defaults(func=cmd_apikey_create)

    # apikey list
    parser_apikey_list = apikey_subparsers.add_parser("list", help="List API keys")
    parser_apikey_list.set_defaults(func=cmd_apikey_list)

    # apikey revoke
    parser_apikey_revoke = apikey_subparsers.add_parser("revoke", help="Revoke API key")
    parser_apikey_revoke.add_argument("key_id", help="Key ID")
    parser_apikey_revoke.set_defaults(func=cmd_apikey_revoke)

    # Parse arguments
    args = parser.parse_args()

    # Setup logging
    if args.debug:
        setup_logging(level=logging.DEBUG)
    else:
        setup_logging(level=logging.WARNING)

    # Check for command
    if not hasattr(args, "func"):
        parser.print_help()
        sys.exit(1)

    # Run command
    try:
        args.func(args)
    except NeuroGraphError as e:
        print(f"Error: {e.message}", file=sys.stderr)
        if args.debug:
            print(f"Error code: {e.error_code}", file=sys.stderr)
            print(f"Details: {e.details}", file=sys.stderr)
        sys.exit(1)
    except KeyboardInterrupt:
        print("\nAborted", file=sys.stderr)
        sys.exit(130)
    except Exception as e:
        print(f"Unexpected error: {e}", file=sys.stderr)
        if args.debug:
            import traceback
            traceback.print_exc()
        sys.exit(1)


if __name__ == "__main__":
    main()
