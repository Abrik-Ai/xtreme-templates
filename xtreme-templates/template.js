// IEEEXtreme 20.0 - team template (JavaScript / Node.js)
// Number is exact only up to 2^53 (about 9e15). Use BigInt (123n) above that.
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
//  32  Bellman-Ford + negative cycle            37  Miller-Rabin primality (BigInt, up to 2^64)
//  33  Bridges + articulation points            38  CRT (any moduli, BigInt) + Euler totient
//  34  Trie (prefix counting)                   39  Segment intersection + point in polygon
//  35  Coordinate compression                   40  Manacher (longest palindrome)
//   +  Queue and MinHeap classes (JS has neither built in)
'use strict';

// ---------- 1. Fast input ----------
// Numbers / words separated by whitespace. Input is read on the first call only,
// so GAME tasks (section 13) still work as long as you never call next().
let data = null, idx = 0;
function next() {                        // next token as a string
  if (data === null) data = require('fs').readFileSync(0, 'utf8').split(/\s+/).filter(s => s.length);
  return data[idx++];
}
const nextInt = () => Number(next());
// Text lines (grids, sentences): use this instead
//   const lines = require('fs').readFileSync(0, 'utf8').split('\n');

// ---------- 2. Prefix sums ----------
// p[i] = a[0] + ... + a[i-1];  sum of a[l..r] (0-indexed, inclusive) = p[r+1] - p[l]
function prefix(a) {
  const p = new Array(a.length + 1);
  p[0] = 0;
  for (let i = 0; i < a.length; i++) p[i + 1] = p[i] + a[i];
  return p;
}

// ---------- 3. Binary search on the answer ----------
// Smallest x in [lo, hi] with ok(x) true. ok must be F F F ... T T T.
function firstTrue(lo, hi, ok) {
  while (lo < hi) {
    const mid = Math.floor((lo + hi) / 2);
    if (ok(mid)) hi = mid;
    else lo = mid + 1;
  }
  return lo;
}

// ---------- Queue (array + head index; Array.shift() is O(n)) ----------
class Queue {
  constructor() { this.a = []; this.h = 0; }
  push(x) { this.a.push(x); }
  pop() { return this.a[this.h++]; }
  get size() { return this.a.length - this.h; }
}

// ---------- Min-heap of [key, value] pairs ----------
class MinHeap {
  constructor() { this.h = []; }
  get size() { return this.h.length; }
  push(item) {
    const h = this.h;
    h.push(item);
    let i = h.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (h[p][0] <= h[i][0]) break;
      [h[p], h[i]] = [h[i], h[p]];
      i = p;
    }
  }
  pop() {
    const h = this.h;
    const top = h[0];
    const last = h.pop();
    if (h.length > 0) {
      h[0] = last;
      let i = 0;
      for (;;) {
        const l = 2 * i + 1, r = l + 1;
        let s = i;
        if (l < h.length && h[l][0] < h[s][0]) s = l;
        if (r < h.length && h[r][0] < h[s][0]) s = r;
        if (s === i) break;
        [h[s], h[i]] = [h[i], h[s]];
        i = s;
      }
    }
    return top;
  }
}

// ---------- 4. BFS on a grid ----------
// Distances from (si, sj) over cells that are not '#'. -1 = unreachable.
const DI = [-1, 1, 0, 0], DJ = [0, 0, -1, 1];
function bfsGrid(grid, si, sj) {
  const n = grid.length, m = grid[0].length;
  const dist = Array.from({ length: n }, () => new Array(m).fill(-1));
  const q = new Queue();
  dist[si][sj] = 0;
  q.push([si, sj]);
  while (q.size > 0) {
    const [i, j] = q.pop();
    for (let d = 0; d < 4; d++) {
      const ni = i + DI[d], nj = j + DJ[d];
      if (ni >= 0 && ni < n && nj >= 0 && nj < m && grid[ni][nj] !== '#' && dist[ni][nj] === -1) {
        dist[ni][nj] = dist[i][j] + 1;
        q.push([ni, nj]);
      }
    }
  }
  return dist;
}

// ---------- 5. BFS on a graph (unweighted shortest path) ----------
// adj: array of neighbour arrays, nodes 0..n-1
function bfsGraph(adj, s) {
  const n = adj.length;
  const dist = new Array(n).fill(-1), par = new Array(n).fill(-1);
  const q = new Queue();
  dist[s] = 0;
  q.push(s);
  while (q.size > 0) {
    const u = q.pop();
    for (const v of adj[u]) {
      if (dist[v] === -1) {
        dist[v] = dist[u] + 1;
        par[v] = u;
        q.push(v);
      }
    }
  }
  return { dist, par };
}

function pathTo(par, t) {
  const path = [];
  for (; t !== -1; t = par[t]) path.push(t);
  return path.reverse();
}

