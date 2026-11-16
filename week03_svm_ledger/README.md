# Модуль 3. Симулятор расчетного реестра SVM на Rust

Автономная консольная программа на языке Rust, моделирующая модель исполнения инструкций среды Solana (SVM): двоичную сериализацию Borsh, модель аккаунтов, проверку полномочий подписанта и безопасную арифметику при разделении заимствований среза аккаунтов.

## Архитектура модуля

- `src/ledger.rs` — структуры `Address` (32-байтный ключ), `SimAccount` (баланс, буфер данных, владелец, флаги `is_signer` и `is_writable`), а также состояния `EpochQuotaState` и `OperatorRegistryEntry`.
- `src/instructions.rs` — перечисление `LedgerInstruction` с методами кодирования/декодирования Borsh.
- `src/engine.rs` — диспетчер `LedgerEngine::execute`, реализующий проверку владельца (`owner == program_id`), раздельное мутабельное заимствование через `accounts.split_at_mut(1)` и контролируемую арифметику (`checked_add`, `checked_sub`).
- `src/errors.rs` — типизированный enum ошибок `LedgerError` на базе `thiserror`.
- `src/main.rs` — точка входа CLI и набор модульных тестов.

## Команды запуска

```bash
cd week03_svm_ledger
cargo test
cargo run -- simulate
cargo run -- quota 100000 25000
cargo run -- fault-check
```
