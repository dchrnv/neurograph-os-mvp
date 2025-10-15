"""
NeuroGraph OS - Event System Integration Examples
Примеры интеграции Event Bus с существующими компонентами

Путь: examples/event_integration_example.py
"""

import asyncio
from typing import Dict, Any, List
from datetime import datetime

from core.events import (
    Event, EventType, EventCategory, EventPriority, EventFilter,
    EventBus, EventHandler, EventEmitter, event_publisher,
    start_event_bus, stop_event_bus, get_event_bus
)


print("""
╔════════════════════════════════════════════════════════════════════╗
║        NeuroGraph OS - Event System Integration Example            ║
║                 Full Component Integration Demo                     ║
╚════════════════════════════════════════════════════════════════════╝
""")


# =============================================================================
# Компонент 1: TokenService с событиями
# =============================================================================

class TokenService(EventEmitter):
    """Сервис управления токенами с поддержкой событий"""
    
    def __init__(self, event_bus: EventBus):
        super().__init__(event_bus, source_id="token_service")
        self.tokens = {}
        print("✅ TokenService initialized")
    
    @event_publisher(EventType.TOKEN_CREATED)
    async def create_token(self, token_id: str, coordinates: list, data: dict) -> dict:
        """Создать токен и автоматически опубликовать событие"""
        token = {
            "id": token_id,
            "coordinates": coordinates,
            "data": data,
            "created_at": datetime.now().timestamp()
        }
        self.tokens[token_id] = token
        
        print(f"🎯 TokenService: Created token {token_id}")
        
        # Возвращаемый словарь станет payload события
        return {
            "token_id": token_id,
            "coordinates": coordinates,
            "token_type": data.get("type", "unknown")
        }
    
    async def update_token(self, token_id: str, updates: dict) -> None:
        """Обновить токен"""
        if token_id not in self.tokens:
            await self.emit_error(
                error_type="token_not_found",
                error_message=f"Token {token_id} not found",
                error_details={"token_id": token_id}
            )
            return
        
        self.tokens[token_id].update(updates)
        
        print(f"🔄 TokenService: Updated token {token_id}")
        
        await self.emit(
            EventType.TOKEN_UPDATED,
            payload={
                "token_id": token_id,
                "updates": updates
            },
            priority=EventPriority.NORMAL
        )
    
    async def activate_token(self, token_id: str, activation_level: float) -> None:
        """Активировать токен (высокоприоритетное событие)"""
        if token_id not in self.tokens:
            return
        
        self.tokens[token_id]["activation"] = activation_level
        
        priority = EventPriority.HIGH if activation_level > 0.8 else EventPriority.NORMAL
        
        print(f"⚡ TokenService: Activated token {token_id} (level={activation_level:.2f}, priority={priority.name})")
        
        await self.emit(
            EventType.TOKEN_ACTIVATED,
            payload={
                "token_id": token_id,
                "activation_level": activation_level
            },
            priority=priority
        )


# =============================================================================
# Компонент 2: GraphManager с событиями
# =============================================================================

class GraphManager(EventEmitter):
    """Менеджер графа с поддержкой событий"""
    
    def __init__(self, event_bus: EventBus):
        super().__init__(event_bus, source_id="graph_manager")
        self.connections = []
        self.tokens = set()
        print("✅ GraphManager initialized")
    
    async def add_connection(
        self,
        source_id: str,
        target_id: str,
        weight: float = 1.0,
        connection_type: str = "spatial"
    ) -> None:
        """Добавить связь в граф"""
        connection = {
            "source": source_id,
            "target": target_id,
            "weight": weight,
            "type": connection_type,
            "created_at": datetime.now().timestamp()
        }
        self.connections.append(connection)
        self.tokens.add(source_id)
        self.tokens.add(target_id)
        
        print(f"🔗 GraphManager: Added connection {source_id} -> {target_id} (weight={weight})")
        
        await self.emit(
            EventType.GRAPH_CONNECTION_ADDED,
            payload={
                "connection": connection,
                "total_connections": len(self.connections)
            }
        )
        
        # Проверка на кластер (каждые 5 связей)
        if len(self.connections) % 5 == 0 and len(self.connections) > 0:
            await self._check_for_clusters()
    
    async def _check_for_clusters(self):
        """Проверить наличие кластеров"""
        if len(self.tokens) >= 3:
            print(f"🎯 GraphManager: Detected cluster with {len(self.tokens)} tokens")
            
            await self.emit(
                EventType.GRAPH_CLUSTER_DETECTED,
                payload={
                    "cluster_size": len(self.tokens),
                    "connections_count": len(self.connections),
                    "density": len(self.connections) / max(1, len(self.tokens))
                },
                priority=EventPriority.HIGH if len(self.tokens) > 5 else EventPriority.NORMAL
            )


