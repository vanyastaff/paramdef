# Сводка проблем для исправления

**Источник:** CODEBASE_ANALYSIS_2026-01-28.md  
**Всего проблем:** 20  
**Спецификация:** specs/002-code-quality-improvements/spec.md

---

## 🔴 КРИТИЧЕСКИЕ (3) - Немедленно

### Проблема #1: Panel::collapsed field
**Категория:** Архитектура  
**Файл:** `src/types/group/panel.rs:76-83, 154-156`  
**Описание:** Runtime состояние (collapsed) хранится в schema типе  
**Нарушает:** Принцип I (Immutability-First), инвариант "Schema is ALWAYS immutable"  
**Решение:** Перенести в Context/RuntimeNode  
**Время:** 2 дня  
**FR:** FR-001, FR-002, FR-003

---

### Проблема #2: Visibility mutations
**Категория:** Архитектура  
**Файл:** Все 23 типа  
**Описание:** `set_visibility_rule(&mut self)` позволяет мутацию schema  
**Нарушает:** Принцип I (Immutability-First)  
**Решение:** Убрать метод, оставить только builder pattern  
**Время:** 1 день  
**FR:** FR-004, FR-005

---

### Проблема #11: Validation errors без field paths
**Категория:** UX  
**Файл:** `src/core/error.rs:65-73`  
**Описание:** Nested validation errors не показывают полный путь (user.address.email)  
**Влияние:** HIGH - невозможно найти ошибку в сложных формах  
**Решение:** Добавить `path: String` в ValidationError  
**Время:** 1 день  
**FR:** FR-014

---

## 🟠 ВЫСОКИЙ ПРИОРИТЕТ (6) - Неделя 2

### Проблема #3: Документация (14 vs 23 типа)
**Категория:** Документация  
**Файл:** `src/node.rs:3`, `src/types/traits/base.rs:16-23`  
**Описание:** Устаревшая информация о количестве типов  
**Решение:** Обновить с 14 → 23 во всех местах  
**Время:** 1 час  
**FR:** FR-026

---

### Проблема #4: Arc wrapping boilerplate
**Категория:** Эргономика  
**Файл:** `src/context/mod.rs`  
**Описание:** `Context::new(Arc::new(schema))` каждый раз  
**Влияние:** HIGH - используется везде  
**Решение:** `Context::from_schema(schema)`  
**Время:** 2 часа  
**FR:** FR-007

---

### Проблема #5: Нет ValueBuilder
**Категория:** Эргономика  
**Файл:** `src/core/value/mod.rs`  
**Описание:** Создание Value::Object многословно  
**Решение:** Добавить builder для Value  
**Время:** 4 часа  
**FR:** FR-020

---

### Проблема #6: Required fields - 4 строки
**Категория:** Эргономика  
**Файл:** Все leaf types  
**Описание:** `.builder().label().required().build()` слишком много  
**Решение:** `Text::required(key, label)` shorthand  
**Время:** 3 часа  
**FR:** FR-008, FR-009, FR-010

---

### Проблема #7: Validation setup многословен
**Категория:** Эргономика  
**Файл:** `examples/03_validation.rs`  
**Описание:** 3 слоя (Rules, Rule, Expr) для простой валидации  
**Решение:** Builder shortcuts `.validate_email()`, etc.  
**Время:** 4 часа  
**FR:** FR-011, FR-012

---

### Проблема #10: Ошибки без подсказок
**Категория:** UX  
**Файл:** `src/core/error.rs`  
**Описание:** Errors говорят "что не так", не "как исправить"  
**Решение:** Добавить hints в error messages  
**Время:** 3 часа  
**FR:** FR-024

---

## 🟡 СРЕДНИЙ ПРИОРИТЕТ (8) - Недели 3-4

### Проблема #8: Дублирование *_arc() методов
**Категория:** API Complexity  
**Файл:** Builders (List, Object, Schema)  
**Описание:** `field()` и `field_arc()` для каждого метода (6+ пар)  
**Решение:** Скрыть `*_arc()` или унифицировать через trait  
**Время:** 2 часа  
**FR:** (не в FR, cleanup task)

---

### Проблема #9: Object bulk field setter
**Категория:** Эргономика  
**Файл:** `src/types/container/object.rs`  
**Описание:** `.field().field().field()` для больших объектов  
**Решение:** `.fields([...])` bulk method  
**Время:** 2 часа  
**FR:** FR-015

---

### Проблема #12: Нет error recovery helpers
**Категория:** Эргономика  
**Файл:** Context  
**Описание:** Нет `get_text_or()` для fallback defaults  
**Решение:** Добавить `*_or()` variants  
**Время:** 2 часа  
**FR:** FR-013

---

### Проблема #13: Excess cloning в hot paths
**Категория:** Производительность  
**Файл:** `src/context/mod.rs:184-188`  
**Описание:** Value клонируется 3 раза при set() с events  
**Решение:** Использовать Arc<Value> в Event  
**Время:** 4 часа  
**FR:** FR-016

---

### Проблема #14: HashMap allocations
**Категория:** Производительность  
**Файл:** `src/context/mod.rs:271-280`  
**Описание:** Transactional updates всегда аллоцируют HashMap  
**Решение:** SmallVec/stack buffer для <8 items  
**Время:** 3 часа  
**FR:** FR-017

---

### Проблема #15: Нет batch getter
**Категория:** Производительность  
**Файл:** Context (missing)  
**Описание:** get("a") + get("b") = N hash lookups  
**Решение:** `get_many(["a", "b"])`  
**Время:** 2 часа  
**FR:** FR-018

---