// ---------- 6. Dijkstra (non-negative weights) ----------
// adj[u] = array of [v, w]
function dijkstra(adj, s) {
  const dist = new Array(adj.length).fill(Infinity);
  const pq = new MinHeap();
  dist[s] = 0;
  pq.push([0, s]);
  while (pq.size > 0) {
    const [d, u] = pq.pop();
    if (d > dist[u]) continue; // old entry
    for (const [v, w] of adj[u]) {
      if (d + w < dist[v]) {
        dist[v] = d + w;
        pq.push([dist[v], v]);
      }
    }
  }
  return dist;
}

// ---------- 7. DSU / union-find ----------
class DSU {
  constructor(n) {
    this.p = Array.from({ length: n }, (_, i) => i);
    this.sz = new Array(n).fill(1);
  }
  find(x) {
    while (this.p[x] !== x) { this.p[x] = this.p[this.p[x]]; x = this.p[x]; }
    return x;
  }
  union(a, b) {
    a = this.find(a); b = this.find(b);
    if (a === b) return false;
    if (this.sz[a] < this.sz[b]) [a, b] = [b, a];
    this.p[b] = a;
    this.sz[a] += this.sz[b];
    return true;
  }
}

// ---------- 8. Modular math (MOD must be prime for the inverse) ----------
// a * b can reach ~1e18, beyond Number's exact range, so multiply with BigInt.
const MOD = 1000000007;
const MODn = 1000000007n;
const mulMod = (a, b) => Number((BigInt(a) * BigInt(b)) % MODn);

function power(a, b) {
  let r = 1n, x = ((BigInt(a) % MODn) + MODn) % MODn, e = BigInt(b);
  while (e > 0n) {
    if (e & 1n) r = r * x % MODn;
    x = x * x % MODn;
    e >>= 1n;
  }
  return Number(r);
}
const inverse = a => power(a, MOD - 2);

let fact = [], invFact = [];
function buildFactorials(n) {
  fact = new Array(n + 1).fill(1);
  invFact = new Array(n + 1).fill(1);
  for (let i = 1; i <= n; i++) fact[i] = mulMod(fact[i - 1], i);
  invFact[n] = inverse(fact[n]);
  for (let i = n; i > 0; i--) invFact[i - 1] = mulMod(invFact[i], i);
}
function ncr(n, r) {
  if (r < 0 || r > n) return 0;
  return mulMod(mulMod(fact[n], invFact[r]), invFact[n - r]);
}

// ---------- 9. Sieve + gcd / lcm ----------
function sieve(n) {
  const isPrime = new Uint8Array(n + 1).fill(1);
  isPrime[0] = 0;
  if (n >= 1) isPrime[1] = 0;
  for (let i = 2; i * i <= n; i++)
    if (isPrime[i])
      for (let j = i * i; j <= n; j += i) isPrime[j] = 0;
  return isPrime;                       // 1 = prime
}
function gcd(a, b) { while (b) [a, b] = [b, a % b]; return a; }
const lcm = (a, b) => a / gcd(a, b) * b;  // use BigInt if it can pass 9e15

// ---------- 10. Fenwick tree (range sums WITH updates) ----------
// 0-indexed outside, 1-indexed inside.
class Fenwick {
  constructor(n) { this.n = n; this.t = new Array(n + 1).fill(0); }
  add(i, delta) {                       // a[i] += delta
    for (i++; i <= this.n; i += i & -i) this.t[i] += delta;
  }
  prefixSum(i) {                        // a[0] + ... + a[i-1]
    let s = 0;
    for (; i > 0; i -= i & -i) s += this.t[i];
    return s;
  }
  rangeSum(l, r) { return this.prefixSum(r + 1) - this.prefixSum(l); }  // inclusive
}

// ---------- 11. Topological sort (Kahn) ----------
// adj[u] = nodes that must come AFTER u. Returns an order, or [] if there is a cycle.
function topoSort(adj) {
  const n = adj.length;
  const indeg = new Array(n).fill(0);
  for (let u = 0; u < n; u++) for (const v of adj[u]) indeg[v]++;
  const q = new Queue();
  for (let u = 0; u < n; u++) if (indeg[u] === 0) q.push(u);
  const order = [];
  while (q.size > 0) {
    const u = q.pop();
    order.push(u);
    for (const v of adj[u]) if (--indeg[v] === 0) q.push(v);
  }
  return order.length === n ? order : [];
}

// ---------- 12. Reminders: Kadane + sliding window ----------
function kadane(a) {                    // max subarray sum, at least one element
  let cur = 0, best = -Infinity;
  for (const v of a) {
    cur = Math.max(v, cur + v);
    best = Math.max(best, cur);
  }
  return best;
}

function countWindow(a, x) {            // subarrays with sum == x, ALL values positive
  let left = 0, s = 0, count = 0;
  for (let right = 0; right < a.length; right++) {
    s += a[right];
    while (s > x) s -= a[left++];
    if (s === x) count++;
  }
  return count;
}

