# Jupyter Integration Specification

**Версия:** 1.0.0  
**Дата:** 2025-01-XX  
**Статус:** Спецификация  
**Зависимости:** neurograph, IPython, matplotlib, networkx

---

## Обзор

Интеграция с Jupyter Notebook для интерактивного исследования и визуализации семантических графов. Включает magic commands, rich display и визуализации.

---

## Быстрый старт

```python
# Cell 1: Setup
%load_ext neurograph

import neurograph as ng
runtime = ng.Runtime()
runtime.bootstrap("embeddings.txt", limit=10000)

# Cell 2: Query
result = runtime.query("кошка")
result  # Rich display автоматически
```

---

## Magic Commands

### Line Magics

```python
# Статус системы
%ng_status

# Быстрый запрос
%ng_query кошка

# Информация о токене
%ng_token 4523

# Статистика
%ng_stats
```

### Cell Magics

```python
%%ng_explore
# Интерактивное исследование
start: кошка
depth: 2
show_connections: true

%%ng_batch
# Пакетные запросы
кошка
собака
птица
```

---

## Rich Display

### QueryResult

```python
result = runtime.query("программирование")
result  # Автоматический rich display
```

Output:
```
╭─────────────────────────────────────────────────────────────╮
│  Query: "программирование"                                  │
│  Signal ID: 12847 | Time: 14.2ms | Confidence: 0.87        │
├─────────────────────────────────────────────────────────────┤
│  Results (10):                                              │
│                                                             │
│  1. программист     0.89  ████████▉   Concept              │
│  2. код             0.84  ████████▍   Concept              │
│  3. разработка      0.81  ████████    Process              │
│  4. алгоритм        0.78  ███████▊    Concept              │
│  5. компьютер       0.71  ███████     Object               │
│                                                             │
│  Connections: 23 | Clusters: 3                             │
╰─────────────────────────────────────────────────────────────╯
```

### Token Display

```python
token = runtime.get_token(4523)
token  # Rich display
```

Output:
```
╭─────────────────────────────────────────────────────────────╮
│  Token #4523: "кот"                                        │
├─────────────────────────────────────────────────────────────┤
│  Type: Concept | Weight: 1.5 | Active: ✓                   │
│                                                             │
│  Coordinates:                                               │
│  L1 Physical:  (0.12, 0.34, 0.56)                          │
│  L4 Emotional: (0.78, 0.56, 0.67)  [positive, calm]        │
│  L8 Abstract:  (0.78, 0.90, 0.12)                          │
│                                                             │
│  Connections: 47 (12 incoming, 35 outgoing)                │
╰─────────────────────────────────────────────────────────────╯
```

---

## Визуализации

### Graph View

```python
result = runtime.query("животные", limit=20)
result.visualize(style="graph")
```

```
        ┌─────────┐
        │ животное│
        └────┬────┘
             │
    ┌────────┼────────┐
    │        │        │
┌───▼──┐ ┌───▼──┐ ┌───▼──┐
│ кошка│ │собака│ │ птица│
└───┬──┘ └───┬──┘ └───┬──┘
    │        │        │
┌───▼──┐ ┌───▼──┐ ┌───▼──┐
│  кот │ │щенок │ │воробей│
└──────┘ └──────┘ └──────┘
```

### Heatmap

```python
result.visualize(style="heatmap", space="L4Emotional")
```

Показывает распределение токенов в эмоциональном пространстве (VAD).

### Timeline

```python
runtime.visualize_activity(minutes=60)
```

График активности системы за последний час.

### Coordinate Space

```python
result.visualize(style="3d", space="L8Abstract")
```

3D scatter plot токенов в абстрактном пространстве.

---

## API

### visualize()

```python
def visualize(
    self,
    style: Literal["graph", "tree", "table", "heatmap", "3d"] = "graph",
    space: Optional[str] = None,
    max_nodes: int = 50,
    show_labels: bool = True,
    show_weights: bool = True,
    figsize: Tuple[int, int] = (12, 8),
    **kwargs
) -> None:
    """
    Визуализация результатов запроса.
    
    Args:
        style: Стиль визуализации
        space: Координатное пространство (для heatmap, 3d)
        max_nodes: Максимум узлов
        show_labels: Показывать метки
        show_weights: Показывать веса связей
        figsize: Размер фигуры
    """
```

### to_dataframe()

```python
df = result.to_dataframe()
df.head()
```

| | label | score | entity_type | x | y | z |
|---|-------|-------|-------------|---|---|---|
| 0 | кот | 0.92 | Concept | 0.12 | 0.34 | 0.56 |
| 1 | котёнок | 0.87 | Concept | 0.15 | 0.32 | 0.58 |

### to_networkx()

```python
G = result.to_networkx()
# NetworkX graph для дальнейшего анализа
nx.degree_centrality(G)
```

---

## Примеры

### Исследование концепта

```python
# Cell 1
%load_ext neurograph
import neurograph as ng

runtime = ng.Runtime()
runtime.bootstrap("glove.6B.100d.txt", limit=50000)

# Cell 2
result = runtime.query("любовь", limit=20)
result.visualize(style="graph")

# Cell 3
# Эмоциональное пространство
result.visualize(style="heatmap", space="L4Emotional")

# Cell 4
# Анализ через pandas
df = result.to_dataframe()
df.groupby('entity_type').agg({'score': ['mean', 'count']})
```

