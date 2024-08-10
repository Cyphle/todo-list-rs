use std::fmt::{Debug};
use sea_orm_migration::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

pub const UP_SQL_DIR: &str = "./migration/src/sql/up/";
pub const DOWN_SQL_DIR: &str = "./migration/src/sql/down/";

pub async fn execute_sql_scripts(db: &SchemaManagerConnection<'_>, path: &str, sql_file_names: &[&str]) -> Result<(), DbErr> {
    for file_name in sql_file_names {
        let file_path = build_file_path(path, file_name);
        if let Some(Err(err)) = find_and_execute_sql_instruction(db, &file_path).await {
            return Err(DbErr::Custom(format!("Failed to execute SQL instruction '{}': {}", file_name, err)));
        }
    }
    Ok(())
}

fn build_file_path(dir: &str, file_name: &str) -> PathBuf {
    Path::new(dir).join(file_name)
}

async fn find_and_execute_sql_instruction(db: &SchemaManagerConnection<'_>, path: &PathBuf) -> Option<Result<(), DbErr>> {
    match read_sql_file(path) {
        Ok(instruction) => Some(execute_sql(db, &instruction).await),
        Err(err) => Some(Err(DbErr::Custom(format!("Failed to read SQL file: {}", err)))),
    }
}

fn read_sql_file<P: AsRef<Path> + Debug + Copy>(path: P) -> Result<String, String> {
    fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file {:?}. Error: {}", path, e))
}

async fn execute_sql(db: &SchemaManagerConnection<'_>, instruction: &str) -> Result<(), DbErr> {
    db.execute_unprepared(instruction).await.map(|_res| ())
}

fn find_sql_file(path: &str) -> Result<String, DbErr> {
    match fs::read_to_string(Path::new(&path)) {
        Ok(content) => Ok(content),
        Err(err) => Err(DbErr::Custom(format!("Failed to read file: {}", err))),
    }
}

#[cfg(test)]
mod tests {
    use std::fs::ReadDir;
    use super::*;

    #[test]
    fn should_build_file_path() {
        let dir = "./migration/src/sql/up";
        let file_name = "01-create-table.sql";
        let expected_path = Path::new(dir).join(file_name);
        assert_eq!(build_file_path(dir, file_name), expected_path);
    }

    #[test]
    fn should_verify_that_files_exist() {
        let files: Vec<&str> = vec![
            "./src/sql/up/01-create-todo-lists-table.sql",
            "./src/sql/up/02-create-todo-list-items-table.sql",
            "./src/sql/up/03-add-foreign-key-to-todo-list-items.sql",
            "./src/sql/down/01-drop-todo-lists-table.sql",
            "./src/sql/down/02-drop-todo-list-items-table.sql",
            "./src/sql/down/03-drop-foreign-key-in-todo-list-items.sql"
        ];

        for path in files {
            assert!(find_sql_file(path).is_ok());
        }
    }

    #[test]
    fn up_and_down_files_should_be_equals() {
        const UP_SQL_DIR: &str = "./src/sql/up/";
        const DOWN_SQL_DIR: &str = "./src/sql/down/";
        let up_files = fs::read_dir(UP_SQL_DIR).unwrap();
        let down_files = fs::read_dir(DOWN_SQL_DIR).unwrap();

        let mut up_file_numbers: Vec<String> = Vec::new();
        let mut down_file_numbers: Vec<String> = Vec::new();

        pass_through_files(up_files, &mut up_file_numbers);

        pass_through_files(down_files, &mut down_file_numbers);

        for up_num in &up_file_numbers {
            assert!(
                down_file_numbers.contains(up_num),
                "No matching down file for up script number: {}",
                up_num
            );
        }
    }

    fn pass_through_files(down_files: ReadDir, down_file_numbers: &mut Vec<String>) {
        for entry in down_files {
            let entry = entry.unwrap();
            let file_name = entry.file_name();
            let file_name_str = file_name.to_str().unwrap();

            // Extract the script number from the filename
            if let Some(script_number) = file_name_str.split('-').next() {
                down_file_numbers.push(script_number.to_string());
            }
        }
    }
}