// ---------- 13. Interactive I/O (GAME tasks) ----------
// The bot answers only after it gets your move: never call next() (it reads ALL input),
// read one line at a time with `await readLine()`, and print each move right away.
let _lines = null;
async function readLine() {              // null = the judge closed the input
  if (_lines === null) _lines = require('readline').createInterface({ input: process.stdin })[Symbol.asyncIterator]();
  const r = await _lines.next();
  return r.done ? null : r.value.trim();
}
function say(...parts) { process.stdout.write(parts.join(' ') + '\n'); }
// Example (main must be async):
//   async function game() {
//     for (;;) {
//       const msg = await readLine();
//       if (msg === null || msg === 'END') break;
//       say(myMove(msg));
//     }
//     process.exit(0);
//   }
//   game();

// ---------- 14. 2D prefix sums ----------
// P[i][j] = sum of g[0..i-1][0..j-1]. Rectangle sum, inclusive, 0-indexed.
function prefix2d(g) {
  const n = g.length, m = g[0].length;
  const P = Array.from({ length: n + 1 }, () => new Array(m + 1).fill(0));
  for (let i = 0; i < n; i++)
    for (let j = 0; j < m; j++)
      P[i + 1][j + 1] = g[i][j] + P[i + 1][j] + P[i][j + 1] - P[i][j];
  return P;
}
const rectSum = (P, r1, c1, r2, c2) => P[r2 + 1][c2 + 1] - P[r1][c2 + 1] - P[r2 + 1][c1] + P[r1][c1];
// Char grid (count '*'): prefix2d(grid.map(row => [...row].map(ch => ch === '*' ? 1 : 0)))

// ---------- 15. Iterative DFS + connected components ----------
// Returns { count, comp }; comp[v] = component number of v.
function components(adj) {
  const n = adj.length;
  const comp = new Array(n).fill(-1);
  let count = 0;
  for (let s = 0; s < n; s++) {
    if (comp[s] !== -1) continue;
    comp[s] = count;
    const stack = [s];
    while (stack.length > 0) {
      const u = stack.pop();
      for (const v of adj[u])
        if (comp[v] === -1) { comp[v] = count; stack.push(v); }
    }
    count++;
  }
  return { count, comp };
}

// ======================= ADVANCED =======================

// a * b % m for a, b < m < 2^30 without BigInt (split b into 15-bit halves)
function mulm(a, b, m) {
  return ((a * (b >>> 15)) % m * 32768 + a * (b & 32767)) % m;
}

// ---------- 16. Segment tree (point set, range query) ----------
// Any associative op: new SegTree(a, Math.min, Infinity), new SegTree(a, (x, y) => x + y, 0)
class SegTree {
  constructor(a, op, e) {
    this.n = a.length; this.op = op; this.e = e;
    this.t = new Array(2 * this.n).fill(e);
    for (let i = 0; i < this.n; i++) this.t[this.n + i] = a[i];
    for (let i = this.n - 1; i > 0; i--) this.t[i] = op(this.t[2 * i], this.t[2 * i + 1]);
  }
  set(i, v) {
    i += this.n;
    this.t[i] = v;
    for (i >>= 1; i >= 1; i >>= 1) this.t[i] = this.op(this.t[2 * i], this.t[2 * i + 1]);
  }
  query(l, r) {                          // inclusive
    let resl = this.e, resr = this.e;
    for (l += this.n, r += this.n + 1; l < r; l >>= 1, r >>= 1) {
      if (l & 1) resl = this.op(resl, this.t[l++]);
      if (r & 1) resr = this.op(this.t[--r], resr);
    }
    return this.op(resl, resr);
  }
}

// ---------- 17. Lazy segment tree (range add, range sum + range min) ----------
class LazySeg {
  constructor(a) {
    this.n = a.length;
    this.sm = new Array(4 * this.n).fill(0);
    this.mn = new Array(4 * this.n).fill(0);
    this.lz = new Array(4 * this.n).fill(0);
    this._build(1, 0, this.n - 1, a);
  }
  _build(x, l, r, a) {
    if (l === r) { this.sm[x] = this.mn[x] = a[l]; return; }
    const m = (l + r) >> 1;
    this._build(2 * x, l, m, a); this._build(2 * x + 1, m + 1, r, a);
    this._pull(x);
  }
  _pull(x) { this.sm[x] = this.sm[2 * x] + this.sm[2 * x + 1]; this.mn[x] = Math.min(this.mn[2 * x], this.mn[2 * x + 1]); }
  _apply(x, l, r, v) { this.sm[x] += v * (r - l + 1); this.mn[x] += v; this.lz[x] += v; }
  _push(x, l, r) {
    if (this.lz[x]) {
      const m = (l + r) >> 1;
      this._apply(2 * x, l, m, this.lz[x]); this._apply(2 * x + 1, m + 1, r, this.lz[x]);
      this.lz[x] = 0;
    }
  }
  add(ql, qr, v, x = 1, l = 0, r = this.n - 1) {          // a[ql..qr] += v
    if (qr < l || r < ql) return;
    if (ql <= l && r <= qr) { this._apply(x, l, r, v); return; }
    this._push(x, l, r);
    const m = (l + r) >> 1;
    this.add(ql, qr, v, 2 * x, l, m); this.add(ql, qr, v, 2 * x + 1, m + 1, r);
    this._pull(x);
  }
  query(ql, qr, x = 1, l = 0, r = this.n - 1) {           // returns [sum, min]
    if (qr < l || r < ql) return [0, Infinity];
    if (ql <= l && r <= qr) return [this.sm[x], this.mn[x]];
    this._push(x, l, r);
    const m = (l + r) >> 1;
    const a = this.query(ql, qr, 2 * x, l, m), b = this.query(ql, qr, 2 * x + 1, m + 1, r);
    return [a[0] + b[0], Math.min(a[1], b[1])];
  }
}

