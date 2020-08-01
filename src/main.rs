use async_std::task;

// struct Problem {
//     id: String,
//     contest_id: String,
//     title: String
// }

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    task::block_on(async {
        let mut res = surf::get("https://kenkoooo.com/atcoder/resources/problems.json").await?;
        dbg!(res.body_string().await?);
        let body = res.body_string().await?;
        println!("{}", body);
        Ok(())
    })
}
