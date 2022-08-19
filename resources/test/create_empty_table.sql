CREATE TABLE IF NOT EXISTS textual_objects
(
    id PRIMARY KEY                    NOT NULL,
    ticket_id      TEXT               NOT NULL,
    ticket_minimal TEXT  DEFAULT ''   NOT NULL,

    source_id      TEXT               NOT NULL,
    source_name    TEXT  DEFAULT ''   NOT NULL,
    source_id_type TEXT  DEFAULT ''   NOT NULL,
    source_path    TEXT  DEFAULT ''   NOT NULL,

    store_info     TEXT  DEFAULT ''   NOT NULL,
    store_url      TEXT  DEFAULT ''   NOT NULL,

    created        TIMESTAMP          NOT NULL,
    updated        TIMESTAMP          NOT NULL,

    json           JSONB DEFAULT '{}' NOT NULL,

    card           JSONB DEFAULT NULL,
    card_map       TEXT  DEFAULT ''   NOT NULL
)
