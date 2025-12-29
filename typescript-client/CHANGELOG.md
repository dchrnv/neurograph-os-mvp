# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.59.2] - 2025-12-29

### Added
- Initial release of TypeScript/JavaScript client
- Full TypeScript support with comprehensive type definitions
- JWT and API Key authentication
- Automatic JWT token refresh
- Comprehensive error handling (8 exception types)
- Retry mechanism with exponential backoff
- Tokens resource (create, get, list, update, delete, query)
- API Keys resource (create, list, get, revoke, delete)
- Health resource (check, status)
- ESM and CJS builds
- Complete test suite with vitest
- Examples for basic usage, retry, and error handling
- Full API documentation

### Technical Details
- Built with TypeScript 5.3
- Uses axios for HTTP requests
- Tree-shakeable with tsup bundler
- Works in Node.js 18+, browsers, Deno, Bun
- Zero dependencies except axios

[0.59.2]: https://github.com/dchrnv/neurograph-os/releases/tag/v0.59.2
