# Parser Crate

Библиотека для парсинга и сериализации финансовых данных YPBank.

## Функциональность

- Парсинг данных из CSV, Text и Binary форматов
- Сериализация данных в CSV, Text и Binary форматы
- Использование трейтов `Read` и `Write` для гибкости работы с различными источниками

## Основные типы

### `TransactionBatch`

Представляет набор транзакций с опциональным идентификатором счёта.

### `Transaction`

Одна банковская транзакция со всеми полями:
- ID транзакции
- Дата проводки
- Дата исполнения (опционально)
- Тип (Credit/Debit)
- Сумма и валюта
- Описание
- Счёт, контрагент, категория (опционально)

## Примеры использования

```rust
use parser::{Format, parse, write};
use std::io::Cursor;

let csv_data = "TransactionId,PostedDate,ExecutedDate,Type,Amount,Currency,Description,Account,Counterparty,Category
TX001,2024-01-15,2024-01-15 10:30:00,Credit,1000.50,USD,Payment,ACC123,Company,Salary";

let cursor = Cursor::new(csv_data);
let batch = parse(cursor, Format::Csv)?;

let mut output = Vec::new();
write(&batch, &mut output, Format::Text)?;
```

## Обработка ошибок

Все функции возвращают `Result<T, Error>` где `Error` может быть:
- `Io` - ошибки ввода-вывода
- `Parse` - ошибки парсинга с указанием формата и сообщения
- `UnsupportedFormat` - неподдерживаемый формат
