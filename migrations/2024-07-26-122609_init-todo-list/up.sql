CREATE TABLE IF NOT EXISTS to_do_list(
    id      SERIAL PRIMARY KEY,
    title   VARCHAR(255)
);

CREATE TABLE IF NOT EXISTS to_do_list_item(
    id              SERIAL PRIMARY KEY,
    title           VARCHAR(255),
    content         TEXT,
    to_do_list_id   INTEGER
);

ALTER TABLE IF EXISTS to_do_list_item ADD CONSTRAINT fk_to_do_list_item_to_do_list_id FOREIGN KEY (to_do_list_id) REFERENCES to_do_list(id);
CREATE INDEX IF NOT EXISTS to_do_list_item_idx ON to_do_list_item(id);