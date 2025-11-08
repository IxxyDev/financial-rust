# YPBank Compare

CLI-приложение для сравнения двух файлов транзакций в разных форматах.

## Использование

```bash
ypbank_compare --file1 <file1> --format1 <format> --file2 <file2> --format2 <format>
```

### Параметры

- `--file1` - путь к первому файлу
- `--format1` - формат первого файла (csv, text, binary)
- `--file2` - путь ко второму файлу
- `--format2` - формат второго файла (csv, text, binary)

## Примеры

Сравнение CSV и бинарного файла:

```bash
ypbank_compare --file1 records.csv --format1 csv --file2 records.bin --format2 binary
```

Если файлы идентичны:

```
The transaction records in 'records.csv' and 'records.bin' are identical.
```

Если есть различия:

```
The transaction records in 'records.csv' and 'records.bin' differ:

Transaction #1 (ID: TX001):
  Amount: 1000.50 USD vs 1000.00 USD
```

## Логика сравнения

Приложение сравнивает транзакции попарно по всем полям:
- ID транзакции
- Даты (posted и executed)
- Тип (Credit/Debit)
- Сумму и валюту
- Описание, счёт, контрагент, категорию

При обнаружении различий приложение завершается с кодом 1.

## Сборка

```bash
cargo build --release
```

Бинарник будет доступен по пути: `target/release/ypbank_compare`
