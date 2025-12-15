use std::collections::HashMap;

advent_of_code::solution!(11);

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum Device {
    You,
    Out,
    Other(String),
}

impl From<&str> for Device {
    fn from(s: &str) -> Self {
        match s {
            "you" => Device::You,
            "out" => Device::Out,
            other => Device::Other(other.to_string()),
        }
    }
}

type Flows = HashMap<Device, Vec<Device>>;

pub fn paths(flows: &Flows, src: &Device, dst: &Device) -> u64 {
    fn dfs(flows: &Flows, current: &Device, dst: &Device, memo: &mut HashMap<Device, u64>) -> u64 {
        if current == dst {
            return 1;
        }
        if let Some(&cached) = memo.get(current) {
            return cached;
        }
        let mut total_paths = 0;
        if let Some(outputs) = flows.get(current) {
            for out in outputs {
                total_paths += dfs(flows, out, dst, memo);
            }
        }
        memo.insert(current.clone(), total_paths);
        total_paths
    }
    let mut memo: HashMap<Device, u64> = HashMap::new();
    dfs(flows, src, dst, &mut memo)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut flows = Flows::new();
    input
        .lines()
        .map(|line| {
            let (s, rest) = line.split_once(": ").unwrap();
            let source = Device::from(s);
            let outputs: Vec<Device> = rest.split_whitespace().map(Device::from).collect();
            (source, outputs)
        })
        .for_each(|(source, outputs)| {
            flows.insert(source, outputs);
        });

    let res = paths(&flows, &Device::You, &Device::Out);
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut flows = Flows::new();
    input
        .lines()
        .map(|line| {
            let (s, rest) = line.split_once(": ").unwrap();
            let source = Device::from(s);
            let outputs: Vec<Device> = rest.split_whitespace().map(Device::from).collect();
            (source, outputs)
        })
        .for_each(|(source, outputs)| {
            flows.insert(source, outputs);
        });

    let dac = Device::Other("dac".to_string());
    let fft = Device::Other("fft".to_string());
    let svr = Device::Other("svr".to_string());

    let n = paths(&flows, &dac, &fft);
    let res = if n > 0 {
        paths(&flows, &svr, &dac) * n * paths(&flows, &fft, &Device::Out)
    } else {
        paths(&flows, &svr, &fft) * paths(&flows, &fft, &dac) * paths(&flows, &dac, &Device::Out)
    };

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