# =============================================================================
# Компонент 3: DNAGuardian с валидацией и событиями
# =============================================================================

class DNAGuardian(EventEmitter):
    """DNA Guardian с валидацией и событиями"""
    
    def __init__(self, event_bus: EventBus):
        super().__init__(event_bus, source_id="dna_guardian")
        self.constraints = {
            "max_tokens": 1000,
            "max_connections_per_token": 10,
            "min_activation_level": 0.1
        }
        self.violations_count = 0
        print("✅ DNAGuardian initialized")
    
    @EventHandler.on(
        EventType.TOKEN_CREATED,
        subscription_name="dna_token_validator"
    )
    async def validate_token_creation(self, event: Event):
        """Валидация создания токена по DNA правилам"""
        token_id = event.payload.get("token_id")
        
        # Простая валидация (в реальности здесь сложная логика)
        is_valid = True
        
        if is_valid:
            print(f"✅ DNAGuardian: Token {token_id} validated")
        else:
            print(f"❌ DNAGuardian: Token {token_id} violates constraints")
            await self.emit(
                EventType.DNA_CONSTRAINT_VIOLATED,
                payload={
                    "token_id": token_id,
                    "violation_type": "invalid_structure"
                },
                priority=EventPriority.CRITICAL
            )
    
    @EventHandler.on(
        EventType.GRAPH_CONNECTION_ADDED,
        subscription_name="dna_connection_validator"
    )
    async def validate_connection(self, event: Event):
        """Валидация связи по DNA правилам"""
        connection = event.payload.get("connection")
        print(f"✅ DNAGuardian: Connection {connection['source']} -> {connection['target']} validated")
    
    async def mutate_dna(self, mutation_type: str, changes: dict):
        """Изменение DNA"""
        old_constraints = self.constraints.copy()
        self.constraints.update(changes)
        
        print(f"🧬 DNAGuardian: DNA mutated ({mutation_type})")
        print(f"   Changes: {changes}")
        
        await self.emit(
            EventType.DNA_MUTATED,
            payload={
                "mutation_type": mutation_type,
                "changes": changes,
                "old_constraints": old_constraints,
                "new_constraints": self.constraints
            },
            priority=EventPriority.HIGH
        )


# =============================================================================
# Компонент 4: AutoGraphBuilder - Автоматические реакции на события
# =============================================================================

class AutoGraphBuilder:
    """Автоматически создает связи при создании токенов"""
    
    def __init__(self, graph_manager: GraphManager):
        self.graph_manager = graph_manager
        self.last_token_id = None
        print("✅ AutoGraphBuilder initialized")
    
    @EventHandler.on(
        EventType.TOKEN_CREATED,
        subscription_name="auto_link_tokens"
    )
    async def auto_link_new_token(self, event: Event):
        """При создании нового токена, связать его с предыдущим"""
        new_token_id = event.payload.get("token_id")
        
        if self.last_token_id and self.last_token_id != new_token_id:
            # Создаем связь между последним и новым токеном
            await self.graph_manager.add_connection(
                self.last_token_id,
                new_token_id,
                weight=0.8,
                connection_type="auto"
            )
            
            print(f"🤖 AutoGraphBuilder: Auto-linked {self.last_token_id} -> {new_token_id}")
        
        self.last_token_id = new_token_id


# =============================================================================
# Компонент 5: EventLogger - Централизованное логирование
# =============================================================================

