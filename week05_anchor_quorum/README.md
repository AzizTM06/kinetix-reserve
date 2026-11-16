# Модуль 5. Программа голосования и кворума на Anchor (`kinetix_quorum`)

Смарт-контракт на фреймворке Anchor для подсчета голосов по предложениям протокола.

## Реализованная логика

- Аккаунт `ProposalTally` (73 байта с 8-байтным дискриминатором Anchor): хранит адрес куратора `curator`, идентификатор предложения `proposal_id`, счетчики `yes_votes` и `no_votes`, целевой порог `quorum_target` и флаг закрытия `is_sealed`.
- Инструкции:
  - `create_tally(proposal_id, quorum_target)` — инициализирует аккаунт голосования с проверкой `quorum_target > 0`.
  - `cast_vote(support, weight)` — добавляет вес голоса с контролем переполнения (`checked_add`) и блокировкой после закрытия.
  - `revoke_vote(support, weight)` — отзывает голос с защитой от underflow (`require!(votes >= weight)`).
  - `seal_tally()` — финализирует голосование с проверкой `has_one = curator @ QuorumError::Forbidden`.

## Проверка

```bash
cd week05_anchor_quorum
cargo test
anchor test

cd client
npx tsc --noEmit
npx ts-node quorum_cli.ts 5EWB2t83QvN9MMDB6fgCLatRZQBxHW1LSi1aYzSBpEu4
```
