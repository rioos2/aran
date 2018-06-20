
pub struct Range {
    from: String,
    to: String,
}

impl Range {
    pub fn new(from: String, to: String) -> Self {
        Range { from: from, to: to }
    }
    pub fn get_ip_list(&self) -> Vec<String> {
        let x1 = self.from.split('.').collect::<Vec<_>>();
        let x2 = self.to.split('.').collect::<Vec<_>>();
        let d1 = x1[3].parse::<i32>().unwrap();
        let d2 = x2[3].parse::<i32>().unwrap() + 1;
        let mut data = vec![];
        for x in d1..d2 {
            data.push(format!("{}.{}.{}.{}", x1[0], x1[1], x1[2], x));
        }
        data
    }
}