### Проблема #16: Doc examples не компилируются
**Категория:** Документация  
**Файл:** `src/prelude.rs:28-49`, другие  
**Описание:** ~20% examples помечены `ignore`  
**Решение:** Исправить примеры, использовать `no_run`  
**Время:** 4 часа  
**FR:** FR-021

---

### Проблема #17: Нет COOKBOOK.md
**Категория:** Документация  
**Файл:** (missing)  
**Описание:** Нет quick reference для частых задач  
**Решение:** Создать cookbook с 10+ recipes  
**Время:** 6 часов  
**FR:** FR-023

---

## 🔵 НИЗКИЙ ПРИОРИТЕТ (3) - Nice to have

### Проблема #18: Error handling guide
**Категория:** Документация  
**Файл:** `src/core/error.rs`  
**Описание:** Нет гайда по обработке ошибок  
**Решение:** Добавить guide в module docs  
**Время:** 2 часа  
**FR:** FR-022

---

### Проблема #19: Context 40+ методов
**Категория:** API Complexity  
**Файл:** `src/context/mod.rs`  
**Описание:** Overwhelming количество публичных методов  
**Решение:** Группировка с комментариями  
**Время:** 1 час  
**FR:** (documentation only)

---

### Проблема #20: Много iterator методов
**Категория:** API Complexity  
**Файл:** `src/context/mod.rs:516-621`  
**Описание:** 5 методов возвращают одинаковый тип  
**Решение:** Консолидировать с filter parameter  
**Время:** 3 часа  
**FR:** (optional improvement)

---

## Статистика

| Приоритет | Количество | Суммарное время |
|-----------|------------|-----------------|
| 🔴 КРИТИЧНО | 3 | 4 дня |
| 🟠 ВЫСОКИЙ | 6 | 3 дня |
| 🟡 СРЕДНИЙ | 8 | 5 дней |
| 🔵 НИЗКИЙ | 3 | 1 день |
| **ВСЕГО** | **20** | **~13 дней** |

## Покрытие Functional Requirements

| FR | Проблема | Приоритет |
|----|----------|-----------|
| FR-001, FR-002, FR-003 | #1 Panel collapsed | 🔴 |
| FR-004, FR-005 | #2 Visibility mutations | 🔴 |
| FR-014 | #11 Field paths | 🔴 |
| FR-026 | #3 Documentation | 🟠 |
| FR-007 | #4 Arc wrapping | 🟠 |
| FR-020 | #5 ValueBuilder | 🟠 |
| FR-008, FR-009, FR-010 | #6 Required shortcuts | 🟠 |
| FR-011, FR-012 | #7 Validation shortcuts | 🟠 |
| FR-024 | #10 Error hints | 🟠 |
| FR-015 | #9 Bulk fields | 🟡 |
| FR-013 | #12 Error recovery | 🟡 |
| FR-016 | #13 Cloning | 🟡 |
| FR-017 | #14 Allocations | 🟡 |
| FR-018 | #15 Batch getter | 🟡 |
| FR-021 | #16 Doc examples | 🟡 |
| FR-023 | #17 COOKBOOK | 🟡 |
| FR-022 | #18 Error guide | 🔵 |

## Рекомендуемый порядок исправления

### Sprint 1 (Неделя 1): Архитектурные критические

```
День 1-2: Проблема #1 (Panel collapsed)
День 3:   Проблема #2 (Visibility mutations)
День 4:   Проблема #11 (Field paths)
День 5:   Проблема #3 (Documentation)
```

**Результат:** Устранены все архитектурные нарушения

---

### Sprint 2 (Неделя 2): Эргономика API

```
День 1: Проблема #4 (Arc wrapping) + #6 (Required shortcuts)
День 2: Проблема #7 (Validation shortcuts)
День 3: Проблема #5 (ValueBuilder)
День 4: Проблема #10 (Error hints)
День 5: Проблема #12 (Error recovery)
```

**Результат:** 40-50% меньше boilerplate кода

---

### Sprint 3 (Неделя 3): Производительность

```
День 1-2: Проблема #13 (Cloning optimization)
День 3:   Проблема #14 (Allocations)
День 4:   Проблема #15 (Batch getter)
День 5:   Проблема #9 (Bulk fields) + benchmarks
```

**Результат:** 20-30% улучшение throughput

---

### Sprint 4 (Неделя 4): Документация и полировка

```
День 1-2: Проблема #16 (Doc examples) + #17 (COOKBOOK)
День 3:   Проблема #18 (Error guide)
День 4:   Проблема #8 (API cleanup) + #19, #20
День 5:   Review, integration testing, release prep
```

**Результат:** Готово к релизу с улучшенной документацией

---

## Метрики успеха (из Success Criteria)

| Метрика | Текущее | Целевое | Проблемы |
|---------|---------|---------|----------|
| Mutable schema fields | 1 (Panel) | 0 | #1 |
| `&mut self` на schema | 2 | 0 | #1, #2 |
| Boilerplate reduction | 0% | 40-50% | #4, #6, #7 |
| Doc examples compile | ~80% | 95%+ | #16 |
| Error hints | 0% | 100% | #10 |
| Nested error paths | 0% | 100% | #11 |
| Value clones per set | 3 | 1 | #13 |
| Heap allocs (txn <8) | Always | Never | #14 |
| API methods (Context) | 41 | ~35 | #8, #19, #20 |

---

## Связанные документы

- **CODEBASE_ANALYSIS_2026-01-28.md** - Полный анализ (источник проблем)
- **specs/002-code-quality-improvements/spec.md** - Спецификация решений
- **.specify/memory/constitution.md** - Архитектурные инварианты
- **docs/01-ARCHITECTURE.md** - Документация архитектуры
