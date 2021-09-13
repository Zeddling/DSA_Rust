mod linked_list;

fn main() {
    let mut log = linked_list::TransactionLog::new_empty();

    log.append("Run".to_string());
    log.append("Wait".to_string());
    log.append("Hide".to_string());
    log.append("Jump".to_string());
    log.append("Fly".to_string());
    println!("{:#?}", log);

    log.pop();
    log.pop();
    println!("{:#?}", log);
}
