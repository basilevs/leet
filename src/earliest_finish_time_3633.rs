pub fn earliest_finish_time(
    land_start_time: Vec<i32>,
    land_duration: Vec<i32>,
    water_start_time: Vec<i32>,
    water_duration: Vec<i32>,
) -> i32 {
    let first_land_end = land_start_time.iter().enumerate().map(|(i, s)| s + land_duration[i]).min().unwrap();
    let first_water_end = water_start_time.iter().enumerate().map(|(i, s)| s + water_duration[i]).min().unwrap();
    let water_end = water_start_time.iter().enumerate().map(|(i, &s)| s.max(first_land_end) + water_duration[i]).min().unwrap();
    let land_end = land_start_time.iter().enumerate().map(|(i, &s)| s.max(first_water_end) + land_duration[i]).min().unwrap();
    water_end.min(land_end)
}