// ---------- 18. Sparse table (static range min, O(1) query) ----------
class SparseMin {
  constructor(a) {
    this.t = [a.slice()];
    for (let j = 1; (1 << j) <= a.length; j++) {
      const prev = this.t[j - 1], h = 1 << (j - 1), row = [];
      for (let i = 0; i + (1 << j) <= a.length; i++) row.push(Math.min(prev[i], prev[i + h]));
      this.t.push(row);
    }
  }
  query(l, r) {                          // inclusive
    const k = 31 - Math.clz32(r - l + 1);
    return Math.min(this.t[k][l], this.t[k][r - (1 << k) + 1]);
  }
}

// ---------- 19. LCA (binary lifting) + tree distance ----------
class LCA {
  constructor(adj, root = 0) {
    const n = adj.length;
    this.LOG = 1;
    while ((1 << this.LOG) < n) this.LOG++;
    this.depth = new Array(n).fill(0);
    const up0 = new Array(n).fill(root), seen = new Array(n).fill(false);
    const q = new Queue();
    seen[root] = true; q.push(root);
    while (q.size > 0) {
      const u = q.pop();
      for (const v of adj[u])
        if (!seen[v]) { seen[v] = true; this.depth[v] = this.depth[u] + 1; up0[v] = u; q.push(v); }
    }
    this.up = [up0];
    for (let k = 1; k < this.LOG; k++) {
      const prev = this.up[k - 1];
      this.up.push(prev.map(v => prev[v]));
    }
  }
  lca(a, b) {
    if (this.depth[a] < this.depth[b]) [a, b] = [b, a];
    let diff = this.depth[a] - this.depth[b];
    for (let k = 0; diff; k++, diff >>= 1) if (diff & 1) a = this.up[k][a];
    if (a === b) return a;
    for (let k = this.LOG - 1; k >= 0; k--)
      if (this.up[k][a] !== this.up[k][b]) { a = this.up[k][a]; b = this.up[k][b]; }
    return this.up[0][a];
  }
  dist(a, b) { return this.depth[a] + this.depth[b] - 2 * this.depth[this.lca(a, b)]; }
}

// ---------- 20. KMP prefix function + pattern search ----------
function prefixFunction(s) {
  const pi = new Array(s.length).fill(0);
  for (let i = 1; i < s.length; i++) {
    let k = pi[i - 1];
    while (k && s[i] !== s[k]) k = pi[k - 1];
    if (s[i] === s[k]) k++;
    pi[i] = k;
  }
  return pi;
}
function findAll(text, pat) {            // start indices of pat in text
  const pi = prefixFunction(pat), res = [];
  let k = 0;
  for (let i = 0; i < text.length; i++) {
    while (k && text[i] !== pat[k]) k = pi[k - 1];
    if (text[i] === pat[k]) k++;
    if (k === pat.length) { res.push(i - k + 1); k = pi[k - 1]; }
  }
  return res;
}

// ---------- 21. Z-function ----------
// z[i] = length of the longest common prefix of s and s.slice(i)  (z[0] = n)
function zFunction(s) {
  const n = s.length, z = new Array(n).fill(0);
  if (n) z[0] = n;
  for (let i = 1, l = 0, r = 0; i < n; i++) {
    if (i < r) z[i] = Math.min(r - i, z[i - l]);
    while (i + z[i] < n && s[z[i]] === s[i + z[i]]) z[i]++;
    if (i + z[i] > r) { l = i; r = i + z[i]; }
  }
  return z;
}

// ---------- 22. String hashing (double mod, random base) ----------
// h.get(l, r) === h.get(l2, r2)  ->  substrings equal (inclusive); get returns a string key
class StrHash {
  constructor(s) {
    const M1 = 1000000007, M2 = 998244353, b = 256 + Math.floor(Math.random() * 1e8);
    const n = s.length;
    this.h1 = new Array(n + 1).fill(0); this.h2 = new Array(n + 1).fill(0);
    this.p1 = new Array(n + 1).fill(1); this.p2 = new Array(n + 1).fill(1);
    for (let i = 0; i < n; i++) {
      const c = s.charCodeAt(i);
      this.h1[i + 1] = (mulm(this.h1[i], b, M1) + c) % M1;
      this.h2[i + 1] = (mulm(this.h2[i], b, M2) + c) % M2;
      this.p1[i + 1] = mulm(this.p1[i], b, M1);
      this.p2[i + 1] = mulm(this.p2[i], b, M2);
    }
    this.M1 = M1; this.M2 = M2;
  }
  get(l, r) {                            // inclusive
    r++;
    const a = (this.h1[r] - mulm(this.h1[l], this.p1[r - l], this.M1) + this.M1) % this.M1;
    const c = (this.h2[r] - mulm(this.h2[l], this.p2[r - l], this.M2) + this.M2) % this.M2;
    return a + ',' + c;
  }
}

