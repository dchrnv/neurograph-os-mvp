"""
NeuroGraph OS - Token-based spatial computing system
Setup script for installation
"""

from setuptools import setup, find_packages
from pathlib import Path

# Read README
readme_file = Path(__file__).parent / "README.md"
long_description = readme_file.read_text(encoding="utf-8") if readme_file.exists() else ""

# Read requirements from requirements.txt
requirements_file = Path(__file__).parent / "requirements.txt"
if requirements_file.exists():
    with open(requirements_file, 'r', encoding='utf-8') as f:
        install_requires = [
            line.strip()
            for line in f
            if line.strip() and not line.startswith('#')
        ]
else:
    install_requires = []

setup(
    # Package metadata
    name="neurograph-os",
    version="0.10.0",  # v0.10 - HTTP API реализован
    author="Chernov Denys",
    author_email="dreeftwood@gmail.com",
    description="Когнитивная архитектура с токен-ориентированными вычислениями и пространственным интеллектом",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/dchrnv/neurograph-os-dev",
    license="MIT",

    # Package structure
    packages=find_packages(where="src"),
    package_dir={"": "src"},
    include_package_data=True,
    zip_safe=False,

    # Dependencies
    install_requires=install_requires,

    # Python version
    python_requires=">=3.10",

    # Optional dependencies
    extras_require={
        "dev": [
            "pytest>=7.0.0",
            "pytest-asyncio>=0.21.0",
            "pytest-cov>=4.0.0",
            "black>=23.0.0",
            "isort>=5.10.1",
            "flake8>=6.0.0",
            "mypy>=1.0.0",
        ],
        "docs": [
            "mkdocs>=1.5.0",
            "mkdocs-material>=9.0.0",
            "mkdocstrings[python]>=0.22.0",
        ],
    },

    # CLI commands
    entry_points={
        "console_scripts": [
            "neurograph=cli.main:cli",
        ],
    },

    # PyPI classifiers
    classifiers=[
        "Development Status :: 4 - Beta",
        "Intended Audience :: Developers",
        "Intended Audience :: Science/Research",
        "Topic :: Software Development :: Libraries :: Application Frameworks",
        "Topic :: Scientific/Engineering :: Artificial Intelligence",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
        "Operating System :: OS Independent",
    ],
)
