# YPBank Financial Data Parser

Библиотека для парсинга, сериализации и десериализации финансовых данных в различных форматах.

## Структура проекта

Проект состоит из трёх крейтов:

- **parser** - основная библиотека для парсинга и сериализации данных
- **converter** - CLI-приложение для конвертации между форматами
- **comparer** - CLI-приложение для сравнения файлов транзакций

## Поддерживаемые форматы

1. **CSV** - табличный формат с заголовками
2. **Text** - текстовый формат с ключ-значение парами
3. **Binary** - бинарный формат с magic number и версионированием

## Установка

```bash
cargo build --release
```

## Использование

### Конвертация файлов

```bash
cargo run --release --bin ypbank_converter -- \
  --input examples/transactions.csv \
  --input-format csv \
  --output-format binary > output.bin
```

Из stdin:

```bash
cat examples/transactions.csv | cargo run --release --bin ypbank_converter -- \
  --input - \
  --input-format csv \
  --output-format text
```

Поддерживаемые форматы: `csv`, `text` (или `txt`), `binary` (или `bin`)

### Сравнение файлов

```bash
cargo run --release --bin ypbank_compare -- \
  --file1 examples/transactions.csv \
  --format1 csv \
  --file2 examples/transactions.bin \
  --format2 binary
```

Результат:
```
The transaction records in 'examples/transactions.csv' and 'examples/transactions.bin' are identical.
```

## Использование библиотеки

```rust
use parser::{Format, parse, write};
use std::fs::File;
use std::io::BufReader;

let file = File::open("transactions.csv")?;
let reader = BufReader::new(file);
let batch = parse(reader, Format::Csv)?;

let mut output = Vec::new();
write(&batch, &mut output, Format::Binary)?;
```

## Запуск тестов

```bash
cargo test
```

## Примеры данных

В директории `examples/` находятся примеры файлов:
- `transactions.csv` - CSV формат
- `transactions.txt` - текстовый формат
- `transactions.bin` - бинарный формат

## Архитектура

### Библиотека parser

Использует трейты `Read` и `Write`, что позволяет работать с любыми источниками данных:
- Файлы
- Буферы в памяти
- stdin/stdout

Основные типы:
- `Transaction` - одна транзакция
- `TransactionBatch` - набор транзакций
- `Money` - сумма с валютой
- `TransactionKind` - тип (Credit/Debit)

### Обработка ошибок

Все функции возвращают `Result<T, Error>`:
- `Error::Io` - ошибки ввода-вывода
- `Error::Parse` - ошибки парсинга
- `Error::UnsupportedFormat` - неподдерживаемый формат

## Требования

- Rust 2021 edition
- Dependencies:
  - chrono ~0.4
  - rust_decimal ~1
  - clap ~4 (только для CLI)