// ---------- 23. Matrix power (mod MOD) ----------
function matMul(A, B) {
  const n = A.length, m = B.length, p = B[0].length;
  const C = Array.from({ length: n }, () => new Array(p).fill(0));
  for (let i = 0; i < n; i++)
    for (let k = 0; k < m; k++)
      if (A[i][k])
        for (let j = 0; j < p; j++) C[i][j] = (C[i][j] + mulm(A[i][k], B[k][j], MOD)) % MOD;
  return C;
}
function matPow(A, e) {
  let R = A.map((row, i) => row.map((_, j) => (i === j ? 1 : 0)));
  for (; e > 0; e = Math.floor(e / 2)) {
    if (e % 2 === 1) R = matMul(R, A);
    A = matMul(A, A);
  }
  return R;
}
// Fibonacci: F(n) = matPow([[1, 1], [1, 0]], n)[0][1]

// ---------- 24. Floyd-Warshall (all pairs, n <= ~400) ----------
// d[i][i] = 0, Infinity where no edge. Changed in place.
function floyd(d) {
  const n = d.length;
  for (let k = 0; k < n; k++)
    for (let i = 0; i < n; i++) {
      const dik = d[i][k];
      if (dik === Infinity) continue;
      for (let j = 0; j < n; j++) if (dik + d[k][j] < d[i][j]) d[i][j] = dik + d[k][j];
    }
  return d;
}

// ---------- 25. Kruskal MST (uses DSU, section 7) ----------
// edges: [w, u, v]. Returns the total weight (fewer than n-1 unions = not connected).
function kruskal(n, edges) {
  const ds = new DSU(n);
  let total = 0;
  for (const [w, u, v] of edges.slice().sort((x, y) => x[0] - y[0]))
    if (ds.union(u, v)) total += w;
  return total;
}

// ---------- 26. SCC (Kosaraju, iterative) ----------
// Returns { count, comp }; components numbered in topological order of the condensed graph.
function scc(adj) {
  const n = adj.length, radj = Array.from({ length: n }, () => []);
  for (let u = 0; u < n; u++) for (const v of adj[u]) radj[v].push(u);
  const order = [], seen = new Array(n).fill(false);
  for (let s = 0; s < n; s++) {
    if (seen[s]) continue;
    seen[s] = true;
    const st = [[s, 0]];
    while (st.length) {
      const top = st[st.length - 1], u = top[0];
      if (top[1] < adj[u].length) {
        const v = adj[u][top[1]++];
        if (!seen[v]) { seen[v] = true; st.push([v, 0]); }
      } else { order.push(u); st.pop(); }
    }
  }
  const comp = new Array(n).fill(-1);
  let count = 0;
  for (let idx = n - 1; idx >= 0; idx--) {
    const s = order[idx];
    if (comp[s] !== -1) continue;
    comp[s] = count;
    const st = [s];
    while (st.length) {
      const u = st.pop();
      for (const v of radj[u]) if (comp[v] === -1) { comp[v] = count; st.push(v); }
    }
    count++;
  }
  return { count, comp };
}

// ---------- 27. Max flow (Dinic) ----------
// const f = new Dinic(n); f.addEdge(u, v, cap); f.maxFlow(s, t)
// Bipartite matching = source -> left (cap 1), left -> right (1), right -> sink (1).
class Dinic {
  constructor(n) { this.n = n; this.g = Array.from({ length: n }, () => []); }
  addEdge(u, v, cap) {
    this.g[u].push({ to: v, cap, rev: this.g[v].length });
    this.g[v].push({ to: u, cap: 0, rev: this.g[u].length - 1 });
  }
  _bfs(s, t) {
    this.level = new Array(this.n).fill(-1);
    const q = new Queue();
    this.level[s] = 0; q.push(s);
    while (q.size > 0) {
      const u = q.pop();
      for (const e of this.g[u])
        if (e.cap > 0 && this.level[e.to] < 0) { this.level[e.to] = this.level[u] + 1; q.push(e.to); }
    }
    return this.level[t] >= 0;
  }
  _dfs(u, t, f) {
    if (u === t) return f;
    for (; this.it[u] < this.g[u].length; this.it[u]++) {
      const e = this.g[u][this.it[u]];
      if (e.cap > 0 && this.level[e.to] === this.level[u] + 1) {
        const d = this._dfs(e.to, t, Math.min(f, e.cap));
        if (d > 0) { e.cap -= d; this.g[e.to][e.rev].cap += d; return d; }
      }
    }
    return 0;
  }
  maxFlow(s, t) {
    let flow = 0;
    while (this._bfs(s, t)) {
      this.it = new Array(this.n).fill(0);
      for (let f; (f = this._dfs(s, t, Infinity)) > 0;) flow += f;
    }
    return flow;
  }
}

