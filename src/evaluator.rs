use std::collections::HashMap;
use std::fmt::{Debug};
use std::hash::Hash;
use log::{debug, trace};
use crate::log_parser::ParsedLog;

fn comb_2(n: u64) -> u64 {
    if n == 0 { panic!("Comb number should not be 0."); }
    if n >= (1 << 32) {
        panic!("Comb_2: input value too large. Expect <= {}, Actual: {}", (1u64 << 32 - 1), n);
    }
    n * (n - 1) / 2
}

fn counter<'a, T: Eq + Hash + 'a>(list: impl Iterator<Item=&'a T>) -> HashMap<&'a T, u64> {
    let mut counter: HashMap<&T, u64> = HashMap::new();
    for item in list {
        *counter.entry(item).or_insert(0) += 1;
    }
    counter
}

pub(crate) fn get_accuracy<T1: Eq + Hash + Debug, T2: Eq + Hash + Debug>
(ground_truth: &Vec<T1>, parsed_result: &Vec<T2>) -> (f64, f64, f64, f64) {
    assert_eq!(ground_truth.len(), parsed_result.len());
    let mut gt_counter: HashMap<&T1, Vec<usize>> = HashMap::new();
    for (i, event_id) in ground_truth.iter().enumerate() {
        gt_counter.entry(event_id).or_insert(vec![]).push(i);
    }
    let real_pairs: u64 = gt_counter.values()
        .map(|x| comb_2(x.len() as u64))
        .sum();

    let mut pr_counter: HashMap<&T2, Vec<usize>> = HashMap::new();
    for (i, event_id) in parsed_result.iter().enumerate() {
        pr_counter.entry(event_id).or_insert(vec![]).push(i);
    }
    let parsed_pairs: u64 = pr_counter.values()
        .map(|x| comb_2(x.len() as u64))
        .sum();

    let mut accurate_pairs: u64 = 0;
    let mut accurate_events: u64 = 0;
    for (parsed_event_id, parsed_event_cluster) in pr_counter.iter() {
        let error_counter:HashMap<&T1, u64> = counter(
            parsed_event_cluster.iter().map(|&idx| &ground_truth[idx])
        );
        if error_counter.len() == 1 {
            let ground_truth_event_id = error_counter.keys().next().unwrap();
            if gt_counter[ground_truth_event_id].len() == pr_counter[parsed_event_id].len() {
                accurate_events += pr_counter[parsed_event_id].len() as u64;
            }
        }
        for &count in error_counter.values() {
            if count > 1 {
                accurate_pairs += comb_2(count);
            }
        }
    }

    let precision = accurate_pairs as f64 / parsed_pairs as f64;
    let recall = accurate_pairs as f64 / real_pairs as f64;
    let f_measure = 2. * precision * recall / (precision + recall);
    let accuracy = accurate_events as f64 / ground_truth.len() as f64;

    (precision, recall, f_measure, accuracy)
}

pub(crate) fn get_accuracy_detail<T1: Eq + Hash + Debug>
(ground_truth: Vec<T1>, parsed_result: &ParsedLog) -> () {
    assert_eq!(ground_truth.len(), parsed_result.parsed_list.len());
    let mut gt_counter: HashMap<&T1, Vec<usize>> = HashMap::new();
    for (i, event_id) in ground_truth.iter().enumerate() {
        gt_counter.entry(event_id).or_insert(vec![]).push(i);
    }

    let mut pr_counter: HashMap<&usize, Vec<usize>> = HashMap::new();
    for (i, event_id) in parsed_result.parsed_list.iter().enumerate() {
        pr_counter.entry(event_id).or_insert(vec![]).push(i);
    }

    for pr_event_id in pr_counter.keys() {
        let error_counter = counter(
            pr_counter[pr_event_id].iter().map(|&idx| &ground_truth[idx])
        );
        if error_counter.len() != 1 {
            debug!("{} - {:?} -> {:?}", pr_counter[pr_event_id].len(), pr_event_id, error_counter);
            trace!("{:?}", parsed_result.templates[**pr_event_id]);
        }
    }
    for gt_event_id in gt_counter.keys() {
        let error_counter = counter(
            gt_counter[gt_event_id].iter().map(|&idx| &parsed_result.parsed_list[idx])
        );
        if error_counter.len() != 1 {
            debug!("{} - {:?} -> {:?}", gt_counter[gt_event_id].len(), gt_event_id, error_counter);
            for i in error_counter.keys() {
                trace!("{:?}", parsed_result.templates[**i]);
            }
        }
    }


}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn comb_2_normal_success() {
        assert_eq!(comb_2(1), 0);
        assert_eq!(comb_2(2), 1);
        assert_eq!(comb_2(3), 3);
        assert_eq!(comb_2(4), 6);
        assert_eq!(comb_2(5), 10);
        assert_eq!(comb_2(100), 4950);
    }

    #[test]
    fn comb_2_large_success() {
        assert_eq!(comb_2(u32::MAX as u64), 9223372030412324865);
    }

    #[test]
    #[should_panic]
    fn comb_2_large_fail() {
        comb_2(1<<32);
    }

    #[test]
    #[should_panic]
    fn comb_2_very_large_fail() {
        comb_2( u64::MAX);
    }

    #[test]
    fn counter_normal_success() {
        let stub: Vec<i32> = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
        let test_counter: HashMap<&i32, u64> = counter(stub.iter());
        assert_eq!(test_counter[&1], 1);
        assert_eq!(test_counter[&2], 2);
        assert_eq!(test_counter[&3], 3);
        assert_eq!(test_counter[&4], 4);
    }

    #[test]
    fn counter_str_slice_success() {
        let mut stub: Vec<&str> = vec!["1", "2", "2", "3", "3", "3", "4", "4", "4", "4"];
        stub.sort();
        let test_counter: HashMap<&&str, u64> = counter(stub.iter());
        assert_eq!(test_counter[&"1"], 1);
        assert_eq!(test_counter[&"2"], 2);
        assert_eq!(test_counter[&"3"], 3);
        assert_eq!(test_counter[&"4"], 4);
    }

    #[test]
    fn counter_string_success() {
        let stub: Vec<String> =
            vec!["1", "2", "2", "3", "3", "3", "4", "4", "4", "4"]
                .into_iter()
                .map(|s| s.to_string())
                .collect();
        let test_counter: HashMap<&String, u64> = counter(stub.iter());
        assert_eq!(test_counter[&"1".to_string()], 1);
        assert_eq!(test_counter[&"2".to_string()], 2);
        assert_eq!(test_counter[&"3".to_string()], 3);
        assert_eq!(test_counter[&"4".to_string()], 4);
    }

    #[test]
    fn get_accuracy_trivial_success() {
        let gt = vec!['1', '1' , '2', '2', '3', '4'];
        let pr = vec![1, 1, 2, 2, 3, 4];
        let acc = get_accuracy(&gt, &pr);
        assert_eq!(acc.0, 1.0);
        assert_eq!(acc.1, 1.0);
        assert_eq!(acc.2, 1.0);
        assert_eq!(acc.3, 1.0);
    }

    #[test]
    fn get_accuracy_success() {
        let gt = vec!['1', '1', '1', '2', '2', '3', '3', '4'];
        let pr = vec![1, 1, 1, 2, 2, 2, 2, 4];
        let acc = get_accuracy(&gt, &pr);
        assert_eq!(acc.0, 5. / 9.);
        assert_eq!(acc.1, 1.0);
        assert_eq!(acc.2, 10. / 14.);
        assert_eq!(acc.3, 0.5);
    }
}