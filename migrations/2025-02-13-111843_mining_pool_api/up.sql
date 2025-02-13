CREATE TABLE IF NOT EXISTS wallet
(
    address         UUID PRIMARY KEY        NOT NULL,
    club_name       TEXT                    NOT NULL
);

CREATE TABLE IF NOT EXISTS miner
(
    id              UUID PRIMARY KEY        NOT NULL,
    address         UUID                    NOT NULL REFERENCES wallet(address),
    nickname        TEXT                    NOT NULL,
    hash_rate       INT                     NOT NULL,
    shares_mined    INT                     NOT NULL
);
