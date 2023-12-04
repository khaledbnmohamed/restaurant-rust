#[cfg(test)]
mod tests {
    use crate::table::Table;

    #[test]
    fn test_table_add_item() -> Result<(), String> {
        let table_number = 12;
        let mut t = Table::new(table_number);
        let item_id = 4;
        t.add_item(item_id);
        t.items.get(&item_id).unwrap();
        Ok(())
    }

    #[test]
    fn test_table_check_item() -> Result<(), String> {
        let mut t = Table::new(1);
        let item_id = 7;
        t.add_item(item_id);
        let i = t.check_item(item_id).unwrap();
        assert_eq!(i.id(), item_id);
        let i2 = t.check_item(123);
        assert_eq!(i2, None);
        Ok(())
    }

    #[test]
    fn test_table_delete_item() -> Result<(), String> {
        let mut t = Table::new(1);
        let item_id = 11;
        t.add_item(item_id);
        let i = t.delete_item(item_id).unwrap();
        assert_eq!(i.id(), item_id);
        let i2 = t.delete_item(item_id);
        assert_eq!(i2, None);
        Ok(())
    }
}