class EventLogger:
    """Логирует все важные события"""
    
    def __init__(self):
        self.event_log = []
        print("✅ EventLogger initialized")
    
    @EventHandler.on(
        min_priority=EventPriority.HIGH,
        subscription_name="high_priority_logger"
    )
    async def log_important_events(self, event: Event):
        """Логировать важные события"""
        self.event_log.append(event)
        print(f"📝 EventLogger: Logged {event.type.value} (priority={event.priority.name})")
    
    @EventHandler.on(
        categories=[EventCategory.ERROR],
        subscription_name="error_logger"
    )
    async def log_errors(self, event: Event):
        """Централизованное логирование ошибок"""
        error_type = event.payload.get("error_type")
        error_msg = event.payload.get("error_message")
        print(f"❌ EventLogger: ERROR [{error_type}] {error_msg}")


# =============================================================================
# Компонент 6: StatisticsCollector - Сбор статистики
# =============================================================================

class StatisticsCollector:
    """Собирает статистику о событиях"""
    
    def __init__(self):
        self.stats = {
            "tokens_created": 0,
            "tokens_activated": 0,
            "connections_added": 0,
            "clusters_detected": 0,
            "dna_mutations": 0,
            "errors": 0
        }
        print("✅ StatisticsCollector initialized")
    
    @EventHandler.on(EventType.TOKEN_CREATED)
    async def count_tokens(self, event: Event):
        self.stats["tokens_created"] += 1
    
    @EventHandler.on(EventType.TOKEN_ACTIVATED)
    async def count_activations(self, event: Event):
        self.stats["tokens_activated"] += 1
    
    @EventHandler.on(EventType.GRAPH_CONNECTION_ADDED)
    async def count_connections(self, event: Event):
        self.stats["connections_added"] += 1
    
    @EventHandler.on(EventType.GRAPH_CLUSTER_DETECTED)
    async def count_clusters(self, event: Event):
        self.stats["clusters_detected"] += 1
    
    @EventHandler.on(EventType.DNA_MUTATED)
    async def count_mutations(self, event: Event):
        self.stats["dna_mutations"] += 1
    
    @EventHandler.on(categories=[EventCategory.ERROR])
    async def count_errors(self, event: Event):
        self.stats["errors"] += 1
    
    def print_stats(self):
        """Вывести статистику"""
        print("\n" + "="*70)
        print("📊 Statistics Summary")
        print("="*70)
        for key, value in self.stats.items():
            print(f"   {key.replace('_', ' ').title()}: {value}")
        print("="*70 + "\n")


# =============================================================================
# Главная демонстрация
# =============================================================================

