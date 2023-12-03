
#[cfg(test)]
mod tests {
    use crate::item::Item;
    #[test]
    fn test_item() -> Result<(), String> {
        let i = Item::new(1, 2, 3);

        assert_eq!(
            i,
            Item {
                item_id: 1,
                table_number: 2,
                preparation_time: 3,
            }
        );
        Ok(())
    }
}