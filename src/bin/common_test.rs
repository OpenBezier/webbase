use webbase;

#[tokio::main]
pub async fn main() {
    let current_day = webbase::db_datatime::get_current_day();
    println!("current_day: {:?}", current_day);

    let current_datetime = webbase::db_datatime::get_current_time_east_8_str();
    println!("current_datetime: {:?}", current_datetime);

    println!("{}", webbase::common::get_uuid_string_without_minus());
    println!("{}", webbase::common::get_uuid());
    println!("{}", webbase::common::get_uuid_string());
}
