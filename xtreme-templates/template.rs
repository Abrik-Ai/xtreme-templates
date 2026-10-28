// IEEEXtreme 20.0 - team template (Rust)
//
// CONTENTS
//   1  Fast input / output          9  Sieve + gcd / lcm
//   2  Prefix sums (1D)            10  Fenwick tree (sums with updates)
//   3  Binary search on answer     11  Topological sort
//   4  BFS on a grid               12  Kadane + sliding window
//   5  BFS on a graph + path       13  Interactive I/O (GAME tasks)
//   6  Dijkstra                    14  2D prefix sums
//   7  DSU / union-find            15  Iterative DFS + components
//   8  Modular math (pow, inverse, nCr)
//  ADVANCED
//  16  Segment tree (point set, range query)    24  Floyd-Warshall (all pairs)
//  17  Lazy segment tree (range add, sum/min)   25  Kruskal MST
//  18  Sparse table (static range min)          26  SCC (Kosaraju, iterative)
//  19  LCA (binary lifting) + tree distance     27  Max flow (Dinic)
//  20  KMP prefix function + pattern search     28  Geometry: cross, convex hull, area
//  21  Z-function                               29  LIS (longest increasing subsequence)
//  22  String hashing (double mod)              30  Factorization (SPF) + extended gcd
//  23  Matrix power (e.g. Fibonacci mod p)
//  MORE
//  31  NTT polynomial multiply (mod 998244353)  36  Ternary search (max of unimodal f)
//  32  Bellman-Ford + negative cycle            37  Miller-Rabin primality (up to 2^64)
//  33  Bridges + articulation points            38  CRT (any moduli) + Euler totient
//  34  Trie (prefix counting, b'a'..b'z')       39  Segment intersection + point in polygon
//  35  Coordinate compression                   40  Manacher (longest palindrome)
#![allow(dead_code, unused_imports, unused_variables, unused_mut)]
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use std::io::{self, BufWriter, Read, Write};

const INF: i64 = i64::MAX / 4;

// ---------- 1. Fast input: read everything, hand out tokens ----------
struct Scanner {
    tokens: Vec<String>,
    pos: usize,
}

impl Scanner {
    fn new() -> Self {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).unwrap();
        let tokens = s.split_ascii_whitespace().map(String::from).collect();
        Scanner { tokens, pos: 0 }
    }
    fn next<T: std::str::FromStr>(&mut self) -> T {
        let t = &self.tokens[self.pos];
        self.pos += 1;
        t.parse().ok().expect("parse error")
    }
    fn word(&mut self) -> String {
        self.pos += 1;
        self.tokens[self.pos - 1].clone()
    }
}

// ---------- 2. Prefix sums ----------
// p[i] = a[0] + ... + a[i-1];  sum of a[l..=r] (0-indexed) = p[r+1] - p[l]
fn prefix(a: &[i64]) -> Vec<i64> {
    let mut p = vec![0i64; a.len() + 1];
    for i in 0..a.len() {
        p[i + 1] = p[i] + a[i];
    }
    p
}

