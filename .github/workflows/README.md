# GitHub Actions Workflows

Automated CI/CD pipelines for NeuroGraph project.

## Workflows

### Python Client CI (`python-client-ci.yml`)

**Triggers:**
- Push to `main` or `develop` branches (if python-client/** changed)
- Pull requests to `main` or `develop` (if python-client/** changed)

**Jobs:**
1. **Test** - Run on Python 3.10, 3.11, 3.12
   - Lint with ruff
   - Format check with black
   - Type check with mypy
   - Run tests with pytest
   - Upload coverage to Codecov

2. **Build** - Build Python package
   - Build wheel and sdist
   - Check package with twine
   - Upload build artifacts

### TypeScript Client CI (`typescript-client-ci.yml`)

**Triggers:**
- Push to `main` or `develop` branches (if typescript-client/** changed)
- Pull requests to `main` or `develop` (if typescript-client/** changed)

**Jobs:**
1. **Test** - Run on Node.js 18, 20
   - Lint with ESLint
   - Format check with Prettier
   - Type check with TypeScript
   - Run tests with Vitest
   - Upload coverage to Codecov

2. **Build** - Build TypeScript package
   - Build ESM and CJS bundles
   - Upload build artifacts

### Publish Python Client (`publish-python.yml`)

**Triggers:**
- Release published with tag `python-*`
- Manual workflow dispatch

**Jobs:**
1. **Publish**
   - Build package
   - Publish to Test PyPI (manual dispatch)
   - Publish to PyPI (release)
   - Create GitHub release (manual dispatch)

**Required Secrets:**
- `PYPI_API_TOKEN` - PyPI API token
- `TEST_PYPI_API_TOKEN` - Test PyPI API token

### Publish TypeScript Client (`publish-typescript.yml`)

**Triggers:**
- Release published with tag `typescript-*`
- Manual workflow dispatch

**Jobs:**
1. **Publish**
   - Build package
   - Run tests
   - Publish to npm
   - Create GitHub release (manual dispatch)

**Required Secrets:**
- `NPM_TOKEN` - npm access token

## Setup Instructions

### 1. Configure Secrets

Go to **Settings → Secrets and variables → Actions** and add:

**For Python:**
```
PYPI_API_TOKEN=pypi-...
TEST_PYPI_API_TOKEN=pypi-...
```

**For TypeScript:**
```
NPM_TOKEN=npm_...
```

### 2. PyPI Setup

1. Create account on https://pypi.org
2. Create account on https://test.pypi.org
3. Generate API tokens:
   - PyPI: Account settings → API tokens
   - Test PyPI: Account settings → API tokens
4. Add tokens to GitHub secrets

### 3. npm Setup

1. Create account on https://www.npmjs.com
2. Generate access token:
   - Profile → Access Tokens → Generate New Token
   - Type: Automation
3. Add token to GitHub secrets

### 4. Codecov Setup (Optional)

1. Go to https://codecov.io
2. Connect GitHub repository
3. Codecov will automatically receive coverage reports

## Manual Release Process

### Python Client

1. **Update version** in `python-client/pyproject.toml`

2. **Update CHANGELOG** in `python-client/CHANGELOG.md`

3. **Commit changes:**
   ```bash
   git add python-client/
   git commit -m "chore(python-client): bump version to 0.59.3"
   ```

4. **Create tag:**
   ```bash
   git tag python-0.59.3
   git push origin python-0.59.3
   ```

5. **Create GitHub Release:**
   - Go to Releases → Create new release
   - Tag: `python-0.59.3`
   - Title: `Python Client v0.59.3`
   - Description: Copy from CHANGELOG
   - Publish release

6. **Workflow runs automatically**

### TypeScript Client

1. **Update version** in `typescript-client/package.json`

2. **Update CHANGELOG** in `typescript-client/CHANGELOG.md`

3. **Commit changes:**
   ```bash
   git add typescript-client/
   git commit -m "chore(typescript-client): bump version to 0.59.3"
   ```

4. **Create tag:**
   ```bash
   git tag typescript-0.59.3
   git push origin typescript-0.59.3
   ```

5. **Create GitHub Release:**
   - Go to Releases → Create new release
   - Tag: `typescript-0.59.3`
   - Title: `TypeScript Client v0.59.3`
   - Description: Copy from CHANGELOG
   - Publish release

6. **Workflow runs automatically**

## Testing Releases

### Test PyPI (Python)

Use manual workflow dispatch:

1. Go to Actions → Publish Python Client
2. Click "Run workflow"
3. Enter version (e.g., `0.59.3`)
4. Check Test PyPI: https://test.pypi.org/project/neurograph-python/

Install from Test PyPI:
```bash
pip install --index-url https://test.pypi.org/simple/ neurograph-python==0.59.3
```

### npm Dry Run (TypeScript)

Use manual workflow dispatch to test without publishing:

1. Go to Actions → Publish TypeScript Client
2. Click "Run workflow"
3. Enter version
4. Check logs for dry-run output

## Troubleshooting

### Build Fails

- Check linter errors in CI logs
- Run tests locally: `pytest` or `npm test`
- Verify Python/Node.js versions match CI matrix

### Publish Fails

- Verify secrets are configured correctly
- Check token permissions
- Ensure version number is not already published
- Check package name availability

### Coverage Upload Fails

- Verify Codecov integration is enabled
- Check coverage files are generated
- Ensure paths in workflow match actual coverage files

## Best Practices

1. **Always test locally before pushing:**
   ```bash
   # Python
   cd python-client
   ruff check .
   black --check .
   mypy neurograph/
   pytest

   # TypeScript
   cd typescript-client
   npm run lint
   npm run format:check
   npm run typecheck
   npm test
   ```

2. **Use semantic versioning:**
   - MAJOR.MINOR.PATCH (e.g., 1.2.3)
   - Increment PATCH for bug fixes
   - Increment MINOR for new features
   - Increment MAJOR for breaking changes

3. **Update CHANGELOG:**
   - Document all changes
   - Follow Keep a Changelog format
   - Include migration guides for breaking changes

4. **Test in staging:**
   - Use Test PyPI for Python
   - Use npm dry-run for TypeScript
   - Test installation and usage

5. **Create meaningful releases:**
   - Clear release notes
   - Link to issues/PRs
   - Include upgrade instructions

## Monitoring

### Check Workflow Status

- Go to Actions tab
- View workflow runs
- Check logs for failures

### Package Download Stats

- PyPI: https://pypistats.org/packages/neurograph-python
- npm: https://www.npmjs.com/package/@neurograph/client

### Coverage Reports

- Codecov: https://codecov.io/gh/YOUR_USERNAME/neurograph-os
