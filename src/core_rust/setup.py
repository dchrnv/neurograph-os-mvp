"""
Setup script for NeuroGraph Core Python bindings

This uses maturin (or setuptools-rust) to build the Rust extension.

Installation:
    pip install maturin
    maturin develop --release --features python

Or for production:
    maturin build --release --features python
    pip install target/wheels/*.whl
"""

from setuptools import setup

setup(
    name="neurograph-core",
    version="0.14.0",
    author="NeuroGraph OS Team",
    description="Rust-powered core for NeuroGraph OS with Python bindings",
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    url="https://github.com/dchrnv/neurograph-os-mvp",
    packages=["neurograph"],
    package_dir={"neurograph": "python"},
    classifiers=[
        "Development Status :: 4 - Beta",
        "Intended Audience :: Developers",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Rust",
    ],
    python_requires=">=3.8",
    install_requires=[],
    zip_safe=False,
)