// ---------- 28. Geometry: cross, convex hull, polygon area ----------
// Points are [x, y].
const cross = (o, a, b) => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);  // > 0: left turn
function convexHull(points) {          // counter-clockwise, no collinear points
  const pts = points.slice().sort((p, q) => p[0] - q[0] || p[1] - q[1])
    .filter((p, i, arr) => i === 0 || p[0] !== arr[i - 1][0] || p[1] !== arr[i - 1][1]);
  if (pts.length <= 2) return pts;
  const lower = [], upper = [];
  for (const p of pts) {
    while (lower.length >= 2 && cross(lower[lower.length - 2], lower[lower.length - 1], p) <= 0) lower.pop();
    lower.push(p);
  }
  for (let i = pts.length - 1; i >= 0; i--) {
    const p = pts[i];
    while (upper.length >= 2 && cross(upper[upper.length - 2], upper[upper.length - 1], p) <= 0) upper.pop();
    upper.push(p);
  }
  return lower.slice(0, -1).concat(upper.slice(0, -1));
}
function area2(poly) {                 // TWICE the area (stays an integer)
  let s = 0;
  for (let i = 0; i < poly.length; i++) {
    const a = poly[i], b = poly[(i + 1) % poly.length];
    s += a[0] * b[1] - b[0] * a[1];
  }
  return Math.abs(s);
}

// ---------- 29. LIS (strictly increasing), O(n log n) ----------
// For non-decreasing, change `tails[mid] < v` to `tails[mid] <= v`.
function lisLength(a) {
  const tails = [];
  for (const v of a) {
    let lo = 0, hi = tails.length;
    while (lo < hi) {
      const mid = (lo + hi) >> 1;
      if (tails[mid] < v) lo = mid + 1; else hi = mid;
    }
    tails[lo] = v;
  }
  return tails.length;
}

// ---------- 30. Factorization (smallest prime factor) + extended gcd ----------
function spfSieve(n) {                 // spf[x] = smallest prime factor of x
  const spf = new Int32Array(n + 1);
  for (let i = 0; i <= n; i++) spf[i] = i;
  for (let i = 2; i * i <= n; i++)
    if (spf[i] === i)
      for (let j = i * i; j <= n; j += i) if (spf[j] === j) spf[j] = i;
  return spf;
}
function factorize(x, spf) {           // primes with repetition, ascending
  const res = [];
  while (x > 1) { res.push(spf[x]); x /= spf[x]; }
  return res;
}
function extGcd(a, b) {                // returns [g, x, y] with a*x + b*y = g
  let x0 = 1, y0 = 0, x1 = 0, y1 = 1;
  while (b) {
    const q = Math.floor(a / b);
    [a, b] = [b, a - q * b];
    [x0, x1] = [x1, x0 - q * x1];
    [y0, y1] = [y1, y0 - q * y1];
  }
  return [a, x0, y0];
}

// ======================= MORE =======================

// ---------- 31. NTT polynomial multiply (mod 998244353) ----------
// Coefficients mod 998244353. For exact (non-mod) products the true values must stay below it.
const NTT_MOD = 998244353;
function powm(a, e, m) {                 // a^e mod m for m < 2^30 (no BigInt)
  let r = 1;
  a %= m;
  for (; e > 0; e = Math.floor(e / 2)) {
    if (e % 2 === 1) r = mulm(r, a, m);
    a = mulm(a, a, m);
  }
  return r;
}
function ntt(a, invert) {
  const n = a.length;
  for (let i = 1, j = 0; i < n; i++) {
    let bit = n >> 1;
    for (; j & bit; bit >>= 1) j ^= bit;
    j ^= bit;
    if (i < j) [a[i], a[j]] = [a[j], a[i]];
  }
  for (let len = 2; len <= n; len <<= 1) {
    let w = powm(3, (NTT_MOD - 1) / len, NTT_MOD);
    if (invert) w = powm(w, NTT_MOD - 2, NTT_MOD);
    const half = len >> 1, ws = new Array(half);
    ws[0] = 1;
    for (let k = 1; k < half; k++) ws[k] = mulm(ws[k - 1], w, NTT_MOD);
    for (let i = 0; i < n; i += len)
      for (let k = 0; k < half; k++) {
        const u = a[i + k], v = mulm(a[i + k + half], ws[k], NTT_MOD);
        a[i + k] = (u + v) % NTT_MOD;
        a[i + k + half] = (u - v + NTT_MOD) % NTT_MOD;
      }
  }
  if (invert) {
    const invN = powm(n, NTT_MOD - 2, NTT_MOD);
    for (let i = 0; i < n; i++) a[i] = mulm(a[i], invN, NTT_MOD);
  }
}
function nttMultiply(a, b) {             // lowest degree first
  if (!a.length || !b.length) return [];
  const need = a.length + b.length - 1;
  let n = 1;
  while (n < need) n <<= 1;
  const fa = new Array(n).fill(0), fb = new Array(n).fill(0);
  a.forEach((x, i) => { fa[i] = ((x % NTT_MOD) + NTT_MOD) % NTT_MOD; });
  b.forEach((x, i) => { fb[i] = ((x % NTT_MOD) + NTT_MOD) % NTT_MOD; });
  ntt(fa, false); ntt(fb, false);
  for (let i = 0; i < n; i++) fa[i] = mulm(fa[i], fb[i], NTT_MOD);
  ntt(fa, true);
  return fa.slice(0, need);
}