### Сравнение концептов

```python
# Cell 1
concepts = ["радость", "грусть", "страх", "гнев"]
results = [runtime.query(c, limit=10) for c in concepts]

# Cell 2
import matplotlib.pyplot as plt

fig, axes = plt.subplots(2, 2, figsize=(12, 10))
for ax, result, concept in zip(axes.flat, results, concepts):
    df = result.to_dataframe()
    ax.barh(df['label'][:5], df['score'][:5])
    ax.set_title(concept)
plt.tight_layout()
```

### Кластерный анализ

```python
# Cell 1
from sklearn.cluster import KMeans
import numpy as np

result = runtime.query("наука", limit=100)
df = result.to_dataframe()

# Координаты L8Abstract
coords = np.array([[r.x, r.y, r.z] for r in df.itertuples()])

# Cell 2
kmeans = KMeans(n_clusters=5)
df['cluster'] = kmeans.fit_predict(coords)

# Cell 3
for cluster_id in range(5):
    cluster_df = df[df['cluster'] == cluster_id]
    print(f"\nCluster {cluster_id}:")
    print(cluster_df['label'].head(5).tolist())
```

---

## Настройки отображения

```python
# Конфигурация display
ng.display.max_results = 20
ng.display.show_coordinates = True
ng.display.color_scheme = "dark"  # or "light"
ng.display.figure_size = (14, 10)

# Или через magic
%ng_config max_results=20 color_scheme=dark
```

---

## Установка

```bash
pip install neurograph[jupyter]

# Включает:
# - ipython
# - matplotlib
# - networkx
# - pandas (optional)
```

---

## Реализация

### Extension loader

```python
# neurograph/integrations/jupyter.py

def load_ipython_extension(ipython):
    """Загрузка расширения."""
    from .magic import NeurographMagics
    from .display import register_formatters
    
    ipython.register_magics(NeurographMagics)
    register_formatters()
    
    print("✓ neurograph extension loaded")
```

### Magic commands

```python
# neurograph/integrations/magic.py

from IPython.core.magic import Magics, magics_class, line_magic, cell_magic

@magics_class
class NeurographMagics(Magics):
    
    @line_magic
    def ng_status(self, line):
        """Show system status."""
        runtime = self._get_runtime()
        status = runtime.status()
        display(status)
    
    @line_magic
    def ng_query(self, line):
        """Quick query: %ng_query кошка"""
        runtime = self._get_runtime()
        result = runtime.query(line.strip())
        display(result)
    
    @cell_magic
    def ng_batch(self, line, cell):
        """Batch queries."""
        runtime = self._get_runtime()
        queries = [q.strip() for q in cell.split('\n') if q.strip()]
        results = [runtime.query(q) for q in queries]
        for r in results:
            display(r)
    
    def _get_runtime(self):
        """Get runtime from user namespace."""
        ns = self.shell.user_ns
        if 'runtime' in ns:
            return ns['runtime']
        raise RuntimeError("No 'runtime' found. Create one first.")
```

### Rich formatters

```python
# neurograph/integrations/display.py

from IPython.display import display, HTML

def register_formatters():
    """Register HTML formatters for neurograph types."""
    ip = get_ipython()
    
    formatter = ip.display_formatter.formatters['text/html']
    formatter.for_type(QueryResult, format_query_result)
    formatter.for_type(Token, format_token)
    formatter.for_type(RuntimeStatus, format_status)

def format_query_result(result: QueryResult) -> str:
    """Format QueryResult as HTML."""
    html = f"""
    <div style="font-family: monospace; background: #1a1a1a; color: #e0e0e0; 
                padding: 16px; border-radius: 8px; margin: 8px 0;">
        <div style="color: #00ffcc; font-size: 14px; margin-bottom: 8px;">
            Query: "{result.query_text}"
        </div>
        <div style="color: #888; font-size: 12px; margin-bottom: 12px;">
            Signal: {result.signal_id} | Time: {result.processing_time_ms:.1f}ms | 
            Confidence: {result.confidence:.2f}
        </div>
        <table style="width: 100%; border-collapse: collapse;">
            <tr style="color: #888; font-size: 11px;">
                <th style="text-align: left; padding: 4px;">Label</th>
                <th style="text-align: left; padding: 4px;">Score</th>
                <th style="text-align: left; padding: 4px;">Type</th>
            </tr>
    """
    
    for t in result.tokens[:10]:
        bar_width = int(t.score * 100)
        html += f"""
            <tr>
                <td style="padding: 4px; color: #fff;">{t.label}</td>
                <td style="padding: 4px;">
                    <div style="background: #00ffcc; width: {bar_width}px; 
                                height: 12px; border-radius: 2px;"></div>
                    <span style="color: #888; font-size: 11px;">{t.score:.2f}</span>
                </td>
                <td style="padding: 4px; color: #888;">{t.entity_type}</td>
            </tr>
        """
    
    html += "</table></div>"
    return html
```

---

## Следующий документ

→ **TELEGRAM_BOT_SPEC.md** (опционально, по запросу)

---

**Jupyter Integration Specification v1.0.0**  
*Интерактивное исследование Tiro*
