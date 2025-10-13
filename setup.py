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
    name="model-requirements",
    version="0.1.0",
    
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
            "model-reqs=model_requirements:main",
        ],
    },
    
    # Метаданные пакета
    author="Chernov Denys",
    author_email="dreeftwood@gmail.com",
    description="Управление зависимостями для проекта Model",
    long_description=open("README.md", encoding="utf-8").read(),
    long_description_content_type="text/markdown",
    url="https://github.com/dchrnv/neurograph-os.git",
    
    # Классификаторы для PyPI
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
    
    # Минимальная версия Python
    python_requires=">=3.8",
)
