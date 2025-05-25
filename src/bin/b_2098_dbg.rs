fn tsp_debug(n: usize, grid: &[Vec<usize>]) -> usize {
    println!("=== TSP 시작 ===");
    println!("도시 수: {}", n);
    println!("거리 grid:");
    for i in 0..n {
        println!("{:?}", grid[i]);
    }
    println!();

    // dp[mask][i] = 지금 위치가 i 이며, mask로 표현된 도시를 방문했을때 최소 비용
    let mut dp = vec![vec![usize::MAX; n]; 1 << n];

    // 0001 0번 도시만 방문한 비용 0
    dp[1][0] = 0;
    println!("초기 상태: dp[1][0] = 0 (0번 도시에서 시작)");
    println!();

    // 모든 방문 경로에서
    for mask in 1..(1 << n) {
        println!("=== mask = {} (binary: {:0width$b}) ===", mask, mask, width = n);
        print!("방문한 도시들: ");
        for i in 0..n {
            if (mask & (1 << i)) != 0 {
                print!("{} ", i);
            }
        }
        println!();

        let mut has_valid_state = false;

        // 해당 반복문의 시작 지점을 찾는다
        for i in 0..n {
            // mask & (1 << i) == 0 : 방문하지 않은 도시를 체크한다. 즉 방문하지 않은 도시에서 출발할 수 없다.
            // dp[mask][i] == usize::MAX 아직 도달하지 못한 경로
            if (mask & (1 << i)) == 0 {
                continue; // 방문하지 않은 도시에서는 출발할 수 없음
            }

            if dp[mask][i] == usize::MAX {
                continue; // 아직 도달하지 못한 경로
            }

            println!("  현재 위치 {}: 비용 = {}", i, dp[mask][i]);
            has_valid_state = true;

            // 지금기준의 방문경로(mask)와 시작점(i) 다음 지점(j) 최소 비용을 구하고 memo
            for j in 0..n {
                if (mask & (1 << j)) != 0 {
                    continue; // 이미 방문한 도시
                }

                if grid[i][j] == 0 {
                    continue; // 갈 수 없는 경로
                }

                let next_mask = mask | (1 << j);
                let new_cost = dp[mask][i] + grid[i][j];

                println!("    {}→{}: 현재비용({}) + 이동비용({}) = {} (다음 mask: {:0width$b})",
                         i, j, dp[mask][i], grid[i][j], new_cost, next_mask, width = n);

                if new_cost < dp[next_mask][j] {
                    println!("      dp[{:0width$b}][{}] 업데이트: {} → {}",
                             next_mask, j,
                             if dp[next_mask][j] == usize::MAX { "MAX".to_string() } else { dp[next_mask][j].to_string() },
                             new_cost, width = n);
                    print!("        (방문도시: ");
                    for k in 0..n {
                        if (next_mask & (1 << k)) != 0 {
                            print!("{} ", k);
                        }
                    }
                    println!(")");
                    dp[next_mask][j] = new_cost;
                } else {
                    println!("      dp[{:0width$b}][{}] 유지: {} (새 비용 {} 더 큼)",
                             next_mask, j, dp[next_mask][j], new_cost, width = n);
                }
            }
        }

        if !has_valid_state {
            println!("  (이 mask에서 유효한 상태 없음)");
        }
        println!();
    }

    let full_mask = (1 << n) - 1;
    println!("=== 최종 결과 계산 ===");
    println!("모든 도시 방문 완료 mask: {} (binary: {:0width$b})", full_mask, full_mask, width = n);

    let mut result = usize::MAX;

    for i in 1..n {
        if dp[full_mask][i] != usize::MAX && grid[i][0] > 0 {
            let total_cost = dp[full_mask][i] + grid[i][0];
            println!("도시 {}에서 시작점으로: {}(방문비용) + {}(복귀비용) = {}",
                     i, dp[full_mask][i], grid[i][0], total_cost);
            result = result.min(total_cost);
        } else if dp[full_mask][i] == usize::MAX {
            println!("도시 {}: 도달 불가능", i);
        } else if grid[i][0] == 0 {
            println!("도시 {}: 시작점으로 복귀 불가능", i);
        }
    }

    println!("최종 결과: {}", if result == usize::MAX { "불가능".to_string() } else { result.to_string() });
    result
}

// 테스트 함수
fn test_tsp() {
    println!("==== 테스트 케이스 : 4개 도시 ====");
    let grid = vec![
        vec![0, 10, 15, 20],
        vec![5, 0, 9, 10],
        vec![6, 13, 0, 12],
        vec![8, 8, 9, 0]
    ];
    tsp_debug(4, &grid);
}

fn main() {
    test_tsp();
}