// ---------- 3. Binary search on the answer ----------
// Smallest x in [lo, hi] with ok(x) true. ok must be F F F ... T T T.
fn first_true(mut lo: i64, mut hi: i64, ok: impl Fn(i64) -> bool) -> i64 {
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if ok(mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

// ---------- 4. BFS on a grid ----------
// grid: rows as byte vectors. Distances over cells that are not '#'. -1 = unreachable.
const MOVES: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn bfs_grid(grid: &[Vec<u8>], si: usize, sj: usize) -> Vec<Vec<i32>> {
    let (n, m) = (grid.len(), grid[0].len());
    let mut dist = vec![vec![-1i32; m]; n];
    let mut q = VecDeque::new();
    dist[si][sj] = 0;
    q.push_back((si, sj));
    while let Some((i, j)) = q.pop_front() {
        for &(di, dj) in MOVES.iter() {
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if ni < 0 || nj < 0 || ni >= n as i32 || nj >= m as i32 {
                continue;
            }
            let (ni, nj) = (ni as usize, nj as usize);
            if grid[ni][nj] != b'#' && dist[ni][nj] == -1 {
                dist[ni][nj] = dist[i][j] + 1;
                q.push_back((ni, nj));
            }
        }
    }
    dist
}

// ---------- 5. BFS on a graph (unweighted shortest path) ----------
// Returns (dist, par); par[v] = usize::MAX for the start / unreachable.
fn bfs_graph(adj: &[Vec<usize>], s: usize) -> (Vec<i64>, Vec<usize>) {
    let n = adj.len();
    let mut dist = vec![-1i64; n];
    let mut par = vec![usize::MAX; n];
    let mut q = VecDeque::new();
    dist[s] = 0;
    q.push_back(s);
    while let Some(u) = q.pop_front() {
        for &v in &adj[u] {
            if dist[v] == -1 {
                dist[v] = dist[u] + 1;
                par[v] = u;
                q.push_back(v);
            }
        }
    }
    (dist, par)
}

fn path_to(par: &[usize], mut t: usize) -> Vec<usize> {
    let mut path = vec![];
    while t != usize::MAX {
        path.push(t);
        t = par[t];
    }
    path.reverse();
    path
}

// ---------- 6. Dijkstra (non-negative weights) ----------
// adj[u] = list of (v, w)
fn dijkstra(adj: &[Vec<(usize, i64)>], s: usize) -> Vec<i64> {
    let mut dist = vec![INF; adj.len()];
    let mut pq = BinaryHeap::new();
    dist[s] = 0;
    pq.push(Reverse((0i64, s)));
    while let Some(Reverse((d, u))) = pq.pop() {
        if d > dist[u] {
            continue; // old entry
        }
        for &(v, w) in &adj[u] {
            if d + w < dist[v] {
                dist[v] = d + w;
                pq.push(Reverse((dist[v], v)));
            }
        }
    }
    dist
}

// ---------- 7. DSU / union-find ----------
struct Dsu {
    p: Vec<usize>,
    sz: Vec<usize>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Dsu { p: (0..n).collect(), sz: vec![1; n] }
    }
    fn find(&mut self, mut x: usize) -> usize {
        while self.p[x] != x {
            self.p[x] = self.p[self.p[x]]; // path halving
            x = self.p[x];
        }
        x
    }
    fn union(&mut self, a: usize, b: usize) -> bool {
        let (mut a, mut b) = (self.find(a), self.find(b));
        if a == b {
            return false;
        }
        if self.sz[a] < self.sz[b] {
            std::mem::swap(&mut a, &mut b);
        }
        self.p[b] = a;
        self.sz[a] += self.sz[b];
        true
    }
}

// ---------- 8. Modular math (MOD must be prime for the inverse) ----------
const MOD: i64 = 1_000_000_007;

fn power(mut a: i64, mut b: i64) -> i64 {
    let mut r = 1i64;
    a = ((a % MOD) + MOD) % MOD;
    while b > 0 {
        if b & 1 == 1 {
            r = r * a % MOD;
        }
        a = a * a % MOD;
        b >>= 1;
    }
    r
}

fn inverse(a: i64) -> i64 {
    power(a, MOD - 2)
}

struct Comb {
    fact: Vec<i64>,
    inv: Vec<i64>,
}

impl Comb {
    fn new(n: usize) -> Self {
        let mut fact = vec![1i64; n + 1];
        for i in 1..=n {
            fact[i] = fact[i - 1] * i as i64 % MOD;
        }
        let mut inv = vec![1i64; n + 1];
        inv[n] = inverse(fact[n]);
        for i in (1..=n).rev() {
            inv[i - 1] = inv[i] * i as i64 % MOD;
        }
        Comb { fact, inv }
    }
    fn ncr(&self, n: usize, r: usize) -> i64 {
        if r > n {
            return 0;
        }
        self.fact[n] * self.inv[r] % MOD * self.inv[n - r] % MOD
    }
}

// ---------- 9. Sieve + gcd / lcm ----------
fn sieve(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    if n >= 1 {
        is_prime[1] = false;
    }
    let mut i = 2;
    while i * i <= n {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    is_prime
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

// ---------- 10. Fenwick tree (range sums WITH updates) ----------
// 0-indexed outside, 1-indexed inside.
struct Fenwick {
    t: Vec<i64>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick { t: vec![0; n + 1] }
    }
    fn add(&mut self, i: usize, delta: i64) {   // a[i] += delta
        let mut i = i + 1;
        while i < self.t.len() {
            self.t[i] += delta;
            i += i & i.wrapping_neg();
        }
    }
    fn prefix_sum(&self, mut i: usize) -> i64 { // a[0] + ... + a[i-1]
        let mut s = 0;
        while i > 0 {
            s += self.t[i];
            i -= i & i.wrapping_neg();
        }
        s
    }
    fn range_sum(&self, l: usize, r: usize) -> i64 { // inclusive
        self.prefix_sum(r + 1) - self.prefix_sum(l)
    }
}

// ---------- 11. Topological sort (Kahn) ----------
// adj[u] = nodes that must come AFTER u. Returns an order, or empty if there is a cycle.
fn topo_sort(adj: &[Vec<usize>]) -> Vec<usize> {
    let n = adj.len();
    let mut indeg = vec![0usize; n];
    for u in 0..n {
        for &v in &adj[u] {
            indeg[v] += 1;
        }
    }
    let mut q: VecDeque<usize> = (0..n).filter(|&u| indeg[u] == 0).collect();
    let mut order = vec![];
    while let Some(u) = q.pop_front() {
        order.push(u);
        for &v in &adj[u] {
            indeg[v] -= 1;
            if indeg[v] == 0 {
                q.push_back(v);
            }
        }
    }
    if order.len() == n { order } else { vec![] }
}

// ---------- 12. Reminders: Kadane + sliding window ----------
fn kadane(a: &[i64]) -> i64 {                    // max subarray sum, at least one element
    let (mut cur, mut best) = (0i64, i64::MIN);
    for &v in a {
        cur = v.max(cur + v);
        best = best.max(cur);
    }
    best
}

fn count_window(a: &[i64], x: i64) -> i64 {      // subarrays with sum == x, ALL values positive
    let (mut left, mut s, mut count) = (0usize, 0i64, 0i64);
    for right in 0..a.len() {
        s += a[right];
        while s > x {
            s -= a[left];
            left += 1;
        }
        if s == x {
            count += 1;
        }
    }
    count
}

// ---------- 13. Interactive I/O (GAME tasks) ----------
// The bot answers only after it gets your move, so: do NOT create Scanner::new()
// (it reads ALL input), read one line at a time, and flush after every move.
fn read_line() -> Option<String> {           // None = the judge closed the input
    let mut s = String::new();
    match io::stdin().read_line(&mut s) {
        Ok(0) | Err(_) => None,
        Ok(_) => Some(s.trim().to_string()),
    }
}

fn say(msg: &str) {
    let mut out = io::stdout().lock();
    writeln!(out, "{}", msg).unwrap();
    out.flush().unwrap();
}
// Example loop:
//   while let Some(msg) = read_line() {
//       if msg == "END" { break; }
//       say(&my_move(&msg));
//   }

// ---------- 14. 2D prefix sums ----------
// p[i][j] = sum of g[0..i][0..j] (exclusive). Rectangle sum, inclusive, 0-indexed.
fn prefix2d(g: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let (n, m) = (g.len(), g[0].len());
    let mut p = vec![vec![0i64; m + 1]; n + 1];
    for i in 0..n {
        for j in 0..m {
            p[i + 1][j + 1] = g[i][j] + p[i + 1][j] + p[i][j + 1] - p[i][j];
        }
    }
    p
}

fn rect_sum(p: &[Vec<i64>], r1: usize, c1: usize, r2: usize, c2: usize) -> i64 {
    p[r2 + 1][c2 + 1] - p[r1][c2 + 1] - p[r2 + 1][c1] + p[r1][c1]
}

// ---------- 15. Iterative DFS + connected components ----------
// Returns (count, comp); comp[v] = component number of v.
fn components(adj: &[Vec<usize>]) -> (usize, Vec<usize>) {
    let n = adj.len();
    let mut comp = vec![usize::MAX; n];
    let mut count = 0;
    for s in 0..n {
        if comp[s] != usize::MAX {
            continue;
        }
        comp[s] = count;
        let mut stack = vec![s];
        while let Some(u) = stack.pop() {
            for &v in &adj[u] {
                if comp[v] == usize::MAX {
                    comp[v] = count;
                    stack.push(v);
                }
            }
        }
        count += 1;
    }
    (count, comp)
}

// ======================= ADVANCED =======================

// ---------- 16. Segment tree (point set, range query) ----------
// Any associative op: SegTree::new(&a, |x, y| x.min(y), i64::MAX)  or  |x, y| x + y, 0
struct SegTree<T: Copy> {
    n: usize,
    t: Vec<T>,
    op: fn(T, T) -> T,
    e: T,
}

impl<T: Copy> SegTree<T> {
    fn new(a: &[T], op: fn(T, T) -> T, e: T) -> Self {
        let n = a.len();
        let mut t = vec![e; 2 * n];
        t[n..].copy_from_slice(a);
        for i in (1..n).rev() {
            t[i] = op(t[2 * i], t[2 * i + 1]);
        }
        SegTree { n, t, op, e }
    }
    fn set(&mut self, i: usize, v: T) {
        let mut i = i + self.n;
        self.t[i] = v;
        while i > 1 {
            i /= 2;
            self.t[i] = (self.op)(self.t[2 * i], self.t[2 * i + 1]);
        }
    }
    fn query(&self, l: usize, r: usize) -> T {  // inclusive
        let (mut resl, mut resr) = (self.e, self.e);
        let (mut l, mut r) = (l + self.n, r + self.n + 1);
        while l < r {
            if l & 1 == 1 {
                resl = (self.op)(resl, self.t[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                resr = (self.op)(self.t[r], resr);
            }
            l /= 2;
            r /= 2;
        }
        (self.op)(resl, resr)
    }
}

// ---------- 17. Lazy segment tree (range add, range sum + range min) ----------
struct LazySeg {
    n: usize,
    sm: Vec<i64>,
    mn: Vec<i64>,
    lz: Vec<i64>,
}

impl LazySeg {
    fn new(a: &[i64]) -> Self {
        let n = a.len();
        let mut s = LazySeg { n, sm: vec![0; 4 * n], mn: vec![0; 4 * n], lz: vec![0; 4 * n] };
        s.build(1, 0, n - 1, a);
        s
    }
    fn build(&mut self, x: usize, l: usize, r: usize, a: &[i64]) {
        if l == r {
            self.sm[x] = a[l];
            self.mn[x] = a[l];
            return;
        }
        let m = (l + r) / 2;
        self.build(2 * x, l, m, a);
        self.build(2 * x + 1, m + 1, r, a);
        self.pull(x);
    }
    fn pull(&mut self, x: usize) {
        self.sm[x] = self.sm[2 * x] + self.sm[2 * x + 1];
        self.mn[x] = self.mn[2 * x].min(self.mn[2 * x + 1]);
    }
    fn apply(&mut self, x: usize, l: usize, r: usize, v: i64) {
        self.sm[x] += v * (r - l + 1) as i64;
        self.mn[x] += v;
        self.lz[x] += v;
    }
    fn push(&mut self, x: usize, l: usize, r: usize) {
        if self.lz[x] != 0 {
            let m = (l + r) / 2;
            let v = self.lz[x];
            self.apply(2 * x, l, m, v);
            self.apply(2 * x + 1, m + 1, r, v);
            self.lz[x] = 0;
        }
    }
    fn add(&mut self, ql: usize, qr: usize, v: i64) {   // a[ql..=qr] += v
        let n = self.n;
        self.add_rec(ql, qr, v, 1, 0, n - 1);
    }
    fn add_rec(&mut self, ql: usize, qr: usize, v: i64, x: usize, l: usize, r: usize) {
        if qr < l || r < ql {
            return;
        }
        if ql <= l && r <= qr {
            self.apply(x, l, r, v);
            return;
        }
        self.push(x, l, r);
        let m = (l + r) / 2;
        self.add_rec(ql, qr, v, 2 * x, l, m);
        self.add_rec(ql, qr, v, 2 * x + 1, m + 1, r);
        self.pull(x);
    }
    fn query(&mut self, ql: usize, qr: usize) -> (i64, i64) {   // (sum, min)
        let n = self.n;
        self.query_rec(ql, qr, 1, 0, n - 1)
    }
    fn query_rec(&mut self, ql: usize, qr: usize, x: usize, l: usize, r: usize) -> (i64, i64) {
        if qr < l || r < ql {
            return (0, i64::MAX);
        }
        if ql <= l && r <= qr {
            return (self.sm[x], self.mn[x]);
        }
        self.push(x, l, r);
        let m = (l + r) / 2;
        let a = self.query_rec(ql, qr, 2 * x, l, m);
        let b = self.query_rec(ql, qr, 2 * x + 1, m + 1, r);
        (a.0 + b.0, a.1.min(b.1))
    }
}

// ---------- 18. Sparse table (static range min, O(1) query) ----------
struct SparseMin {
    t: Vec<Vec<i64>>,
}

impl SparseMin {
    fn new(a: &[i64]) -> Self {
        let mut t = vec![a.to_vec()];
        let mut j = 1;
        while (1 << j) <= a.len() {
            let h = 1 << (j - 1);
            let row: Vec<i64> = (0..=a.len() - (1 << j)).map(|i| t[j - 1][i].min(t[j - 1][i + h])).collect();
            t.push(row);
            j += 1;
        }
        SparseMin { t }
    }
    fn query(&self, l: usize, r: usize) -> i64 {      // inclusive
        let k = (usize::BITS - 1 - (r - l + 1).leading_zeros()) as usize;
        self.t[k][l].min(self.t[k][r + 1 - (1 << k)])
    }
}

// ---------- 19. LCA (binary lifting) + tree distance ----------
struct Lca {
    log: usize,
    depth: Vec<usize>,
    up: Vec<Vec<usize>>,
}

impl Lca {
    fn new(adj: &[Vec<usize>], root: usize) -> Self {
        let n = adj.len();
        let mut log = 1;
        while (1 << log) < n {
            log += 1;
        }
        let mut depth = vec![0; n];
        let mut up = vec![vec![root; n]; log];
        let mut seen = vec![false; n];
        let mut q = VecDeque::new();
        seen[root] = true;
        q.push_back(root);
        while let Some(u) = q.pop_front() {
            for &v in &adj[u] {
                if !seen[v] {
                    seen[v] = true;
                    depth[v] = depth[u] + 1;
                    up[0][v] = u;
                    q.push_back(v);
                }
            }
        }
        for k in 1..log {
            for v in 0..n {
                up[k][v] = up[k - 1][up[k - 1][v]];
            }
        }
        Lca { log, depth, up }
    }
    fn lca(&self, mut a: usize, mut b: usize) -> usize {
        if self.depth[a] < self.depth[b] {
            std::mem::swap(&mut a, &mut b);
        }
        let mut diff = self.depth[a] - self.depth[b];
        let mut k = 0;
        while diff > 0 {
            if diff & 1 == 1 {
                a = self.up[k][a];
            }
            diff >>= 1;
            k += 1;
        }
        if a == b {
            return a;
        }
        for k in (0..self.log).rev() {
            if self.up[k][a] != self.up[k][b] {
                a = self.up[k][a];
                b = self.up[k][b];
            }
        }
        self.up[0][a]
    }
    fn dist(&self, a: usize, b: usize) -> usize {
        self.depth[a] + self.depth[b] - 2 * self.depth[self.lca(a, b)]
    }
}

// ---------- 20. KMP prefix function + pattern search ----------
fn prefix_function(s: &[u8]) -> Vec<usize> {
    let mut pi = vec![0; s.len()];
    for i in 1..s.len() {
        let mut k = pi[i - 1];
        while k > 0 && s[i] != s[k] {
            k = pi[k - 1];
        }
        if s[i] == s[k] {
            k += 1;
        }
        pi[i] = k;
    }
    pi
}

fn find_all(text: &[u8], pat: &[u8]) -> Vec<usize> {   // start indices (use .as_bytes())
    let pi = prefix_function(pat);
    let (mut res, mut k) = (vec![], 0);
    for i in 0..text.len() {
        while k > 0 && text[i] != pat[k] {
            k = pi[k - 1];
        }
        if text[i] == pat[k] {
            k += 1;
        }
        if k == pat.len() {
            res.push(i + 1 - k);
            k = pi[k - 1];
        }
    }
    res
}

// ---------- 21. Z-function ----------
// z[i] = length of the longest common prefix of s and s[i..]  (z[0] = n)
fn z_function(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0; n];
    if n > 0 {
        z[0] = n;
    }
    let (mut l, mut r) = (0, 0);
    for i in 1..n {
        if i < r {
            z[i] = (r - i).min(z[i - l]);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    z
}

// ---------- 22. String hashing (double mod, random base) ----------
// h.get(l, r) == h.get(l2, r2)  ->  substrings equal (inclusive)
struct StrHash {
    h1: Vec<i64>,
    h2: Vec<i64>,
    p1: Vec<i64>,
    p2: Vec<i64>,
}

impl StrHash {
    const M1: i64 = 1_000_000_007;
    const M2: i64 = 998_244_353;
    fn new(s: &[u8]) -> Self {
        let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().subsec_nanos() as i64;
        let b = 256 + nanos % 100_000_000;
        let n = s.len();
        let (mut h1, mut h2, mut p1, mut p2) = (vec![0i64; n + 1], vec![0i64; n + 1], vec![1i64; n + 1], vec![1i64; n + 1]);
        for i in 0..n {
            let c = s[i] as i64;
            h1[i + 1] = (h1[i] * b + c) % Self::M1;
            h2[i + 1] = (h2[i] * b + c) % Self::M2;
            p1[i + 1] = p1[i] * b % Self::M1;
            p2[i + 1] = p2[i] * b % Self::M2;
        }
        StrHash { h1, h2, p1, p2 }
    }
    fn get(&self, l: usize, r: usize) -> (i64, i64) {   // inclusive
        let r = r + 1;
        (
            ((self.h1[r] - self.h1[l] * self.p1[r - l]) % Self::M1 + Self::M1) % Self::M1,
            ((self.h2[r] - self.h2[l] * self.p2[r - l]) % Self::M2 + Self::M2) % Self::M2,
        )
    }
}

// ---------- 23. Matrix power (mod MOD) ----------
type Mat = Vec<Vec<i64>>;

fn mat_mul(a: &Mat, b: &Mat) -> Mat {
    let (n, m, p) = (a.len(), b.len(), b[0].len());
    let mut c = vec![vec![0i64; p]; n];
    for i in 0..n {
        for k in 0..m {
            if a[i][k] != 0 {
                for j in 0..p {
                    c[i][j] = (c[i][j] + a[i][k] * b[k][j]) % MOD;
                }
            }
        }
    }
    c
}

fn mat_pow(mut a: Mat, mut e: u64) -> Mat {
    let n = a.len();
    let mut r: Mat = (0..n).map(|i| (0..n).map(|j| (i == j) as i64).collect()).collect();
    while e > 0 {
        if e & 1 == 1 {
            r = mat_mul(&r, &a);
        }
        a = mat_mul(&a, &a);
        e >>= 1;
    }
    r
}
// Fibonacci: F(n) = mat_pow(vec![vec![1, 1], vec![1, 0]], n)[0][1]

// ---------- 24. Floyd-Warshall (all pairs, n <= ~400) ----------
// d[i][i] = 0, INF where no edge. Changed in place.
fn floyd(d: &mut Vec<Vec<i64>>) {
    let n = d.len();
    for k in 0..n {
        for i in 0..n {
            if d[i][k] >= INF {
                continue;
            }
            for j in 0..n {
                if d[k][j] < INF && d[i][k] + d[k][j] < d[i][j] {
                    d[i][j] = d[i][k] + d[k][j];
                }
            }
        }
    }
}

// ---------- 25. Kruskal MST (uses DSU, section 7) ----------
// edges: (w, u, v). Returns the total weight (fewer than n-1 unions = not connected).
fn kruskal(n: usize, mut edges: Vec<(i64, usize, usize)>) -> i64 {
    edges.sort();
    let mut ds = Dsu::new(n);
    let mut total = 0;
    for (w, u, v) in edges {
        if ds.union(u, v) {
            total += w;
        }
    }
    total
}

// ---------- 26. SCC (Kosaraju, iterative) ----------
// Returns (count, comp); components numbered in topological order of the condensed graph.
fn scc(adj: &[Vec<usize>]) -> (usize, Vec<usize>) {
    let n = adj.len();
    let mut radj = vec![vec![]; n];
    for u in 0..n {
        for &v in &adj[u] {
            radj[v].push(u);
        }
    }
    let mut order = vec![];
    let mut seen = vec![false; n];
    for s in 0..n {
        if seen[s] {
            continue;
        }
        seen[s] = true;
        let mut st = vec![(s, 0usize)];
        while let Some(&(u, i)) = st.last() {
            if i < adj[u].len() {
                st.last_mut().unwrap().1 += 1;
                let v = adj[u][i];
                if !seen[v] {
                    seen[v] = true;
                    st.push((v, 0));
                }
            } else {
                order.push(u);
                st.pop();
            }
        }
    }
    let mut comp = vec![usize::MAX; n];
    let mut count = 0;
    for &s in order.iter().rev() {
        if comp[s] != usize::MAX {
            continue;
        }
        comp[s] = count;
        let mut st = vec![s];
        while let Some(u) = st.pop() {
            for &v in &radj[u] {
                if comp[v] == usize::MAX {
                    comp[v] = count;
                    st.push(v);
                }
            }
        }
        count += 1;
    }
    (count, comp)
}

// ---------- 27. Max flow (Dinic) ----------
// let mut f = Dinic::new(n); f.add_edge(u, v, cap); f.max_flow(s, t)
// Bipartite matching = source -> left (cap 1), left -> right (1), right -> sink (1).
struct Dinic {
    to: Vec<usize>,
    cap: Vec<i64>,
    g: Vec<Vec<usize>>,   // edge indices; edge e's reverse is e ^ 1
    level: Vec<i64>,
    it: Vec<usize>,
}

impl Dinic {
    fn new(n: usize) -> Self {
        Dinic { to: vec![], cap: vec![], g: vec![vec![]; n], level: vec![0; n], it: vec![0; n] }
    }
    fn add_edge(&mut self, u: usize, v: usize, c: i64) {
        self.g[u].push(self.to.len());
        self.to.push(v);
        self.cap.push(c);
        self.g[v].push(self.to.len());
        self.to.push(u);
        self.cap.push(0);
    }
    fn bfs(&mut self, s: usize, t: usize) -> bool {
        self.level.iter_mut().for_each(|x| *x = -1);
        let mut q = VecDeque::new();
        self.level[s] = 0;
        q.push_back(s);
        while let Some(u) = q.pop_front() {
            for &e in &self.g[u] {
                let v = self.to[e];
                if self.cap[e] > 0 && self.level[v] < 0 {
                    self.level[v] = self.level[u] + 1;
                    q.push_back(v);
                }
            }
        }
        self.level[t] >= 0
    }
    fn dfs(&mut self, u: usize, t: usize, f: i64) -> i64 {
        if u == t {
            return f;
        }
        while self.it[u] < self.g[u].len() {
            let e = self.g[u][self.it[u]];
            let v = self.to[e];
            if self.cap[e] > 0 && self.level[v] == self.level[u] + 1 {
                let d = self.dfs(v, t, f.min(self.cap[e]));
                if d > 0 {
                    self.cap[e] -= d;
                    self.cap[e ^ 1] += d;
                    return d;
                }
            }
            self.it[u] += 1;
        }
        0
    }
    fn max_flow(&mut self, s: usize, t: usize) -> i64 {
        let mut flow = 0;
        while self.bfs(s, t) {
            self.it.iter_mut().for_each(|x| *x = 0);
            loop {
                let f = self.dfs(s, t, i64::MAX);
                if f == 0 {
                    break;
                }
                flow += f;
            }
        }
        flow
    }
}

// ---------- 28. Geometry: cross, convex hull, polygon area ----------
type Pt = (i64, i64);

fn cross(o: Pt, a: Pt, b: Pt) -> i64 {           // > 0: o->a->b turns left (counter-clockwise)
    (a.0 - o.0) * (b.1 - o.1) - (a.1 - o.1) * (b.0 - o.0)
}

fn convex_hull(mut pts: Vec<Pt>) -> Vec<Pt> {    // counter-clockwise, no collinear points
    pts.sort();
    pts.dedup();
    if pts.len() <= 2 {
        return pts;
    }
    let mut h: Vec<Pt> = vec![];
    for &p in &pts {
        while h.len() >= 2 && cross(h[h.len() - 2], h[h.len() - 1], p) <= 0 {
            h.pop();
        }
        h.push(p);
    }
    let lower = h.len() + 1;
    for &p in pts.iter().rev().skip(1) {
        while h.len() >= lower && cross(h[h.len() - 2], h[h.len() - 1], p) <= 0 {
            h.pop();
        }
        h.push(p);
    }
    h.pop();
    h
}

fn area2(poly: &[Pt]) -> i64 {                   // TWICE the area (stays an integer)
    let mut s = 0;
    for i in 0..poly.len() {
        let (a, b) = (poly[i], poly[(i + 1) % poly.len()]);
        s += a.0 * b.1 - b.0 * a.1;
    }
    s.abs()
}

// ---------- 29. LIS (strictly increasing), O(n log n) ----------
// For non-decreasing, change `x < v` to `x <= v`.
fn lis_length(a: &[i64]) -> usize {
    let mut tails: Vec<i64> = vec![];
    for &v in a {
        let i = tails.partition_point(|&x| x < v);
        if i == tails.len() {
            tails.push(v);
        } else {
            tails[i] = v;
        }
    }
    tails.len()
}

// ---------- 30. Factorization (smallest prime factor) + extended gcd ----------
fn spf_sieve(n: usize) -> Vec<usize> {           // spf[x] = smallest prime factor of x
    let mut spf: Vec<usize> = (0..=n).collect();
    let mut i = 2;
    while i * i <= n {
        if spf[i] == i {
            let mut j = i * i;
            while j <= n {
                if spf[j] == j {
                    spf[j] = i;
                }
                j += i;
            }
        }
        i += 1;
    }
    spf
}

fn factorize(mut x: usize, spf: &[usize]) -> Vec<usize> {   // primes with repetition, ascending
    let mut res = vec![];
    while x > 1 {
        res.push(spf[x]);
        x /= spf[x];
    }
    res
}

fn ext_gcd(mut a: i64, mut b: i64) -> (i64, i64, i64) {     // (g, x, y) with a*x + b*y = g
    let (mut x0, mut y0, mut x1, mut y1) = (1i64, 0i64, 0i64, 1i64);
    while b != 0 {
        let q = a / b;
        let t = a - q * b; a = b; b = t;
        let t = x0 - q * x1; x0 = x1; x1 = t;
        let t = y0 - q * y1; y0 = y1; y1 = t;
    }
    (a, x0, y0)
}

// ======================= MORE =======================

// ---------- 31. NTT polynomial multiply (mod 998244353) ----------
// Coefficients mod 998244353. For exact (non-mod) products the true values must stay below it.
const NTT_MOD: i64 = 998_244_353;

fn powm(mut a: i64, mut e: i64, m: i64) -> i64 {
    let mut r = 1i64;
    a = ((a % m) + m) % m;
    while e > 0 {
        if e & 1 == 1 {
            r = r * a % m;
        }
        a = a * a % m;
        e >>= 1;
    }
    r
}

fn ntt(a: &mut Vec<i64>, invert: bool) {
    let n = a.len();
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            a.swap(i, j);
        }
    }
    let mut len = 2;
    while len <= n {
        let mut w = powm(3, (NTT_MOD - 1) / len as i64, NTT_MOD);
        if invert {
            w = powm(w, NTT_MOD - 2, NTT_MOD);
        }
        let half = len / 2;
        let mut ws = vec![1i64; half];
        for k in 1..half {
            ws[k] = ws[k - 1] * w % NTT_MOD;
        }
        for i in (0..n).step_by(len) {
            for k in 0..half {
                let u = a[i + k];
                let v = a[i + k + half] * ws[k] % NTT_MOD;
                a[i + k] = (u + v) % NTT_MOD;
                a[i + k + half] = (u - v + NTT_MOD) % NTT_MOD;
            }
        }
        len <<= 1;
    }
    if invert {
        let inv_n = powm(n as i64, NTT_MOD - 2, NTT_MOD);
        for x in a.iter_mut() {
            *x = *x * inv_n % NTT_MOD;
        }
    }
}

fn ntt_multiply(a: &[i64], b: &[i64]) -> Vec<i64> {   // lowest degree first
    if a.is_empty() || b.is_empty() {
        return vec![];
    }
    let need = a.len() + b.len() - 1;
    let mut n = 1;
    while n < need {
        n <<= 1;
    }
    let mut fa: Vec<i64> = a.iter().map(|&x| ((x % NTT_MOD) + NTT_MOD) % NTT_MOD).collect();
    let mut fb: Vec<i64> = b.iter().map(|&x| ((x % NTT_MOD) + NTT_MOD) % NTT_MOD).collect();
    fa.resize(n, 0);
    fb.resize(n, 0);
    ntt(&mut fa, false);
    ntt(&mut fb, false);
    for i in 0..n {
        fa[i] = fa[i] * fb[i] % NTT_MOD;
    }
    ntt(&mut fa, true);
    fa.truncate(need);
    fa
}

// ---------- 32. Bellman-Ford + negative cycle ----------
// edges: (u, v, w), weights may be negative. Returns (dist, negative cycle reachable from s).
fn bellman_ford(n: usize, edges: &[(usize, usize, i64)], s: usize) -> (Vec<i64>, bool) {
    let mut dist = vec![INF; n];
    dist[s] = 0;
    for _ in 0..n.saturating_sub(1) {
        let mut changed = false;
        for &(u, v, w) in edges {
            if dist[u] < INF && dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
                changed = true;
            }
        }
        if !changed {
            return (dist, false);
        }
    }
    let neg = edges.iter().any(|&(u, v, w)| dist[u] < INF && dist[u] + w < dist[v]);
    (dist, neg)
}

// ---------- 33. Bridges + articulation points (undirected, iterative) ----------
// edges: (u, v); parallel edges are handled. Returns (bridge edge indices, cut vertices), sorted.
fn bridges_and_cuts(n: usize, edges: &[(usize, usize)]) -> (Vec<usize>, Vec<usize>) {
    let mut adj = vec![vec![]; n];
    for (i, &(u, v)) in edges.iter().enumerate() {
        adj[u].push((v, i));
        adj[v].push((u, i));
    }
    let mut tin = vec![usize::MAX; n];
    let mut low = vec![0usize; n];
    let mut is_cut = vec![false; n];
    let mut bridges = vec![];
    let mut timer = 0;
    for root in 0..n {
        if tin[root] != usize::MAX {
            continue;
        }
        tin[root] = timer;
        low[root] = timer;
        timer += 1;
        let mut children = 0;
        let mut st: Vec<(usize, usize, usize)> = vec![(root, usize::MAX, 0)];   // node, parent edge, next index
        while let Some(&(u, pe, i)) = st.last() {
            if i < adj[u].len() {
                st.last_mut().unwrap().2 += 1;
                let (v, eid) = adj[u][i];
                if eid == pe {
                    continue;
                }
                if tin[v] == usize::MAX {
                    tin[v] = timer;
                    low[v] = timer;
                    timer += 1;
                    st.push((v, eid, 0));
                    if u == root {
                        children += 1;
                    }
                } else {
                    low[u] = low[u].min(tin[v]);
                }
            } else {
                st.pop();
                if let Some(&(p, _, _)) = st.last() {
                    low[p] = low[p].min(low[u]);
                    if low[u] > tin[p] {
                        bridges.push(pe);
                    }
                    if p != root && low[u] >= tin[p] {
                        is_cut[p] = true;
                    }
                }
            }
        }
        if children > 1 {
            is_cut[root] = true;
        }
    }
    bridges.sort();
    (bridges, (0..n).filter(|&v| is_cut[v]).collect())
}

// ---------- 34. Trie (prefix counting, lowercase b'a'..b'z') ----------
struct Trie {
    nxt: Vec<[usize; 26]>,
    pass: Vec<usize>,
    end: Vec<usize>,
}

impl Trie {
    fn new() -> Self {
        Trie { nxt: vec![[usize::MAX; 26]], pass: vec![0], end: vec![0] }
    }
    fn insert(&mut self, w: &[u8]) {
        let mut x = 0;
        for &ch in w {
            let c = (ch - b'a') as usize;
            if self.nxt[x][c] == usize::MAX {
                self.nxt[x][c] = self.nxt.len();
                self.nxt.push([usize::MAX; 26]);
                self.pass.push(0);
                self.end.push(0);
            }
            x = self.nxt[x][c];
            self.pass[x] += 1;
        }
        self.end[x] += 1;
    }
    fn walk(&self, s: &[u8]) -> Option<usize> {
        let mut x = 0;
        for &ch in s {
            x = self.nxt[x][(ch - b'a') as usize];
            if x == usize::MAX {
                return None;
            }
        }
        Some(x)
    }
    fn count_prefix(&self, p: &[u8]) -> usize {   // inserted words starting with p
        if p.is_empty() {
            return self.end.iter().sum();
        }
        self.walk(p).map_or(0, |x| self.pass[x])
    }
    fn count_word(&self, w: &[u8]) -> usize {
        self.walk(w).map_or(0, |x| self.end[x])
    }
}

// ---------- 35. Coordinate compression ----------
// Returns (ranks, values): ranks[i] = index of a[i] among the sorted distinct values.
fn compress(a: &[i64]) -> (Vec<usize>, Vec<i64>) {
    let mut values = a.to_vec();
    values.sort();
    values.dedup();
    let ranks = a.iter().map(|x| values.binary_search(x).unwrap()).collect();
    (ranks, values)
}

// ---------- 36. Ternary search (integer argmax of a unimodal f) ----------
// f must strictly increase then strictly decrease. For a minimum, negate f.
fn ternary_max(mut lo: i64, mut hi: i64, f: impl Fn(i64) -> i64) -> i64 {
    while hi - lo > 2 {
        let m1 = lo + (hi - lo) / 3;
        let m2 = hi - (hi - lo) / 3;
        if f(m1) < f(m2) {
            lo = m1 + 1;
        } else {
            hi = m2 - 1;
        }
    }
    let mut best = lo;
    for x in lo + 1..=hi {
        if f(x) > f(best) {
            best = x;
        }
    }
    best
}
// Real numbers: repeat 100 times: m1, m2 at thirds; if f(m1) < f(m2) { lo = m1 } else { hi = m2 }

// ---------- 37. Miller-Rabin primality (deterministic up to 2^64) ----------
fn mulmod(a: u64, b: u64, m: u64) -> u64 {
    (a as u128 * b as u128 % m as u128) as u64
}

fn powmod(mut a: u64, mut e: u64, m: u64) -> u64 {
    let mut r = 1u64;
    a %= m;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, a, m);
        }
        a = mulmod(a, a, m);
        e >>= 1;
    }
    r
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    const BASES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    for &p in &BASES {
        if n % p == 0 {
            return n == p;
        }
    }
    let (mut d, mut r) = (n - 1, 0);
    while d % 2 == 0 {
        d /= 2;
        r += 1;
    }
    'outer: for &a in &BASES {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 0..r - 1 {
            x = mulmod(x, x, n);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