// ---------- 32. Bellman-Ford + negative cycle ----------
// edges: [u, v, w], weights may be negative. Returns { dist, neg } (neg = cycle reachable from s).
function bellmanFord(n, edges, s) {
  const dist = new Array(n).fill(Infinity);
  dist[s] = 0;
  for (let it = 0; it < n - 1; it++) {
    let changed = false;
    for (const [u, v, w] of edges)
      if (dist[u] !== Infinity && dist[u] + w < dist[v]) { dist[v] = dist[u] + w; changed = true; }
    if (!changed) return { dist, neg: false };
  }
  for (const [u, v, w] of edges)
    if (dist[u] !== Infinity && dist[u] + w < dist[v]) return { dist, neg: true };
  return { dist, neg: false };
}

// ---------- 33. Bridges + articulation points (undirected, iterative) ----------
// edges: [u, v]; parallel edges are handled. Returns { bridges (edge indices), cuts } sorted.
function bridgesAndCuts(n, edges) {
  const adj = Array.from({ length: n }, () => []);
  edges.forEach(([u, v], i) => { adj[u].push([v, i]); adj[v].push([u, i]); });
  const tin = new Array(n).fill(-1), low = new Array(n).fill(0), isCut = new Array(n).fill(false), bridges = [];
  let timer = 0;
  for (let root = 0; root < n; root++) {
    if (tin[root] !== -1) continue;
    tin[root] = low[root] = timer++;
    let children = 0;
    const st = [[root, -1, 0]];          // node, parent edge, next index
    while (st.length) {
      const top = st[st.length - 1], u = top[0], pe = top[1];
      if (top[2] < adj[u].length) {
        const [v, eid] = adj[u][top[2]++];
        if (eid === pe) continue;
        if (tin[v] === -1) {
          tin[v] = low[v] = timer++;
          st.push([v, eid, 0]);
          if (u === root) children++;
        } else low[u] = Math.min(low[u], tin[v]);
      } else {
        st.pop();
        if (st.length) {
          const p = st[st.length - 1][0];
          low[p] = Math.min(low[p], low[u]);
          if (low[u] > tin[p]) bridges.push(pe);
          if (p !== root && low[u] >= tin[p]) isCut[p] = true;
        }
      }
    }
    if (children > 1) isCut[root] = true;
  }
  const cuts = [];
  for (let v = 0; v < n; v++) if (isCut[v]) cuts.push(v);
  return { bridges: bridges.sort((x, y) => x - y), cuts };
}

// ---------- 34. Trie (prefix counting) ----------
class Trie {
  constructor() { this.nxt = [new Map()]; this.pass = [0]; this.end = [0]; }
  insert(w) {
    let x = 0;
    for (const ch of w) {
      if (!this.nxt[x].has(ch)) { this.nxt[x].set(ch, this.nxt.length); this.nxt.push(new Map()); this.pass.push(0); this.end.push(0); }
      x = this.nxt[x].get(ch);
      this.pass[x]++;
    }
    this.end[x]++;
  }
  _walk(s) {
    let x = 0;
    for (const ch of s) { if (!this.nxt[x].has(ch)) return -1; x = this.nxt[x].get(ch); }
    return x;
  }
  countPrefix(p) {                       // inserted words starting with p
    if (p === '') return this.end.reduce((s, x) => s + x, 0);
    const x = this._walk(p);
    return x < 0 ? 0 : this.pass[x];
  }
  countWord(w) { const x = this._walk(w); return x < 0 ? 0 : this.end[x]; }
}

// ---------- 35. Coordinate compression ----------
// Returns { ranks, values }: ranks[i] = index of a[i] among the sorted distinct values.
function compress(a) {
  const values = [...new Set(a)].sort((x, y) => x - y);
  const pos = new Map(values.map((v, i) => [v, i]));
  return { ranks: a.map(v => pos.get(v)), values };
}

// ---------- 36. Ternary search (integer argmax of a unimodal f) ----------
// f must strictly increase then strictly decrease. For a minimum, negate f.
function ternaryMax(lo, hi, f) {
  while (hi - lo > 2) {
    const m1 = lo + Math.floor((hi - lo) / 3), m2 = hi - Math.floor((hi - lo) / 3);
    if (f(m1) < f(m2)) lo = m1 + 1; else hi = m2 - 1;
  }
  let best = lo;
  for (let x = lo + 1; x <= hi; x++) if (f(x) > f(best)) best = x;
  return best;
}
// Real numbers: repeat 100 times: m1, m2 at thirds; if (f(m1) < f(m2)) lo = m1; else hi = m2;

