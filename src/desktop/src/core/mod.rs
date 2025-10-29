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

        match msg_lower.as_str() {
            "статус" | "status" => self.get_status(),
            "создать токен" | "create token" => self.create_token(),
            "help" | "помощь" => self.help(),
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
            Ok(_) => format!("✓ Токен создан (ID: {})", token_id),
            Err(e) => format!("Ошибка: {:?}", e),
        }
    }

    fn help(&self) -> String {
        "Доступные команды:\n\
         - status / статус - показать статус системы\n\
         - create token / создать токен - создать новый токен\n\
         - help / помощь - эта справка"
            .to_string()
    }
}

impl Default for CoreBridge {
    fn default() -> Self {
        Self::new()
    }
}
