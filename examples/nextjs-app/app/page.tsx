'use client';

import { useState, useEffect } from 'react';
import { NeuroGraphClient, type Token, type QueryResult } from '@neurograph/client';

// Initialize client (use environment variable in production)
const client = new NeuroGraphClient({
  baseUrl: process.env.NEXT_PUBLIC_NEUROGRAPH_API_URL || 'http://localhost:8000',
  apiKey: process.env.NEXT_PUBLIC_NEUROGRAPH_API_KEY!,
});

export default function Home() {
  const [tokens, setTokens] = useState<Token[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  const [searchResults, setSearchResults] = useState<QueryResult[]>([]);
  const [newTokenText, setNewTokenText] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Load tokens on mount
  useEffect(() => {
    loadTokens();
  }, []);

  async function loadTokens() {
    try {
      setLoading(true);
      setError(null);
      const data = await client.tokens.list({ limit: 20 });
      setTokens(data);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load tokens');
    } finally {
      setLoading(false);
    }
  }

  async function createToken() {
    if (!newTokenText.trim()) return;

    try {
      setLoading(true);
      setError(null);
      const token = await client.tokens.create({
        text: newTokenText,
        metadata: { source: 'nextjs-app' },
      });
      setTokens([token, ...tokens]);
      setNewTokenText('');
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create token');
    } finally {
      setLoading(false);
    }
  }

  async function searchTokens() {
    if (!searchQuery.trim()) return;

    try {
      setLoading(true);
      setError(null);
      const results = await client.tokens.queryByText({
        text: searchQuery,
        topK: 10,
      });
      setSearchResults(results);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to search');
    } finally {
      setLoading(false);
    }
  }

  async function deleteToken(id: number) {
    try {
      setLoading(true);
      setError(null);
      await client.tokens.delete(id);
      setTokens(tokens.filter((t) => t.id !== id));
      setSearchResults(searchResults.filter((r) => r.token.id !== id));
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to delete token');
    } finally {
      setLoading(false);
    }
  }

  return (
    <main className="min-h-screen p-8 max-w-4xl mx-auto">
      <h1 className="text-4xl font-bold mb-8">NeuroGraph Next.js Example</h1>

      {error && (
        <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
          {error}
        </div>
      )}

      {/* Create Token */}
      <section className="mb-8 p-6 bg-gray-50 rounded-lg">
        <h2 className="text-2xl font-semibold mb-4">Create Token</h2>
        <div className="flex gap-2">
          <input
            type="text"
            value={newTokenText}
            onChange={(e) => setNewTokenText(e.target.value)}
            onKeyPress={(e) => e.key === 'Enter' && createToken()}
            placeholder="Enter token text..."
            className="flex-1 px-4 py-2 border rounded"
            disabled={loading}
          />
          <button
            onClick={createToken}
            disabled={loading || !newTokenText.trim()}
            className="px-6 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:bg-gray-300"
          >
            {loading ? 'Creating...' : 'Create'}
          </button>
        </div>
      </section>

      {/* Search */}
      <section className="mb-8 p-6 bg-gray-50 rounded-lg">
        <h2 className="text-2xl font-semibold mb-4">Semantic Search</h2>
        <div className="flex gap-2 mb-4">
          <input
            type="text"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            onKeyPress={(e) => e.key === 'Enter' && searchTokens()}
            placeholder="Search tokens..."
            className="flex-1 px-4 py-2 border rounded"
            disabled={loading}
          />
          <button
            onClick={searchTokens}
            disabled={loading || !searchQuery.trim()}
            className="px-6 py-2 bg-green-500 text-white rounded hover:bg-green-600 disabled:bg-gray-300"
          >
            {loading ? 'Searching...' : 'Search'}
          </button>
        </div>

        {searchResults.length > 0 && (
          <div className="space-y-2">
            <h3 className="font-semibold">Results:</h3>
            {searchResults.map((result) => (
              <div
                key={result.token.id}
                className="p-3 bg-white border rounded flex justify-between items-center"
              >
                <div>
                  <p className="font-medium">{result.token.text}</p>
                  <p className="text-sm text-gray-500">
                    Similarity: {(result.similarity * 100).toFixed(2)}%
                  </p>
                </div>
                <button
                  onClick={() => deleteToken(result.token.id)}
                  className="px-3 py-1 bg-red-500 text-white text-sm rounded hover:bg-red-600"
                >
                  Delete
                </button>
              </div>
            ))}
          </div>
        )}
      </section>

      {/* Token List */}
      <section className="p-6 bg-gray-50 rounded-lg">
        <div className="flex justify-between items-center mb-4">
          <h2 className="text-2xl font-semibold">Recent Tokens</h2>
          <button
            onClick={loadTokens}
            disabled={loading}
            className="px-4 py-2 bg-gray-500 text-white rounded hover:bg-gray-600 disabled:bg-gray-300"
          >
            {loading ? 'Loading...' : 'Refresh'}
          </button>
        </div>

        {tokens.length === 0 ? (
          <p className="text-gray-500">No tokens yet. Create one above!</p>
        ) : (
          <div className="space-y-2">
            {tokens.map((token) => (
              <div
                key={token.id}
                className="p-3 bg-white border rounded flex justify-between items-center"
              >
                <div>
                  <p className="font-medium">{token.text}</p>
                  <p className="text-sm text-gray-500">ID: {token.id}</p>
                </div>
                <button
                  onClick={() => deleteToken(token.id)}
                  className="px-3 py-1 bg-red-500 text-white text-sm rounded hover:bg-red-600"
                >
                  Delete
                </button>
              </div>
            ))}
          </div>
        )}
      </section>
    </main>
  );
}
