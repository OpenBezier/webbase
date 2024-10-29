use common;

#[tokio::main]
pub async fn main() {
    let current_day = common::db_datatime::get_current_day();
    println!("current_day: {:?}", current_day);

    let current_datetime = common::db_datatime::get_current_time_east_8_str();
    println!("current_datetime: {:?}", current_datetime);

    println!("{:?}", common::common::get_uuid_string_without_minus());
    println!("{:?}", common::common::get_uuid());
    println!("{:?}", common::common::get_uuid_string());

    common::keycenter::keycenter_check_health(&"http://49.235.147.196:6013/".into())
        .await
        .unwrap();

    let result = common::keycenter::keycenter_encode_token(
        &"http://49.235.147.196:6013/".into(),
        common::keycenter::JwtEncodeReq {
            account: "test".into(),
            name: "testname".into(),
            appid: "onetwo".into(),
        },
    )
    .await
    .unwrap();
    println!("{:?}", result);

    println!(
        "{:?}",
        common::keycenter::keycenter_docode_token(
            &"http://49.235.147.196:6013/".into(),
            result.access
        )
        .await
        .unwrap()
    );
}
