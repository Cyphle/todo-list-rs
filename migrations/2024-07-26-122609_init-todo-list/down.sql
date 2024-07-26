-- Drop the index
DROP INDEX IF EXISTS to_do_list_item_idx;

-- Drop the foreign key constraint
ALTER TABLE IF EXISTS to_do_list_item DROP CONSTRAINT IF EXISTS fk_to_do_list_item_to_do_list_id;

-- Drop the tables
DROP TABLE IF EXISTS to_do_list_item;
DROP TABLE IF EXISTS to_do_list;