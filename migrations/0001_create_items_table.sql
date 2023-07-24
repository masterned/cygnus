CREATE TABLE IF NOT EXISTS item (
    item_id  INTEGER  PRIMARY KEY  NOT NULL,
    name     TEXT     NOT NULL,
    weight   DECIMAL  NOT NULL
);

CREATE UNIQUE INDEX item_id_idx ON item (item_id);
