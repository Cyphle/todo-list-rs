CREATE TABLE IF NOT EXISTS todo_list(
    id      SERIAL PRIMARY KEY,
    title   VARCHAR(255)
);

CREATE TABLE IF NOT EXISTS todo_list_item(
    id              SERIAL PRIMARY KEY,
    title           VARCHAR(255),
    content         TEXT,
    todo_list_id   INTEGER
);

ALTER TABLE IF EXISTS todo_list_item ADD CONSTRAINT fk_todo_list_item_todo_list_id FOREIGN KEY (todo_list_id) REFERENCES todo_list(id);
CREATE INDEX IF NOT EXISTS todo_list_item_idx ON todo_list_item(id);