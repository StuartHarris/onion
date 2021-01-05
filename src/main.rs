/// 1. Pure. Don't think about IO at all
mod core {
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}

/// 2. think about IO but not its implementation
mod domain {
    use super::core;
    use anyhow::Result;
    use std::future::Future;

    pub async fn add<Fut>(get_x: impl Fn() -> Fut, y: i32) -> Result<i32>
    where
        Fut: Future<Output = Result<i32>>,
    {
        let x = get_x().await?;
        Ok(core::add(x, y))
    }
}

/// 3. IO implementation
mod infra {
    use anyhow::Result;

    pub async fn get_x() -> Result<i32> {
        // call DB
        Ok(7)
    }
}

/// 4. inject dependencies
mod api {
    use super::{domain, infra};
    use anyhow::Result;

    pub async fn add(y: i32) -> Result<i32> {
        let result = domain::add(infra::get_x, y).await?;
        Ok(result)
    }
}

fn main() {
    async_std::task::block_on(async {
        println!(
            "When we add 3 to the DB value (7), we get {:?}",
            api::add(3).await
        );
    })
}
