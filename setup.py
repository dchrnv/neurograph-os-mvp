from setuptools import setup, find_packages

# Читаем зависимости из requirements файлов
def load_requirements(filename):
    with open(f'requirements/{filename}', 'r', encoding='utf-8') as f:
        return [line.strip() for line in f if line.strip() and not line.startswith('#')]

# Основные зависимости
install_requires = load_requirements('core.txt')

# Дополнительные зависимости
extras_require = {
    'api': load_requirements('api.txt'),
    'dev': load_requirements('dev.txt'),
    'docs': load_requirements('docs.txt'),
    'test': load_requirements('test.txt'),
    'ui': load_requirements('ui.txt'),
    'all': load_requirements('all.txt'),
}

setup(
    # Основные настройки пакета
    name="neurograph-os",
    version="0.7.0",
    
    # Поиск и включение всех пакетов в директории src
    packages=find_packages(where="src"),
    package_dir={"": "src"},
    include_package_data=True,
    
    # Зависимости для работы пакета
    install_requires=install_requires,
    
    # Дополнительные зависимости
    extras_require=extras_require,
    
    # Регистрация консольных команд
    entry_points={
        "console_scripts": [
            "neuro-reqs=infrastructure.requirements.requirements_cli:main",
            "neurograph=src.cli.main:cli",
        ],
    },
    
    # Метаданные пакета
    author="Chernov Denys",
    author_email="dreeftwood@gmail.com",
    description="Когнитивная архитектура NeuroGraph OS (dev)",
    long_description=open("README.md", encoding="utf-8").read(),
    long_description_content_type="text/markdown",
    url="https://github.com/dchrnv/neurograph-os-dev.git",
    
    # Классификаторы для PyPI
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
    
    # Минимальная версия Python
    python_requires=">=3.8",
)

    """
    NeuroGraph OS - Setup script for CLI installation.
    """

    from setuptools import setup, find_packages

    with open("README.md", "r", encoding="utf-8") as fh:
        long_description = fh.read()

    with open("requirements-cli.txt", "r", encoding="utf-8") as fh:
        cli_requirements = [line.strip() for line in fh if line.strip() and not line.startswith("#")]

    with open("requirements-persistence.txt", "r", encoding="utf-8") as fh:
        persistence_requirements = [line.strip() for line in fh if line.strip() and not line.startswith("#")]

    setup(
        name="neurograph-os",
        version="0.3.0",
        author="NeuroGraph Team",
        description="Token-based spatial computing system with clean architecture",
        long_description=long_description,
        long_description_content_type="text/markdown",
        url="https://github.com/your-org/neurograph-os",
        packages=find_packages(where="src"),
        package_dir={"": "src"},
        classifiers=[
            "Development Status :: 3 - Alpha",
            "Intended Audience :: Developers",
            "Topic :: Software Development :: Libraries :: Application Frameworks",
            "License :: OSI Approved :: MIT License",
            "Programming Language :: Python :: 3",
            "Programming Language :: Python :: 3.9",
            "Programming Language :: Python :: 3.10",
            "Programming Language :: Python :: 3.11",
            "Programming Language :: Python :: 3.12",
        ],
        python_requires=">=3.9",
        install_requires=cli_requirements + persistence_requirements,
        extras_require={
            "dev": [
                "pytest>=7.0.0",
                "pytest-asyncio>=0.21.0",
                "pytest-cov>=4.0.0",
                "black>=23.0.0",
                "flake8>=6.0.0",
                "mypy>=1.0.0",
            ],
            "docs": [
                "mkdocs>=1.5.0",
                "mkdocs-material>=9.0.0",
                "mkdocstrings[python]>=0.22.0",
            ],
        },
        entry_points={
            "console_scripts": [
                "neurograph=cli.main:main",
            ],
        },
        include_package_data=True,
        zip_safe=False,
    )