use ic_kit::prelude::*;

#[derive(Default)]
pub struct Counter {
    number: u64,
}

impl Counter {
    /// Increment the counter by one.
    pub fn increment(&mut self) -> u64 {
        self.number += 1;
        self.number
    }

    /// Increment the counter by the provided value.
    pub fn increment_by(&mut self, n: u8) -> u64 {
        self.number += n as u64;
        self.number
    }
}

#[update]
pub fn increment(counter: &mut Counter) -> u64 {
    counter.increment()
}

#[update]
pub fn increment_by(counter: &mut Counter, n: u8) -> u64 {
    counter.increment_by(n)
}

#[query]
pub fn get_counter(counter: &Counter) -> u64 {
    counter.number
}

#[derive(KitCanister)]
#[candid_path("playground.did")]
pub struct PlaygroundCanister;

#[cfg(test)]
mod counter_tests {
    use super::*;

    #[kit_test]
    async fn increment(replica: Replica) {
        let c = replica.add_canister(PlaygroundCanister::anonymous());

        let r = c
            .new_call("increment")
            .perform()
            .await
            .decode_one::<u64>()
            .unwrap();

        assert_eq!(r, 1);

        assert_eq!(
            c.new_call("get_counter")
                .perform()
                .await
                .decode_one::<u64>()
                .unwrap(),
            1
        );

        let r = c
            .new_call("increment")
            .perform()
            .await
            .decode_one::<u64>()
            .unwrap();

        assert_eq!(r, 2);

        assert_eq!(
            c.new_call("get_counter")
                .perform()
                .await
                .decode_one::<u64>()
                .unwrap(),
            2
        );
    }

    #[kit_test]
    async fn increment_by(replica: Replica) {
        let c = replica.add_canister(PlaygroundCanister::anonymous());
        assert_eq!(
            c.new_call("increment_by")
                .with_arg(2u8)
                .perform()
                .await
                .decode_one::<u64>()
                .unwrap(),
            2
        );
    }
}
