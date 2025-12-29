/**
 * Express.js API example with NeuroGraph integration.
 *
 * This example shows how to build a REST API that proxies
 * NeuroGraph functionality with additional business logic.
 */

import express, { Request, Response, NextFunction } from 'express';
import cors from 'cors';
import dotenv from 'dotenv';
import {
  NeuroGraphClient,
  NeuroGraphError,
  NotFoundError,
  ValidationError,
  RateLimitError,
  retryWithBackoff,
} from '@neurograph/client';

dotenv.config();

const app = express();
const PORT = process.env.PORT || 3001;

// Initialize NeuroGraph client
const neurograph = new NeuroGraphClient({
  baseUrl: process.env.NEUROGRAPH_API_URL || 'http://localhost:8000',
  apiKey: process.env.NEUROGRAPH_API_KEY,
  username: process.env.NEUROGRAPH_USERNAME,
  password: process.env.NEUROGRAPH_PASSWORD,
});

// Middleware
app.use(cors());
app.use(express.json());

// Request logging
app.use((req, res, next) => {
  console.log(`${req.method} ${req.path}`);
  next();
});

// Health check
app.get('/health', async (req: Request, res: Response) => {
  try {
    const health = await neurograph.health.check();
    res.json({ status: 'ok', neurograph: health });
  } catch (error) {
    res.status(503).json({ status: 'error', error: 'Service unavailable' });
  }
});

// Create document (token)
app.post('/documents', async (req: Request, res: Response, next: NextFunction) => {
  try {
    const { text, metadata } = req.body;

    if (!text || typeof text !== 'string') {
      return res.status(400).json({ error: 'Text is required' });
    }

    // Create with retry
    const token = await retryWithBackoff(
      () => neurograph.tokens.create({ text, metadata }),
      { maxRetries: 3 }
    );

    res.status(201).json({
      id: token.id,
      text: token.text,
      metadata: token.metadata,
      created_at: token.created_at,
    });
  } catch (error) {
    next(error);
  }
});

// Get document
app.get('/documents/:id', async (req: Request, res: Response, next: NextFunction) => {
  try {
    const id = parseInt(req.params.id);
    const token = await neurograph.tokens.get(id);

    res.json({
      id: token.id,
      text: token.text,
      metadata: token.metadata,
      created_at: token.created_at,
      updated_at: token.updated_at,
    });
  } catch (error) {
    next(error);
  }
});

// List documents
app.get('/documents', async (req: Request, res: Response, next: NextFunction) => {
  try {
    const limit = parseInt(req.query.limit as string) || 20;
    const offset = parseInt(req.query.offset as string) || 0;

    const tokens = await neurograph.tokens.list({ limit, offset });

    res.json({
      documents: tokens.map((t) => ({
        id: t.id,
        text: t.text,
        metadata: t.metadata,
        created_at: t.created_at,
      })),
      pagination: {
        limit,
        offset,
        count: tokens.length,
      },
    });
  } catch (error) {
    next(error);
  }
});

// Search documents
app.post('/documents/search', async (req: Request, res: Response, next: NextFunction) => {
  try {
    const { query, top_k, threshold } = req.body;

    if (!query || typeof query !== 'string') {
      return res.status(400).json({ error: 'Query is required' });
    }

    // Search using queryByText
    const results = await neurograph.tokens.queryByText({
      text: query,
      topK: top_k || 10,
      threshold: threshold || 0.0,
    });

    res.json({
      query,
      results: results.map((r) => ({
        id: r.token.id,
        text: r.token.text,
        metadata: r.token.metadata,
        similarity: r.similarity,
      })),
    });
  } catch (error) {
    next(error);
  }
});

// Update document
app.put('/documents/:id', async (req: Request, res: Response, next: NextFunction) => {
  try {
    const id = parseInt(req.params.id);
    const { text, metadata } = req.body;

    const updated = await neurograph.tokens.update(id, { text, metadata });

    res.json({
      id: updated.id,
      text: updated.text,
      metadata: updated.metadata,
      updated_at: updated.updated_at,
    });
  } catch (error) {
    next(error);
  }
});

// Delete document
app.delete('/documents/:id', async (req: Request, res: Response, next: NextFunction) => {
  try {
    const id = parseInt(req.params.id);
    await neurograph.tokens.delete(id);
    res.status(204).send();
  } catch (error) {
    next(error);
  }
});

// Batch create documents
app.post('/documents/batch', async (req: Request, res: Response, next: NextFunction) => {
  try {
    const { documents } = req.body;

    if (!Array.isArray(documents)) {
      return res.status(400).json({ error: 'Documents array is required' });
    }

    // Create all documents concurrently with retry
    const promises = documents.map((doc) =>
      retryWithBackoff(() =>
        neurograph.tokens.create({
          text: doc.text,
          metadata: doc.metadata,
        })
      )
    );

    const tokens = await Promise.all(promises);

    res.status(201).json({
      created: tokens.length,
      documents: tokens.map((t) => ({
        id: t.id,
        text: t.text,
        metadata: t.metadata,
      })),
    });
  } catch (error) {
    next(error);
  }
});

// Error handling middleware
app.use((error: any, req: Request, res: Response, next: NextFunction) => {
  console.error('Error:', error);

  if (error instanceof NotFoundError) {
    return res.status(404).json({
      error: 'Not found',
      message: error.message,
    });
  }

  if (error instanceof ValidationError) {
    return res.status(400).json({
      error: 'Validation error',
      message: error.message,
      details: error.details,
    });
  }

  if (error instanceof RateLimitError) {
    return res.status(429).json({
      error: 'Rate limit exceeded',
      message: error.message,
      retry_after: error.retryAfter,
    });
  }

  if (error instanceof NeuroGraphError) {
    return res.status(error.statusCode || 500).json({
      error: 'NeuroGraph error',
      code: error.errorCode,
      message: error.message,
    });
  }

  res.status(500).json({
    error: 'Internal server error',
    message: error.message,
  });
});

// Start server
app.listen(PORT, () => {
  console.log(`Server running on http://localhost:${PORT}`);
  console.log(`NeuroGraph API: ${process.env.NEUROGRAPH_API_URL || 'http://localhost:8000'}`);
});

// Graceful shutdown
process.on('SIGTERM', () => {
  console.log('SIGTERM received, shutting down gracefully...');
  process.exit(0);
});

process.on('SIGINT', () => {
  console.log('SIGINT received, shutting down gracefully...');
  process.exit(0);
});
