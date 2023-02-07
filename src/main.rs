/// 1. Pure. Don't think about IO at all
mod core {
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn adds() {
            let actual = super::add(1, 2);
            let expected = 3;
            assert_eq!(actual, expected);
        }
    }
}

/// 2. think about IO but not its implementation
mod domain {
    use super::core;
    use anyhow::Result;
    use std::future::Future;

    pub async fn add<F, Fut>(get_x: F, y: i32) -> Result<i32>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<i32>>,
    {
        let x = get_x().await?;

        let result = core::add(x, y);

        Ok(result)
    }
}

/// 3. IO implementation
mod infra {
    use anyhow::Result;

    pub async fn get_x() -> Result<i32> {
        // call DB, then..
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

#[async_std::main]
async fn main() {
    let result = api::add(3).await;
    println!("When we add 3 to the DB value (7), we get {result:?}");
}
