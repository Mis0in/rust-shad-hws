# Описание проектов и полученных навыков

### 1. [Виртуальная машина для CHIP-8](chip8/src/)
- Возможности Pattern Matching
- Работа с эмуляцией инструкций (декодирование байткода, реализация опкодов).
- Понимание низкоуровневых систем (таймеры, обработка ввода)
- Навыки тестирования: проверка корректности эмуляции на тестовых ROM-ах.
- Работа с обработкой ошибок

### 2. [Flat Map (структура данных)](flatmap/src/lib.rs)
- Работа с дженериками и трейтами
- Улучшение асимпотики выполнения операции получения данных
- Бенчмарки

### 3. [Обход файловой системы](fswalk/src/lib.rs)
- Рекурсивный обход фс
- Работа с функциональным стилем раста

### 4. [Реализация вектора без аллокаций на куче](arrayvec/src/lib.rs)
- Работа с unsafe подмножеством раст

### 5. [Параллельный grep](pargrep/src/lib.rs)
- Работа с каналами (mpsc::channel) для межпоточной коммуникации. 
- Создание и управление потоками через thread::spawn.

### 6. [Макросы](stdmacro/src/lib.rs)
- Работа с декларативными макросами, написание нескольких простых макросов: deque![], map![], sorted_vec![].

### 7. [MPSC очередь](mpsc/src/lib.rs)
- Работа с умными указателями
- Опыт реализации механизма отправки/получения

# Extra
 [Парсер обратной польской нотации](polka/src/lib.rs)

 [Парсер ini файлов](ini/src/lib.rs)

 [Персистентный стек](pstack/src/lib.rs)

 [Бот для игры paperio](paperio/strategy/src/lib.rs)
