// Core bridge - прямое использование neurograph-core
// Обёртка для удобного использования из UI

use neurograph_core::{Token, Grid, Graph, Guardian, CDNA};
use std::sync::{Arc, Mutex};

/// Backend состояние - прямой доступ к Rust core
pub struct CoreBridge {
    grid: Arc<Mutex<Grid>>,
    graph: Arc<Mutex<Graph>>,
    guardian: Arc<Mutex<Guardian>>,
}

impl CoreBridge {
    pub fn new() -> Self {
        let guardian = Guardian::new();

        Self {
            grid: Arc::new(Mutex::new(Grid::new())),
            graph: Arc::new(Mutex::new(Graph::new())),
            guardian: Arc::new(Mutex::new(guardian)),
        }
    }

    /// Обработка команды из чата
    pub fn process_message(&self, msg: &str) -> String {
        let msg_lower = msg.to_lowercase();

        // Команды с параметрами
        if msg_lower.starts_with("create ") {
            let count = msg_lower
                .strip_prefix("create ")
                .and_then(|s| s.trim().parse::<usize>().ok())
                .unwrap_or(1);
            return self.create_tokens(count);
        }

        // Простые команды
        match msg_lower.as_str() {
            "статус" | "status" => self.get_status(),
            "создать токен" | "create token" | "create" => self.create_token(),
            "очистить" | "clear" => self.clear_all(),
            "список" | "list" => self.list_tokens(),
            "граф" | "graph" => self.graph_info(),
            "help" | "помощь" | "?" => self.help(),
            _ => format!("Команда не распознана: '{}'\nВведи 'help' для списка команд", msg),
        }
    }

    fn get_status(&self) -> String {
        let grid = self.grid.lock().unwrap();
        let graph = self.graph.lock().unwrap();

        format!(
            "System Status:\n\
             - Tokens in grid: {}\n\
             - Graph nodes: {}\n\
             - Graph edges: {}\n\
             - Mode: Running",
            grid.len(),
            graph.node_count(),
            graph.edge_count()
        )
    }

    fn create_token(&self) -> String {
        let mut grid = self.grid.lock().unwrap();

        // Создаём токен через core_rust
        let token_id = (grid.len() + 1) as u32;
        let token = Token::new(token_id);

        // Валидация через Guardian
        if let Err(e) = self.guardian.lock().unwrap().validate_token(&token) {
            return format!("Ошибка валидации: {:?}", e);
        }

        // Добавляем в Grid
        match grid.add(token) {
            Ok(_) => format!("[OK] Токен создан (ID: {})", token_id),
            Err(e) => format!("Ошибка: {:?}", e),
        }
    }

    fn create_tokens(&self, count: usize) -> String {
        if count == 0 || count > 100 {
            return "Количество токенов должно быть от 1 до 100".to_string();
        }

        let mut grid = self.grid.lock().unwrap();
        let mut created = 0;

        for _ in 0..count {
            let token_id = (grid.len() + 1) as u32;
            let token = Token::new(token_id);

            if self.guardian.lock().unwrap().validate_token(&token).is_ok() {
                if grid.add(token).is_ok() {
                    created += 1;
                }
            }
        }

        format!("[OK] Создано токенов: {}/{}", created, count)
    }

    fn clear_all(&self) -> String {
        let mut grid = self.grid.lock().unwrap();
        let mut graph = self.graph.lock().unwrap();

        let token_count = grid.len();
        let edge_count = graph.edge_count();

        // Очищаем grid (remove_all не существует, но можем пересоздать)
        *grid = Grid::new();
        *graph = Graph::new();

        format!(
            "[OK] Очищено:\n  - Токенов: {}\n  - Связей: {}",
            token_count, edge_count
        )
    }

    fn list_tokens(&self) -> String {
        let grid = self.grid.lock().unwrap();

        if grid.is_empty() {
            return "Нет токенов в системе.\nСоздай токен командой 'create'".to_string();
        }

        let count = grid.len();
        format!(
            "Токенов в системе: {}\n\nИспользуй 'status' для детальной информации",
            count
        )
    }

    fn graph_info(&self) -> String {
        let graph = self.graph.lock().unwrap();

        format!(
            "Информация о графе:\n\
             - Узлов: {}\n\
             - Связей: {}\n\
             - Средняя степень: {:.2}",
            graph.node_count(),
            graph.edge_count(),
            if graph.node_count() > 0 {
                graph.edge_count() as f32 / graph.node_count() as f32
            } else {
                0.0
            }
        )
    }

    fn help(&self) -> String {
        "Доступные команды:\n\n\
         Основные:\n\
         - status / статус         - статус системы\n\
         - help / помощь / ?       - эта справка\n\n\
         Токены:\n\
         - create / создать токен  - создать 1 токен\n\
         - create N                - создать N токенов (max 100)\n\
         - list / список           - количество токенов\n\n\
         Граф:\n\
         - graph / граф            - информация о графе\n\n\
         Система:\n\
         - clear / очистить        - удалить все токены и связи"
            .to_string()
    }
}

impl Default for CoreBridge {
    fn default() -> Self {
        Self::new()
    }
}
