# src/main.py - ПОЛНОЕ ОБНОВЛЕНИЕ

from src.core.token.factory import TokenFactory
from src.core.spatial.coordinate_system import CoordinateSystem
from src.core.dna.guardian import DNAGuardian
from src.core.dna.binary import CDNAStructure
from src.core.experience.stream import ExperienceStream
from src.core.dna.guardian import DNAGuardian
from src.core.experience.stream import DNAExperienceIntegration

def create_integrated_system():
    """Создать полностью интегрированную систему"""
    
    # 1. DNA Guardian
    dna_guardian = DNAGuardian()
    cdna_profile = dna_guardian.create_cdna_profile("explorer")
    dna_guardian.update_cdna("system_init", cdna_profile, {"token", "coordinate_system"})
    
    # 2. Experience Stream
    experience_stream = ExperienceStream()
    
    # 3. Интеграция DNA Guardian и Experience
    dna_experience_integration = DNAExperienceIntegration(dna_guardian, experience_stream)
    
    return dna_experience_integration



def create_system_with_dna_guardian(profile: str = "explorer"):
    """Создать систему с DNA Guardian и выбранным профилем"""
    
    # Создаем DNA Guardian
    guardian = DNAGuardian()
    
    # Загружаем профиль CDNA
    print(f"Loading CDNA profile: {profile}")
    cdna_profile = guardian.create_cdna_profile(profile)
    guardian.update_cdna("system_init", cdna_profile, {"token", "coordinate_system", "graph"})
    
    # Устанавливаем начальные ADNA параметры
    guardian.update_adna("learning_rate", 0.001, "system_init", {"graph"})
    guardian.update_adna("spatial_resolution", 0.01, "system_init", {"coordinate_system"})
    guardian.update_adna("token_weight_default", 0.5, "system_init", {"token"})
    guardian.update_adna("max_connections_per_token", 100, "system_init", {"graph"})
    
    # Создаем компоненты с DNA Guardian
    print("Initializing components with DNA Guardian...")
    token_factory = TokenFactory(dna_guardian=guardian)
    coordinate_system = CoordinateSystem(dna_guardian=guardian)
    
    print(f"System initialized with profile '{profile}'")
    print(f"Guardian stats: {guardian.get_statistics()}")
    
    return {
        "guardian": guardian,
        "token_factory": token_factory,
        "coordinate_system": coordinate_system,
        "profile": profile
    }

def demo_generation_tracking(system):
    """Демонстрация отслеживания поколений токенов"""
    
    factory = system["token_factory"]
    guardian = system["guardian"]
    
    print("\n=== Generation Tracking Demo ===")
    
    # Поколение 1
    print("Creating Generation 1 tokens...")
    gen1_tokens = factory.create_batch_with_generation_tracking(5, "random", {})
    gen1_marker = gen1_tokens[0].get_genetic_marker()
    print(f"Generation 1 marker: {hex(gen1_marker) if gen1_marker else 'None'}")
    
    # Изменяем ADNA (эволюция)
    print("\nEvolving ADNA parameters...")
    guardian.update_adna("learning_rate", 0.002, "evolution", {"graph"})
    guardian.update_adna("token_weight_default", 0.7, "evolution", {"token"})
    
    # Поколение 2
    print("Creating Generation 2 tokens...")
    gen2_tokens = factory.create_batch_with_generation_tracking(5, "random", {})
    gen2_marker = gen2_tokens[0].get_genetic_marker()
    print(f"Generation 2 marker: {hex(gen2_marker) if gen2_marker else 'None'}")
    
    # Сравниваем поколения
    if gen1_marker and gen2_marker and gen1_marker != gen2_marker:
        print(f"✓ Generations are distinguishable!")
        print(f"  Gen1 info: {gen1_tokens[0].get_generation_info()}")
        print(f"  Gen2 info: {gen2_tokens[0].get_generation_info()}")
    
    return gen1_tokens, gen2_tokens

def demo_hot_slices(system):
    """Демонстрация работы горячих срезов CDNA"""
    
    guardian = system["guardian"]
    
    print("\n=== Hot Slices Demo ===")
    
    # Получаем срезы для разных компонентов
    token_slice = guardian.get_cdna_slice("token")
    coord_slice = guardian.get_cdna_slice("coordinate_system")
    graph_slice = guardian.get_cdna_slice("graph")
    
    print(f"Token component slice: {len(token_slice)} bytes")
    print(f"CoordinateSystem slice: {len(coord_slice)} bytes")
    print(f"Graph component slice: {len(graph_slice)} bytes")
    print(f"Total CDNA size: 128 bytes")
    print(f"Memory saved by slicing: {128*3 - (len(token_slice) + len(coord_slice) + len(graph_slice))} bytes")
    
    # Проверяем кэширование
    stats_before = guardian.get_statistics()
    
    # Повторные запросы должны использовать кэш
    for _ in range(10):
        guardian.get_cdna_slice("token")
        guardian.get_cdna_slice("coordinate_system")
    
    stats_after = guardian.get_statistics()
    
    print(f"\nCache performance:")
    print(f"  Hits: {stats_after['cache_hits'] - stats_before['cache_hits']}")
    print(f"  Misses: {stats_after['cache_misses'] - stats_before['cache_misses']}")

def demo_profile_switching(system):
    """Демонстрация переключения профилей CDNA"""
    
    guardian = system["guardian"]
    factory = system["token_factory"]
    
    print("\n=== Profile Switching Demo ===")
    
    profiles = ["explorer", "analyzer", "creator"]
    
    for profile_name in profiles:
        print(f"\nSwitching to profile: {profile_name}")
        
        # Загружаем новый профиль
        new_profile = guardian.create_cdna_profile(profile_name)
        guardian.update_cdna("profile_switcher", new_profile, {"token", "coordinate_system", "graph"})
        
        # Создаем токен с новым профилем
        token = factory.create_empty()
        print(f"  Token created with weight: {token.weight}")
        print(f"  Guardian events published: {guardian.get_statistics()['events_published']}")

def main():
    """Главная функция с полной демонстрацией"""
    
    print("=" * 60)
    print("NeuroGraph OS - Enhanced DNA System")
    print("=" * 60)
    
    # Создаем систему с профилем "explorer"
    system = create_system_with_dna_guardian(profile="explorer")
    
    # Демонстрации
    demo_generation_tracking(system)
    demo_hot_slices(system)
    demo_profile_switching(system)
    
    # Финальная статистика
    print("\n" + "=" * 60)
    print("Final Statistics:")
    print("=" * 60)
    
    final_stats = system["guardian"].get_statistics()
    for key, value in final_stats.items():
        print(f"  {key}: {value}")
    
    print("\n✓ System demonstration complete!")

if __name__ == "__main__":
    main()