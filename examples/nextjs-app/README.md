# NeuroGraph Next.js Example

Example Next.js 14 application demonstrating NeuroGraph TypeScript client integration.

## Features

- Create tokens with semantic embeddings
- Search tokens by semantic similarity
- List and delete tokens
- Real-time UI updates
- Error handling
- TypeScript + React

## Setup

1. Install dependencies:
```bash
npm install
```

2. Create `.env.local`:
```bash
NEXT_PUBLIC_NEUROGRAPH_API_URL=http://localhost:8000
NEXT_PUBLIC_NEUROGRAPH_API_KEY=your_api_key_here
```

3. Start development server:
```bash
npm run dev
```

4. Open http://localhost:3000

## Environment Variables

- `NEXT_PUBLIC_NEUROGRAPH_API_URL` - NeuroGraph API base URL
- `NEXT_PUBLIC_NEUROGRAPH_API_KEY` - API key for authentication

## Production Deployment

```bash
npm run build
npm start
```

## Vercel Deployment

1. Push to GitHub
2. Import project in Vercel
3. Set environment variables
4. Deploy

## Security Notes

- Never commit `.env.local` to git
- Use API keys with minimal required scopes
- Consider server-side API proxy for production
- Enable CORS on API server
