"""
Advanced features examples for NeuroGraph Python client.

This example demonstrates:
- Retry logic with exponential backoff
- Logging configuration
- CLI usage
- Error recovery patterns
"""

from neurograph import (
    NeuroGraphClient,
    RetryConfig,
    retry_with_backoff,
    setup_logging,
    get_logger,
    RateLimitError,
    ServerError,
)
import logging


def example_retry_decorator():
    """Example: Using retry decorator."""
    print("=== Retry Decorator Example ===")

    client = NeuroGraphClient(
        base_url="http://localhost:8000",
        username="developer",
        password="developer123"
    )

    # Simple retry with defaults (3 retries, exponential backoff)
    @retry_with_backoff
    def create_token_with_retry(text):
        return client.tokens.create(text=text)

    # Custom retry configuration
    @retry_with_backoff(config=RetryConfig(
        max_retries=5,
        initial_delay=0.5,
        max_delay=30.0,
        exponential_base=2.0,
        jitter=True
    ))
    def resilient_create(text):
        return client.tokens.create(text=text)

    try:
        # This will automatically retry on rate limit or server errors
        token = create_token_with_retry("resilient token")
        print(f"Created token: {token.id}")

        # Cleanup
        client.tokens.delete(token.id)
    finally:
        client.close()


def example_logging():
    """Example: Logging configuration."""
    print("\n=== Logging Example ===")

    # Enable debug logging
    setup_logging(level=logging.DEBUG)

    # Get logger
    logger = get_logger()
    logger.info("Starting operations...")

    client = NeuroGraphClient(
        base_url="http://localhost:8000",
        username="developer",
        password="developer123"
    )

    try:
        logger.info("Creating token...")
        token = client.tokens.create(text="logged operation")
        logger.info(f"Token created: {token.id}")

        logger.info("Querying tokens...")
        results = client.tokens.query(
            query_vector=token.embedding,
            top_k=5
        )
        logger.info(f"Found {len(results)} results")

        # Cleanup
        logger.info("Cleaning up...")
        client.tokens.delete(token.id)
        logger.info("Done!")

    except Exception as e:
        logger.error(f"Error occurred: {e}", exc_info=True)
    finally:
        client.close()


def example_manual_retry():
    """Example: Manual retry logic."""
    print("\n=== Manual Retry Example ===")

    client = NeuroGraphClient(
        base_url="http://localhost:8000",
        username="developer",
        password="developer123"
    )

    config = RetryConfig(max_retries=3)

    for attempt in range(config.max_retries + 1):
        try:
            token = client.tokens.create(text=f"attempt {attempt}")
            print(f"Success on attempt {attempt + 1}")
            client.tokens.delete(token.id)
            break

        except RateLimitError as e:
            if attempt == config.max_retries:
                print(f"Failed after {config.max_retries} retries")
                raise

            wait_time = e.retry_after or config.get_delay(attempt)
            print(f"Rate limited. Retry in {wait_time:.1f}s...")

            import time
            time.sleep(wait_time)

        except ServerError as e:
            if attempt == config.max_retries:
                raise

            delay = config.get_delay(attempt)
            print(f"Server error. Retry in {delay:.1f}s...")

            import time
            time.sleep(delay)

    client.close()


def example_cli_usage():
    """Example: CLI usage documentation."""
    print("\n=== CLI Usage Examples ===")

    print("""
    # After installing the package, you can use the CLI:

    # Health check
    $ neurograph-cli health

    # Create token
    $ neurograph-cli token create "hello world" --username developer --password developer123

    # List tokens
    $ neurograph-cli token list --limit 10

    # Query tokens
    $ neurograph-cli token query --text "hello" --top-k 5

    # Create API key
    $ neurograph-cli apikey create --name "My Key" --expires 30

    # Enable debug logging
    $ neurograph-cli --debug token list

    # Use API key for auth
    $ neurograph-cli --api-key "ng_your_key_here" token list
    """)


def example_error_patterns():
    """Example: Common error handling patterns."""
    print("\n=== Error Handling Patterns ===")

    from neurograph import (
        NotFoundError,
        AuthenticationError,
        ValidationError,
    )

    client = NeuroGraphClient(
        base_url="http://localhost:8000",
        username="developer",
        password="developer123"
    )

    try:
        # Pattern 1: Graceful degradation
        try:
            token = client.tokens.get(token_id=999999)
        except NotFoundError:
            print("Token not found, creating new one...")
            token = client.tokens.create(text="fallback token")
            client.tokens.delete(token.id)

        # Pattern 2: Retry on specific errors
        max_attempts = 3
        for attempt in range(max_attempts):
            try:
                token = client.tokens.create(text="test")
                client.tokens.delete(token.id)
                break
            except (RateLimitError, ServerError) as e:
                if attempt == max_attempts - 1:
                    raise
                print(f"Transient error, retrying... ({attempt + 1}/{max_attempts})")
                import time
                time.sleep(2 ** attempt)

        # Pattern 3: Different handling per error type
        try:
            token = client.tokens.create(text="test")
        except AuthenticationError:
            print("Re-authenticating...")
            # Re-create client or refresh token
        except ValidationError as e:
            print(f"Invalid input: {e.details}")
            # Fix input and retry
        except RateLimitError as e:
            print(f"Rate limited, wait {e.retry_after}s")
            # Implement backoff
        finally:
            try:
                client.tokens.delete(token.id)
            except:
                pass

    finally:
        client.close()


def main():
    """Run all examples."""
    example_retry_decorator()
    example_logging()
    example_manual_retry()
    example_cli_usage()
    example_error_patterns()


if __name__ == "__main__":
    main()
