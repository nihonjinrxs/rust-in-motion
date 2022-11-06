#[derive(Debug)]
pub struct Bucket {
    pub liters: u32,
}

pub fn pour(source: &mut Bucket, target: &mut Bucket, amount: u32) {
    source.liters -= amount;
    target.liters += amount;
}
