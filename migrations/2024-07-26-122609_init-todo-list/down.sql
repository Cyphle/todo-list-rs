-- Drop the index
DROP INDEX IF EXISTS todo_list_item_idx;

-- Drop the foreign key constraint
ALTER TABLE IF EXISTS todo_list_item DROP CONSTRAINT IF EXISTS fk_todo_list_item_to_do_list_id;

-- Drop the tables
DROP TABLE IF EXISTS todo_list_item;
DROP TABLE IF EXISTS todo_list;