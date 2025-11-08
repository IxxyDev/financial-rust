# YPBank Converter

CLI-приложение для конвертации файлов транзакций между различными форматами.

## Использование

```bash
ypbank_converter --input <file> --input-format <format> --output-format <format>
```

### Параметры

- `--input, -i` - путь к входному файлу (используйте '-' для stdin)
- `--input-format` - формат входного файла (csv, text, binary)
- `--output-format` - формат выходного файла (csv, text, binary)

Результат выводится в stdout.

## Примеры

Конвертация CSV в бинарный формат:

```bash
ypbank_converter --input data.csv --input-format csv --output-format binary > data.bin
```

Конвертация из stdin:

```bash
cat data.csv | ypbank_converter --input - --input-format csv --output-format text
```

Цепочка конвертаций:

```bash
ypbank_converter --input data.csv --input-format csv --output-format binary | \
ypbank_converter --input - --input-format binary --output-format text
```

## Сборка

```bash
cargo build --release
```

Бинарник будет доступен по пути: `target/release/ypbank_converter`
