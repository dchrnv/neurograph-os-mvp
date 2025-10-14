# NeuroGraph OS 🧠

Когнитивная операционная система с пространственным интеллектом и нейрографическими вычислениями.

## 🚀 Быстрый старт

### Предварительные требования

- Python 3.11+
- Redis (для кэширования)
- PostgreSQL (опционально, для продакшена)

### Установка

```bash
# Клонирование репозитория
git clone https://github.com/dchrnv/neurograph-os.git
cd neurograph-os

# Создание виртуального окружения
python -m venv .venv
source .venv/bin/activate  # Linux/Mac
# или
.venv\Scripts\activate     # Windows

# Установка зависимостей
pip install -r requirements/core.txt
pip install -r requirements/dev.txt  # для разработки
