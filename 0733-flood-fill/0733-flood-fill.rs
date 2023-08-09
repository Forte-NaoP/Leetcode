impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        use std::collections::VecDeque;

        let (n, m) = (image.len(), image[0].len());

        let mut fill_image = image.clone();
        let mut visit = vec![vec![false; image[0].len()]; image.len()];
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        
        queue.push_back((sr as usize, sc as usize));
        fill_image[sr as usize][sc as usize] = color;
        visit[sr as usize][sc as usize] = true;

        while !queue.is_empty() {
            let (r, c) = queue.pop_front().unwrap();
            if r >= 1 && image[r][c] == image[r-1][c] && !visit[r-1][c] {
                queue.push_back((r-1, c));
                fill_image[r-1][c] = color;
                visit[r-1][c] = true;
            }
            if r < n-1 && image[r][c] == image[r+1][c] && !visit[r+1][c] {
                queue.push_back((r+1, c));
                fill_image[r+1][c] = color;
                visit[r+1][c] = true;
            }
            if c >= 1 && image[r][c] == image[r][c-1] && !visit[r][c-1] {
                queue.push_back((r, c-1));
                fill_image[r][c-1] = color;
                visit[r][c-1] = true;
            }
            if c < m-1 && image[r][c] == image[r][c+1] && !visit[r][c+1] {
                queue.push_back((r, c+1));
                fill_image[r][c+1] = color;
                visit[r][c+1] = true;
            }

        }

        fill_image
    }
}