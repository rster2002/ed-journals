#[macro_export]
macro_rules! try_feed {
    ($i:expr) => {
        {
            let result = $i;

            if let FeedResult::Later = result {
                return result;
            }

            result
        }
    };
}