async def main():
    """Главная функция демонстрации"""
    
    print("="*70)
    print("Starting NeuroGraph OS Event System Integration Demo")
    print("="*70 + "\n")
    
    # 1. Запускаем Event Bus
    print("🚀 Step 1: Starting Event Bus...")
    event_bus = await start_event_bus(max_queue_size=1000, enable_metrics=True)
    print("✅ Event Bus started\n")
    
    # 2. Устанавливаем для декораторов
    EventHandler.set_event_bus(event_bus)
    
    # 3. Создаем компоненты системы
    print("🔧 Step 2: Initializing components...")
    token_service = TokenService(event_bus)
    graph_manager = GraphManager(event_bus)
    dna_guardian = DNAGuardian(event_bus)
    auto_builder = AutoGraphBuilder(graph_manager)
    event_logger = EventLogger()
    stats_collector = StatisticsCollector()
    print()
    
    # 4. Регистрируем все обработчики
    print("📋 Step 3: Registering event handlers...")
    EventHandler.register_all()
    print("✅ All handlers registered\n")
    
    # Пауза для инициализации
    await asyncio.sleep(0.1)
    
    print("="*70)
    print("🚀 Starting Event Generation...")
    print("="*70 + "\n")
    
    # 5. Генерируем события через различные сервисы
    
    # Сценарий 1: Создание токенов (с автоматическим созданием связей)
    print("📦 Scenario 1: Creating tokens...\n")
    
    await token_service.create_token(
        token_id="token_001",
        coordinates=[10, 20, 5],
        data={"type": "data", "value": 42}
    )
    await asyncio.sleep(0.1)
    
    await token_service.create_token(
        token_id="token_002",
        coordinates=[15, 25, 8],
        data={"type": "neural", "activation": 0.9}
    )
    await asyncio.sleep(0.1)
    
    await token_service.create_token(
        token_id="token_003",
        coordinates=[20, 30, 10],
        data={"type": "experience"}
    )
    await asyncio.sleep(0.1)
    
    print()
    print("-"*70 + "\n")
    
    # Сценарий 2: Обновление и активация токенов
    print("🔄 Scenario 2: Updating and activating tokens...\n")
    
    await token_service.update_token(
        "token_001",
        {"value": 100, "updated": True}
    )
    await asyncio.sleep(0.1)
    
    await token_service.activate_token("token_002", activation_level=0.95)
    await asyncio.sleep(0.1)
    
    await token_service.activate_token("token_003", activation_level=0.65)
    await asyncio.sleep(0.1)
    
    print()
    print("-"*70 + "\n")
    
    # Сценарий 3: Создание дополнительных токенов для кластера
    print("🎯 Scenario 3: Creating more tokens to trigger cluster detection...\n")
    
    for i in range(4, 7):
        await token_service.create_token(
            token_id=f"token_00{i}",
            coordinates=[25 + i*5, 35 + i*5, 15 + i*2],
            data={"type": "cluster_member", "index": i}
        )
        await asyncio.sleep(0.1)
    
    print()
    print("-"*70 + "\n")
    
    # Сценарий 4: Мутация DNA
    print("🧬 Scenario 4: DNA mutation...\n")
    
    await dna_guardian.mutate_dna(
        "expand_capacity",
        {"max_tokens": 2000, "max_connections_per_token": 20}
    )
    await asyncio.sleep(0.1)
    
    print()
    print("-"*70 + "\n")
    
    # Сценарий 5: Попытка обновить несуществующий токен (генерация ошибки)
    print("❌ Scenario 5: Triggering an error...\n")
    
    await token_service.update_token("token_999", {"value": 1})
    await asyncio.sleep(0.1)
    
    print()
    print("-"*70 + "\n")
    
    # Сценарий 6: Ручное создание связи
    print("🔗 Scenario 6: Manual connection creation...\n")
    
    await graph_manager.add_connection(
        "token_001",
        "token_005",
        weight=1.5,
        connection_type="manual"
    )
    await asyncio.sleep(0.1)
    
    print()
    print("="*70 + "\n")
    
    # 6. Выводим финальную статистику
    print("📊 Final Results\n")
    
    # Статистика от коллектора
    stats_collector.print_stats()
    
    # Статистика Event Bus
    print("📈 Event Bus Metrics:")
    print("="*70)
    metrics = event_bus.get_metrics()
    if metrics:
        print(f"   Total events published: {metrics['total_published']}")
        print(f"   Total events delivered: {metrics['total_delivered']}")
        print(f"   Delivery rate: {metrics['delivery_rate']:.2%}")
        print()
        print("   Events by type:")
        for event_type, count in sorted(metrics['events_by_type'].items()):
            print(f"      {event_type}: {count}")
        print()
        print("   Events by priority:")
        for priority, count in sorted(metrics['events_by_priority'].items()):
            print(f"      {priority}: {count}")
    print("="*70 + "\n")
    
    # Информация о подписках
    print("🔌 Subscriptions Info:")
    print("="*70)
    info = event_bus.get_subscriptions_info()
    print(f"   Total subscriptions: {info['total_subscriptions']}")
    print(f"   Queue size: {info['queue_size']}")
    print(f"   Is running: {info['is_running']}")
    print()
    print("   Named subscriptions:")
    for name, sub_info in info['named_subscriptions'].items():
        print(f"      {name}:")
        print(f"         Subscriber: {sub_info['subscriber_id']}")
        print(f"         Calls: {sub_info['call_count']}")
    print("="*70 + "\n")
    
    # 7. Graceful shutdown
    print("🛑 Shutting down Event Bus...")
    await stop_event_bus()
    print("✅ Event Bus stopped\n")
    
    print("="*70)
    print("✅ Demo completed successfully!")
    print("="*70)
    print("""
╔════════════════════════════════════════════════════════════════════╗
║                    Integration Successful!                          ║
║                                                                     ║
║  This demo showed:                                                  ║
║  ✓ TokenService generating events                                  ║
║  ✓ GraphManager reacting to tokens                                 ║
║  ✓ DNAGuardian validating operations                               ║
║  ✓ AutoGraphBuilder creating automatic connections                 ║
║  ✓ EventLogger logging important events                            ║
║  ✓ StatisticsCollector tracking everything                         ║
║  ✓ Event chains (Token → Connection → Cluster)                     ║
║  ✓ Error handling through events                                   ║
║  ✓ Priority-based event delivery                                   ║
║  ✓ Metrics and monitoring                                          ║
║                                                                     ║
║  The Event System is the heartbeat of NeuroGraph OS!              ║
╚════════════════════════════════════════════════════════════════════╝
    """)