// ---------- 38. CRT (any moduli) + Euler totient ----------
// x = a1 (mod m1), x = a2 (mod m2). Returns Some((x, lcm)) or None. Uses ext_gcd (30).
fn crt(a1: i64, m1: i64, a2: i64, m2: i64) -> Option<(i64, i64)> {
    let (g, p, _) = ext_gcd(m1, m2);
    if (a2 - a1) % g != 0 {
        return None;
    }
    let m = (m2 / g) as i128;
    let t = (((a2 - a1) / g) as i128 % m * (p as i128 % m) % m + m) % m;
    let l = (m1 / g) as i128 * m2 as i128;
    let x = ((a1 as i128 + m1 as i128 * t) % l + l) % l;
    Some((x as i64, l as i64))
}

fn totient(mut n: i64) -> i64 {                  // count of 1..n coprime with n, O(sqrt n)
    let mut result = n;
    let mut p = 2;
    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    if n > 1 {
        result -= result / n;
    }
    result
}

// ---------- 39. Segment intersection + point in polygon (integer points) ----------
// Uses Pt and cross (28).
fn on_segment(p: Pt, a: Pt, b: Pt) -> bool {
    cross(a, b, p) == 0
        && a.0.min(b.0) <= p.0 && p.0 <= a.0.max(b.0)
        && a.1.min(b.1) <= p.1 && p.1 <= a.1.max(b.1)
}

