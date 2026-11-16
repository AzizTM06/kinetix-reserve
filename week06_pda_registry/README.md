# Модуль 6. Реестр узлов на PDA (`node_registry`)

Программа управления паспортами узлов и разработчиков с использованием Program Derived Addresses (PDA) и возвратом залога аренды.

## Архитектура аккаунта

- Аккаунт `NodePassport` (110 байт): `owner` (32 B), `callsign` (4 + 24 B), `region` (4 + 32 B), `role_tier` (1 B), `reputation` (4 B), `bump` (1 B).
- Семена PDA: `[b"node-passport", owner.key().as_ref()]`. Детерминированный адрес гарантирует не более одного паспорта на кошелек.
- Инструкции: `issue_passport`, `upgrade_passport` (проверка `has_one = owner`), `revoke_passport` (`close = owner` с возвратом всех lamports).

## Проверка

```bash
cd week06_pda_registry
cargo test
anchor test

cd client
npx tsc --noEmit
npx ts-node registry_cli.ts
```
