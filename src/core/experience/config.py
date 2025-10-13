import os
from typing import Any, Dict, Optional

def load_experience_config(path: Optional[str] = None) -> Dict[str, Any]:
    """Load experience configuration from YAML or JSON file. Returns dict with defaults if not found."""
    cfg = {}
    if path is None:
        # default path inside project
        path = os.path.join(os.getcwd(), 'config', 'application', 'experience.yaml')

    if not os.path.exists(path):
        return cfg

    # try to load YAML if available, otherwise JSON
    try:
        import yaml
    except Exception:
        yaml = None

    try:
        if yaml is not None:
            with open(path, 'r', encoding='utf-8') as f:
                return yaml.safe_load(f) or {}
        else:
            # fallback to json if file extension is .json
            import json
            with open(path, 'r', encoding='utf-8') as f:
                return json.load(f)
    except Exception:
        return cfg