fn segments_intersect(p1: Pt, p2: Pt, p3: Pt, p4: Pt) -> bool {   // touching counts
    let (d1, d2) = (cross(p3, p4, p1), cross(p3, p4, p2));
    let (d3, d4) = (cross(p1, p2, p3), cross(p1, p2, p4));
    if ((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0)) && ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0)) {
        return true;
    }
    on_segment(p1, p3, p4) || on_segment(p2, p3, p4) || on_segment(p3, p1, p2) || on_segment(p4, p1, p2)
}

fn point_in_polygon(p: Pt, poly: &[Pt]) -> i32 {  // 1 inside, 0 boundary, -1 outside
    let mut inside = false;
    for i in 0..poly.len() {
        let (a, b) = (poly[i], poly[(i + 1) % poly.len()]);
        if on_segment(p, a, b) {
            return 0;
        }
        if (a.1 > p.1) != (b.1 > p.1) && (cross(a, b, p) > 0) == (b.1 > a.1) {
            inside = !inside;
        }
    }
    if inside { 1 } else { -1 }
}

// ---------- 40. Manacher (longest palindrome) ----------
// Returns (start, length) of a longest palindromic substring, in O(n).
fn manacher(s: &[u8]) -> (usize, usize) {
    let n = s.len();
    if n == 0 {
        return (0, 0);
    }
    let (mut d1, mut d2) = (vec![0usize; n], vec![0usize; n]);
    let (mut l, mut r): (i64, i64) = (0, -1);
    for i in 0..n {
        let ii = i as i64;
        let mut k = if ii > r { 1 } else { d1[(l + r - ii) as usize].min((r - ii + 1) as usize) };
        while i >= k && i + k < n && s[i - k] == s[i + k] {
            k += 1;
        }
        d1[i] = k;
        if ii + k as i64 - 1 > r {
            l = ii - k as i64 + 1;
            r = ii + k as i64 - 1;
        }
    }
    let (mut l, mut r): (i64, i64) = (0, -1);
    for i in 0..n {
        let ii = i as i64;
        let mut k = if ii > r { 0 } else { d2[(l + r - ii + 1) as usize].min((r - ii + 1) as usize) };
        while i >= k + 1 && i + k < n && s[i - k - 1] == s[i + k] {
            k += 1;
        }
        d2[i] = k;
        if ii + k as i64 - 1 > r {
            l = ii - k as i64;
            r = ii + k as i64 - 1;
        }
    }
    let mut best = (0, 1);
    for i in 0..n {
        if 2 * d1[i] - 1 > best.1 {
            best = (i + 1 - d1[i], 2 * d1[i] - 1);
        }
        if 2 * d2[i] > best.1 {
            best = (i - d2[i], 2 * d2[i]);
        }
    }
    best
}

// ---------- Solution ----------
fn main() {
    // (For GAME tasks, delete the Scanner and BufWriter lines and use section 13.)
    let mut sc = Scanner::new();
    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout.lock()); // writeln!(out, "{}", x).unwrap();

    let n: usize = sc.next();
    let a: Vec<i64> = (0..n).map(|_| sc.next()).collect();

    // solve here
}
