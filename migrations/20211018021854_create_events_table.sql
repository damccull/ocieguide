CREATE TABLE events(
    id uuid NOT NULL,
    PRIMARY KEY (id),
    stream_id uuid NOT NULL,
    event_data TEXT NOT NULL,
    occurred_at timestamptz NOT NULL
);
