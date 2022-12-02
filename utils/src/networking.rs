// pub async fn get_input(year: u32, day: u32) -> Result<String, InputError> {
//     let path = format!("inputs/{}/{:0>2}.txt", year, day);
//     match read_input(&path).await {
//         Ok(input) => Ok(input),
//         Err(_) => {
//             let mut input = download_input(year, day).await?;
//             if input.chars().last() == Some('\n') {
//                 input.truncate(input.len() - 1); // remove the trailing newline
//             }
//             write_input(year, path, &input).await?;
//             Ok(input)
//         }
//     }
// }

// async fn download_input(year: u32, day: u32) -> SurfResult<String> {
//     surf::get(format!(
//         "https://adventofcode.com/{}/day/{}/input",
//         year, day
//     ))
//     .set_header("cookie", format!("session={}", SESSION_TOKEN))
//     .recv_string()
//     .await
// }