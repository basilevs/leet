// https://leetcode.com/problems/network-recovery-pathways

pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
    let n = online.len();
    let en = edges.len();
    let mut adjacent = vec![Vec::new(); n];
    for edge in edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        let price = edge[2];
        if online[u] && online[v] {
            adjacent[u].push((v, price));
        }
    }

    
    let mut queue = Vec::with_capacity(en);
    // (score, cost, node)
    queue.push((i32::MAX, 0i64, 0));
    let mut result = -1;
    while let Some((score, cost, node)) = queue.pop() {
        if node == n - 1 {
            result = result.max(score);
        }
        for &(next_node, price) in &adjacent[node] {
            if price <= result {
                continue;
            }
            let next_cost = cost + price as i64;
            if next_cost > k {
                continue;
            }
            let next_score = score.min(price);
            queue.push((next_score, next_cost, next_node));
        }
    }
    result

}

#[cfg(test)]
mod tests {
    use super::find_max_path_score;

    fn to_vector<const N: usize, const M: usize>(input: [[i32; M]; N]) -> Vec<Vec<i32>> {
        input.iter().map(|row| row.to_vec()).collect()
    }

    #[test]
    fn official1() {
        #[rustfmt::skip]
        let edges = [
            [0, 1,  5],
            [1, 3, 10],
            [0, 2,  3],
            [2, 3,  4],
        ];
        assert_eq!(3, find_max_path_score(to_vector(edges), vec![true, true, true, true], 10));
    }

    #[test]
    fn official2() {
        #[rustfmt::skip]
        let edges = [
            [0, 1, 7],
            [1, 4, 5],
            [0, 2, 6],
            [2, 3, 6],
            [3, 4, 2],
            [2, 4, 6],
        ];
        assert_eq!(6, find_max_path_score(to_vector(edges), vec![true, true, true, false, true], 12));
    }

    #[test]
    fn absent_path() {
        #[rustfmt::skip]
        let edges = [
            [0, 1, 5],
            [1, 2, 7],
        ];
        assert_eq!(
            -1,
            find_max_path_score(to_vector(edges), vec![true, false, true], 20)
        );
    }
}
