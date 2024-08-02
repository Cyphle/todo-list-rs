-- Drop the index
DROP INDEX IF EXISTS todo_list_items_todo_list_id_idx;
-- Drop the foreign key constraint
ALTER TABLE todo_list_items DROP CONSTRAINT IF EXISTS fk_todo_list_items_todo_lists;
-- Drop the table
DROP TABLE IF EXISTS todo_list_items;