# Модуль 4. Нативная программа телеметрии (`kinetix_native_beacon`)

Смарт-контракт на базовом крейте `solana-program` без макросов Anchor. Программа обслуживает аккаунты телеметрии (`BeaconRecord`), проверяет владельца данных (`account.owner == program_id`) и флаг записи, декодирует бинарную команду `BeaconInstruction` через Borsh и инкрементирует счетчик сигналов.

## Структура

- `src/lib.rs` — точка входа `entrypoint!(process_instruction)`, состояние `BeaconRecord` (до 62 байт), инструкции `Ping { status_code, tag }` и `Reset`, а также модульные тесты с моками `AccountInfo`.
- `client/send_beacon.ts` — клиентский скрипт на `@solana/web3.js` и `borsh` v2, формирующий транзакцию инициализации аккаунта (с расчетом Rent Exemption) и вызова `Ping`.

## Проверка

```bash
cd week04_native_beacon
cargo test

cd client
npx tsc --noEmit
npx ts-node send_beacon.ts <PROGRAM_ID>
```
