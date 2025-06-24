fn main() {
    let target = "0.pool.ntp.org:123";
    let res = ntp_client::Client::new()
            .target(target).expect("Failed to target NTP server.")
            .format(Some("%Y/%m/%d %H:%M:%S.%3f"))
            .request().expect("Failed to request NTP time");

    let res_str = res.get_datetime_str().expect("Failed to get datetime as a string.");

    println!("wxstar.dev / EKG");
    println!("This software is licensed under the AGPL V3.0 License.\n");

    
    println!("Current UTC time ... {res_str}");
}
