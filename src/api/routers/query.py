"""
Query Endpoints

Endpoints for semantic queries.
"""

from fastapi import APIRouter, Depends, HTTPException, status
from ..models.response import ApiResponse
from ..models.query import QueryRequest, QueryResponse, TokenResult
from ..dependencies import get_runtime
import time
import uuid
import logging

logger = logging.getLogger(__name__)
router = APIRouter()


@router.post("/query", response_model=ApiResponse)
async def query(
    request: QueryRequest,
    runtime=Depends(get_runtime)
):
    """
    Execute semantic query.

    Search for semantically similar tokens based on input text.

    Args:
        request: Query parameters
        runtime: NeuroGraph runtime instance

    Returns:
        Query results with matching tokens

    Raises:
        HTTPException: If runtime not initialized or query fails
    """
    start_time = time.time()

    # Check if runtime is available
    if runtime is None:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="Runtime not initialized. Bootstrap required."
        )

    try:
        # Generate signal ID for feedback
        signal_id = str(uuid.uuid4())

        # TODO: Execute actual query through runtime
        # For now, return mock data
        logger.info(f"Query: '{request.text}' (limit={request.limit})")

        # Mock results
        tokens = [
            TokenResult(
                token_id=None,
                label=f"result_{i}",
                score=0.9 - (i * 0.05),
                entity_type="Concept"
            )
            for i in range(min(request.limit, 5))
        ]

        processing_time = (time.time() - start_time) * 1000

        response_data = QueryResponse(
            signal_id=signal_id,
            query_text=request.text,
            processing_time_ms=processing_time,
            confidence=0.85,
            interpretation="semantic_query",
            tokens=tokens,
            connections=[] if not request.include_connections else None,
            total_candidates=len(tokens)
        )

        return ApiResponse.success_response(
            response_data.dict(),
            processing_time_ms=processing_time
        )

    except Exception as e:
        logger.error(f"Query failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Query execution failed: {str(e)}"
        )
