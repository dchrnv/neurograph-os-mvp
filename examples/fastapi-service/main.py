"""
FastAPI service example with NeuroGraph Python client integration.

This example shows server-side semantic search service using FastAPI.
"""

from fastapi import FastAPI, HTTPException, Query
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel, Field
from typing import List, Optional, Dict, Any
import asyncio
from contextlib import asynccontextmanager

from neurograph import AsyncNeuroGraphClient, NotFoundError, ValidationError
from neurograph import async_retry_with_backoff, setup_logging

import os
import logging

# Setup logging
setup_logging(level=logging.INFO)


# Pydantic models
class DocumentCreate(BaseModel):
    text: str = Field(..., min_length=1)
    metadata: Optional[Dict[str, Any]] = None


class DocumentUpdate(BaseModel):
    text: Optional[str] = None
    metadata: Optional[Dict[str, Any]] = None


class DocumentResponse(BaseModel):
    id: int
    text: str
    metadata: Dict[str, Any]
    created_at: str
    updated_at: str


class SearchRequest(BaseModel):
    query: str = Field(..., min_length=1)
    top_k: int = Field(default=10, ge=1, le=100)
    threshold: Optional[float] = Field(default=None, ge=0.0, le=1.0)


class SearchResult(BaseModel):
    id: int
    text: str
    metadata: Dict[str, Any]
    similarity: float


class SearchResponse(BaseModel):
    query: str
    results: List[SearchResult]


class BatchCreateRequest(BaseModel):
    documents: List[DocumentCreate]


class BatchCreateResponse(BaseModel):
    created: int
    documents: List[DocumentResponse]


# Global NeuroGraph client
neurograph_client: Optional[AsyncNeuroGraphClient] = None


@asynccontextmanager
async def lifespan(app: FastAPI):
    """Application lifespan manager."""
    global neurograph_client

    # Startup
    api_url = os.getenv("NEUROGRAPH_API_URL", "http://localhost:8000")
    api_key = os.getenv("NEUROGRAPH_API_KEY")
    username = os.getenv("NEUROGRAPH_USERNAME", "developer")
    password = os.getenv("NEUROGRAPH_PASSWORD", "developer123")

    neurograph_client = AsyncNeuroGraphClient(
        base_url=api_url,
        api_key=api_key,
        username=username if not api_key else None,
        password=password if not api_key else None,
    )

    logging.info(f"Connected to NeuroGraph at {api_url}")

    yield

    # Shutdown
    if neurograph_client:
        await neurograph_client.close()
        logging.info("NeuroGraph client closed")


# Create FastAPI app
app = FastAPI(
    title="NeuroGraph FastAPI Service",
    description="Semantic document management service powered by NeuroGraph",
    version="1.0.0",
    lifespan=lifespan,
)

# CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # Configure appropriately for production
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.get("/health")
async def health_check():
    """Health check endpoint."""
    try:
        health = await neurograph_client.health.check()
        return {
            "status": "ok",
            "neurograph": {
                "status": health.status,
                "version": health.version,
            },
        }
    except Exception as e:
        raise HTTPException(status_code=503, detail=f"Service unavailable: {str(e)}")


@app.post("/documents", response_model=DocumentResponse, status_code=201)
@async_retry_with_backoff
async def create_document(doc: DocumentCreate):
    """Create a new document."""
    try:
        token = await neurograph_client.tokens.create(
            text=doc.text, metadata=doc.metadata or {}
        )
        return DocumentResponse(
            id=token.id,
            text=token.text,
            metadata=token.metadata,
            created_at=token.created_at,
            updated_at=token.updated_at,
        )
    except ValidationError as e:
        raise HTTPException(status_code=400, detail=str(e))
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.get("/documents/{document_id}", response_model=DocumentResponse)
async def get_document(document_id: int):
    """Get document by ID."""
    try:
        token = await neurograph_client.tokens.get(token_id=document_id)
        return DocumentResponse(
            id=token.id,
            text=token.text,
            metadata=token.metadata,
            created_at=token.created_at,
            updated_at=token.updated_at,
        )
    except NotFoundError:
        raise HTTPException(status_code=404, detail="Document not found")
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.get("/documents", response_model=List[DocumentResponse])
async def list_documents(
    limit: int = Query(default=20, ge=1, le=1000),
    offset: int = Query(default=0, ge=0),
):
    """List documents with pagination."""
    try:
        tokens = await neurograph_client.tokens.list(limit=limit, offset=offset)
        return [
            DocumentResponse(
                id=t.id,
                text=t.text,
                metadata=t.metadata,
                created_at=t.created_at,
                updated_at=t.updated_at,
            )
            for t in tokens
        ]
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.put("/documents/{document_id}", response_model=DocumentResponse)
async def update_document(document_id: int, doc: DocumentUpdate):
    """Update document."""
    try:
        token = await neurograph_client.tokens.update(
            token_id=document_id, text=doc.text, metadata=doc.metadata
        )
        return DocumentResponse(
            id=token.id,
            text=token.text,
            metadata=token.metadata,
            created_at=token.created_at,
            updated_at=token.updated_at,
        )
    except NotFoundError:
        raise HTTPException(status_code=404, detail="Document not found")
    except ValidationError as e:
        raise HTTPException(status_code=400, detail=str(e))
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.delete("/documents/{document_id}", status_code=204)
async def delete_document(document_id: int):
    """Delete document."""
    try:
        await neurograph_client.tokens.delete(token_id=document_id)
    except NotFoundError:
        raise HTTPException(status_code=404, detail="Document not found")
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.post("/documents/search", response_model=SearchResponse)
async def search_documents(search: SearchRequest):
    """Semantic search for documents."""
    try:
        # Create query token
        query_token = await neurograph_client.tokens.create(text=search.query)

        # Search
        results = await neurograph_client.tokens.query(
            query_vector=query_token.embedding,
            top_k=search.top_k,
            threshold=search.threshold,
        )

        # Cleanup query token
        await neurograph_client.tokens.delete(token_id=query_token.id)

        return SearchResponse(
            query=search.query,
            results=[
                SearchResult(
                    id=r.token.id,
                    text=r.token.text,
                    metadata=r.token.metadata,
                    similarity=r.similarity,
                )
                for r in results
            ],
        )
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


@app.post("/documents/batch", response_model=BatchCreateResponse, status_code=201)
async def batch_create_documents(batch: BatchCreateRequest):
    """Batch create documents."""
    try:
        # Create all documents concurrently
        tasks = [
            neurograph_client.tokens.create(text=doc.text, metadata=doc.metadata or {})
            for doc in batch.documents
        ]
        tokens = await asyncio.gather(*tasks)

        return BatchCreateResponse(
            created=len(tokens),
            documents=[
                DocumentResponse(
                    id=t.id,
                    text=t.text,
                    metadata=t.metadata,
                    created_at=t.created_at,
                    updated_at=t.updated_at,
                )
                for t in tokens
            ],
        )
    except ValidationError as e:
        raise HTTPException(status_code=400, detail=str(e))
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))


if __name__ == "__main__":
    import uvicorn

    uvicorn.run(app, host="0.0.0.0", port=8001)