// ---------- 37. Miller-Rabin primality (BigInt, deterministic up to 2^64) ----------
// isPrime(123n) or isPrime(BigInt(x)). Numbers above 9e15 must be read as BigInt: BigInt(next()).
function powBig(a, e, m) {
  let r = 1n;
  for (a %= m; e > 0n; e >>= 1n, a = a * a % m) if (e & 1n) r = r * a % m;
  return r;
}
function isPrime(n) {
  n = BigInt(n);
  if (n < 2n) return false;
  const bases = [2n, 3n, 5n, 7n, 11n, 13n, 17n, 19n, 23n, 29n, 31n, 37n];
  for (const p of bases) if (n % p === 0n) return n === p;
  let d = n - 1n, r = 0;
  while (d % 2n === 0n) { d /= 2n; r++; }
  for (const a of bases) {
    let x = powBig(a, d, n);
    if (x === 1n || x === n - 1n) continue;
    let comp = true;
    for (let i = 0; i < r - 1 && comp; i++) { x = x * x % n; if (x === n - 1n) comp = false; }
    if (comp) return false;
  }
  return true;
}

// ---------- 38. CRT (any moduli, BigInt) + Euler totient ----------
// x = a1 (mod m1), x = a2 (mod m2). Arguments as BigInt; returns [x, lcm] or null if impossible.
function crt(a1, m1, a2, m2) {
  let a = m1, b = m2, x0 = 1n, x1 = 0n;
  while (b) { const q = a / b; [a, b] = [b, a - q * b]; [x0, x1] = [x1, x0 - q * x1]; }
  const g = a, p = x0;                   // m1 * p = g (mod m2)
  if ((a2 - a1) % g !== 0n) return null;
  const m = m2 / g, l = m1 / g * m2;
  let t = ((a2 - a1) / g % m) * (p % m) % m;
  if (t < 0n) t += m;
  let x = (a1 + m1 * t) % l;
  if (x < 0n) x += l;
  return [x, l];
}
function totient(n) {                    // count of 1..n coprime with n, O(sqrt n)
  let result = n;
  for (let p = 2; p * p <= n; p++)
    if (n % p === 0) {
      while (n % p === 0) n /= p;
      result -= result / p;
    }
  if (n > 1) result -= result / n;
  return result;
}

// ---------- 39. Segment intersection + point in polygon (integer points [x, y]) ----------
// Uses cross (28).
function onSegment(p, a, b) {
  return cross(a, b, p) === 0 && Math.min(a[0], b[0]) <= p[0] && p[0] <= Math.max(a[0], b[0])
    && Math.min(a[1], b[1]) <= p[1] && p[1] <= Math.max(a[1], b[1]);
}
function segmentsIntersect(p1, p2, p3, p4) {   // touching counts
  const d1 = cross(p3, p4, p1), d2 = cross(p3, p4, p2), d3 = cross(p1, p2, p3), d4 = cross(p1, p2, p4);
  if (((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0)) && ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0))) return true;
  return onSegment(p1, p3, p4) || onSegment(p2, p3, p4) || onSegment(p3, p1, p2) || onSegment(p4, p1, p2);
}
function pointInPolygon(p, poly) {       // 1 inside, 0 boundary, -1 outside
  let inside = false;
  for (let i = 0; i < poly.length; i++) {
    const a = poly[i], b = poly[(i + 1) % poly.length];
    if (onSegment(p, a, b)) return 0;
    if ((a[1] > p[1]) !== (b[1] > p[1]))
      if ((cross(a, b, p) > 0) === (b[1] > a[1])) inside = !inside;
  }
  return inside ? 1 : -1;
}

// ---------- 40. Manacher (longest palindrome) ----------
// Returns [start, length] of a longest palindromic substring, in O(n).
function manacher(s) {
  const n = s.length;
  if (n === 0) return [0, 0];
  const d1 = new Array(n).fill(0), d2 = new Array(n).fill(0);
  for (let i = 0, l = 0, r = -1; i < n; i++) {
    let k = i > r ? 1 : Math.min(d1[l + r - i], r - i + 1);
    while (i - k >= 0 && i + k < n && s[i - k] === s[i + k]) k++;
    d1[i] = k;
    if (i + k - 1 > r) { l = i - k + 1; r = i + k - 1; }
  }
  for (let i = 0, l = 0, r = -1; i < n; i++) {
    let k = i > r ? 0 : Math.min(d2[l + r - i + 1], r - i + 1);
    while (i - k - 1 >= 0 && i + k < n && s[i - k - 1] === s[i + k]) k++;
    d2[i] = k;
    if (i + k - 1 > r) { l = i - k; r = i + k - 1; }
  }
  let best = [0, 1];
  for (let i = 0; i < n; i++) {
    if (2 * d1[i] - 1 > best[1]) best = [i - d1[i] + 1, 2 * d1[i] - 1];
    if (2 * d2[i] > best[1]) best = [i - d2[i], 2 * d2[i]];
  }
  return best;
}

// ---------- Solution ----------
function main() {
  const n = nextInt();
  const a = [];
  for (let i = 0; i < n; i++) a.push(nextInt());

  const out = [];
  // solve here

  process.stdout.write(out.join('\n') + '\n');
}

main();