if __name__ == "__main__":
    asyncio.run(main())
():
    """Демонстрация работы Event System"""
    
    print("=" * 70)
    print("NeuroGraph OS - Event System Demo")
    print("=" * 70)
    print()
    
    # 1. Создаем и запускаем Event Bus
    event_bus = EventBus(
        max_queue_size=1000,
        enable_metrics=True,
        log_events=False
    )
    await event_bus.start()
    print("✅ Event Bus started\n")
    
    # 2. Устанавливаем глобальную шину для декораторов
    EventHandler.set_event_bus(event_bus)
    
    # 3. Создаем компоненты системы
    token_service = TokenService(event_bus)
    graph_manager = GraphManager(event_bus)
    dna_guardian = DNAGuardian(event_bus)
    
    # 4. Инициализируем обработчики
    handlers = EventHandlers()
    auto_builder = AutoGraphBuilder(event_bus, graph_manager)
    
    # 5. Регистрируем все обработчики
    EventHandler.register_all()
    print("✅ All handlers registered\n")
    
    # Небольшая пауза для инициализации
    await asyncio.sleep(0.1)
    
    print("🚀 Starting event generation...\n")
    print("-" * 70)
    
    # 6. Генерируем события через различные сервисы
    
    # Создание токенов (цепочка событий)
    await token_service.create_token(
        token_id="token_001",
        coordinates=[10, 20, 5],
        data={"type": "data", "value": 42}
    )
    await asyncio.sleep(0.1)
    
    await token_service.create_token(
        token_id="token_002",
        coordinates=[15, 25, 8],
        data={"type": "neural", "activation": 0.9}
    )
    await asyncio.sleep(0.1)
    
    await token_service.create_token(
        token_id="token_003",
        coordinates=[20, 30, 10],
        data={"type": "experience"}
    )
    await asyncio.sleep(0.1)
    
    print("-" * 70)
    
    # Обновление токена
    await token_service.update_token(
        "token_001",
        {"value": 100}
    )
    await asyncio.sleep(0.1)
    
    # Активация токена (высокий приоритет)
    await token_service.activate_token("token_002", activation_level=0.95)
    await asyncio.sleep(0.1)
    
    print("-" * 70)
    
    # Мутация DNA
    await dna_guardian.mutate_dna(
        "expand_capacity",
        {"max_tokens": 2000}
    )
    await asyncio.sleep(0.1)
    
    print("-" * 70)
    
    # Попытка обновить несуществующий токен (генерация ошибки)
    await token_service.update_token("token_999", {"value": 1})
    await asyncio.sleep(0.1)
    
    print("-" * 70)
    print()
    
    # 7. Выводим статистику
    print("📊 Statistics:")
    print("-" * 70)
    
    metrics = event_bus.get_metrics()
    if metrics:
        print(f"Total events published: {metrics['total_published']}")
        print(f"Total events delivered: {metrics['total_delivered']}")
        print(f"Delivery rate: {metrics['delivery_rate']:.2%}")
        print()
        print("Events by type:")
        for event_type, count in metrics['events_by_type'].items():
            print(f"  {event_type}: {count}")
        print()
    
    info = event_bus.get_subscriptions_info()
    print(f"Total subscriptions: {info['total_subscriptions']}")
    print(f"Named subscriptions: {len(info['named_subscriptions'])}")
    print()
    
    print(f"Processed tokens: {len(handlers.processed_tokens)}")
    print(f"Graph updates: {len(handlers.graph_updates)}")
    print()
    
    # 8. Graceful shutdown
    print("🛑 Shutting down Event Bus...")
    await event_bus.stop()
    print("✅ Event Bus stopped")
    print()
    print("=" * 70)
    print("Demo completed successfully!")
    print("=" * 70)


if __name__ == "__main__":
    asyncio.run(main())