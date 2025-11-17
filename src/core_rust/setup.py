
    # NeuroGraph - Высокопроизводительная система пространственных вычислений на основе токенов.
    # Copyright (C) 2024-2025 Chernov Denys

    # This program is free software: you can redistribute it and/or modify
    # it under the terms of the GNU Affero General Public License as published by
    # the Free Software Foundation, either version 3 of the License, or
    # (at your option) any later version.

    # This program is distributed in the hope that it will be useful,
    # but WITHOUT ANY WARRANTY; without even the implied warranty of
    # MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    # GNU Affero General Public License for more details.

    # You should have received a copy of the GNU Affero General Public License
    # along with this program. If not, see <https://www.gnu.org/licenses/>.
    

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
