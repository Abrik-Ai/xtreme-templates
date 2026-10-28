# IEEEXtreme 20.0 - team template (Python / PyPy3)
# Submit with PyPy3 by default. Use CPython only for numpy, huge ints or deep recursion.
#
# CONTENTS
#   1  Fast input / output          9  Sieve + gcd / lcm
#   2  Prefix sums (1D)            10  Fenwick tree (sums with updates)
#   3  Binary search on answer     11  Topological sort
#   4  BFS on a grid               12  Kadane + sliding window
#   5  BFS on a graph + path       13  Interactive I/O (GAME tasks)
#   6  Dijkstra                    14  2D prefix sums
#   7  DSU / union-find            15  Iterative DFS + components
#   8  Modular math (pow, inverse, nCr)
#  ADVANCED
#  16  Segment tree (point set, range query)    24  Floyd-Warshall (all pairs)
#  17  Lazy segment tree (range add, sum/min)   25  Kruskal MST
#  18  Sparse table (static range min)          26  SCC (Kosaraju, iterative)
#  19  LCA (binary lifting) + tree distance     27  Max flow (Dinic)
#  20  KMP prefix function + pattern search     28  Geometry: cross, convex hull, area
#  21  Z-function                               29  LIS (longest increasing subsequence)
#  22  String hashing (double mod)              30  Factorization (SPF) + extended gcd
#  23  Matrix power (e.g. Fibonacci mod p)
#  MORE
#  31  NTT polynomial multiply (mod 998244353)  36  Ternary search (max of unimodal f)
#  32  Bellman-Ford + negative cycle            37  Miller-Rabin primality (up to 2^64)
#  33  Bridges + articulation points            38  CRT (any moduli) + Euler totient
#  34  Trie (prefix counting)                   39  Segment intersection + point in polygon
#  35  Coordinate compression                   40  Manacher (longest palindrome)
#  MORE
#  31  NTT (fast polynomial multiplication)     36  Ternary search
#  32  Bellman-Ford (negative weights / cycles) 37  Miller-Rabin (primes up to 1e18)
#  33  Bridges + articulation points            38  CRT + Euler's totient
#  34  Trie (prefix counts)                     39  Geometry: segment intersection, point in polygon
#  35  Coordinate compression                   40  Manacher (longest palindrome)
import sys
import heapq
from collections import deque

# ---------- 1. Fast input ----------
# Numbers only: read everything once.
#   data = sys.stdin.buffer.read().split(); idx = 0
#   n = int(data[idx]); idx += 1
# Text / grids: read line by line.
input = sys.stdin.readline


# ---------- 2. Prefix sums ----------
# p[i] = a[0] + ... + a[i-1];  sum of a[l..r] (0-indexed, inclusive) = p[r+1] - p[l]
def prefix(a):
    p = [0]
    for v in a:
        p.append(p[-1] + v)
    return p


# ---------- 3. Binary search on the answer ----------
# Smallest x in [lo, hi] with ok(x) True. ok must be F F F ... T T T.
# If nothing works, returns hi (check ok(hi) yourself).
def first_true(lo, hi, ok):
    while lo < hi:
        mid = (lo + hi) // 2
        if ok(mid):
            hi = mid
        else:
            lo = mid + 1
    return lo


# ---------- 4. BFS on a grid ----------
# Distances from (si, sj) over cells that are not '#'. -1 = unreachable.
MOVES = [(-1, 0), (1, 0), (0, -1), (0, 1)]


def bfs_grid(grid, si, sj):
    n, m = len(grid), len(grid[0])
    dist = [[-1] * m for _ in range(n)]
    dist[si][sj] = 0
    q = deque([(si, sj)])
    while q:
        i, j = q.popleft()
        for di, dj in MOVES:
            ni, nj = i + di, j + dj
            if 0 <= ni < n and 0 <= nj < m and grid[ni][nj] != '#' and dist[ni][nj] == -1:
                dist[ni][nj] = dist[i][j] + 1
                q.append((ni, nj))
    return dist


# ---------- 5. BFS on a graph (unweighted shortest path) ----------
# adj: list of neighbour lists, nodes 0..n-1. Returns dist and parent (for the path).
def bfs_graph(adj, s):
    n = len(adj)
    dist = [-1] * n
    par = [-1] * n
    dist[s] = 0
    q = deque([s])
    while q:
        u = q.popleft()
        for v in adj[u]:
            if dist[v] == -1:
                dist[v] = dist[u] + 1
                par[v] = u
                q.append(v)
    return dist, par


def path_to(par, t):
    path = []
    while t != -1:
        path.append(t)
        t = par[t]
    return path[::-1]


# ---------- 6. Dijkstra (non-negative weights) ----------
# adj[u] = list of (v, w)
INF = float('inf')


def dijkstra(adj, s):
    dist = [INF] * len(adj)
    dist[s] = 0
    pq = [(0, s)]
    while pq:
        d, u = heapq.heappop(pq)
        if d > dist[u]:
            continue  # old entry, a shorter one was already used
        for v, w in adj[u]:
            nd = d + w
            if nd < dist[v]:
                dist[v] = nd
                heapq.heappush(pq, (nd, v))
    return dist


# ---------- 7. DSU / union-find ----------
class DSU:
    def __init__(self, n):
        self.p = list(range(n))
        self.sz = [1] * n

    def find(self, x):
        root = x
        while self.p[root] != root:
            root = self.p[root]
        while self.p[x] != root:  # path compression, no recursion
            self.p[x], x = root, self.p[x]
        return root

    def union(self, a, b):
        a, b = self.find(a), self.find(b)
        if a == b:
            return False
        if self.sz[a] < self.sz[b]:
            a, b = b, a
        self.p[b] = a
        self.sz[a] += self.sz[b]
        return True


# ---------- 8. Modular math (MOD must be prime for the inverse) ----------
MOD = 10**9 + 7
# pow(a, b, MOD) is built in and fast. Inverse: pow(a, MOD - 2, MOD), or pow(a, -1, MOD).


def build_factorials(n):
    fact = [1] * (n + 1)
    for i in range(1, n + 1):
        fact[i] = fact[i - 1] * i % MOD
    inv = [1] * (n + 1)
    inv[n] = pow(fact[n], MOD - 2, MOD)
    for i in range(n, 0, -1):
        inv[i - 1] = inv[i] * i % MOD
    return fact, inv


def ncr(n, r, fact, inv):
    if r < 0 or r > n:
        return 0
    return fact[n] * inv[r] % MOD * inv[n - r] % MOD


# ---------- 9. Sieve + gcd / lcm ----------
# is_prime[i] for 0..n.  gcd: math.gcd;  lcm: a // gcd(a, b) * b  (or math.lcm)
def sieve(n):
    is_prime = [True] * (n + 1)
    is_prime[0] = False
    if n >= 1:
        is_prime[1] = False
    i = 2
    while i * i <= n:
        if is_prime[i]:
            for j in range(i * i, n + 1, i):
                is_prime[j] = False
        i += 1
    return is_prime


# ---------- 10. Fenwick tree (range sums WITH updates) ----------
# 0-indexed outside, 1-indexed inside.
class Fenwick:
    def __init__(self, n):
        self.n = n
        self.t = [0] * (n + 1)

    def add(self, i, delta):          # a[i] += delta
        i += 1
        while i <= self.n:
            self.t[i] += delta
            i += i & -i

    def prefix_sum(self, i):          # a[0] + ... + a[i-1]
        s = 0
        while i > 0:
            s += self.t[i]
            i -= i & -i
        return s

    def range_sum(self, l, r):        # a[l] + ... + a[r], inclusive
        return self.prefix_sum(r + 1) - self.prefix_sum(l)


# ---------- 11. Topological sort (Kahn) ----------
# adj[u] = nodes that must come AFTER u. Returns an order, or [] if there is a cycle.
def topo_sort(adj):
    n = len(adj)
    indeg = [0] * n
    for u in range(n):
        for v in adj[u]:
            indeg[v] += 1
    q = deque(u for u in range(n) if indeg[u] == 0)
    order = []
    while q:
        u = q.popleft()
        order.append(u)
        for v in adj[u]:
            indeg[v] -= 1
            if indeg[v] == 0:
                q.append(v)
    return order if len(order) == n else []


# ---------- 12. Reminders: Kadane + sliding window ----------
def kadane(a):                        # max subarray sum, at least one element
    cur, best = 0, -10**18
    for v in a:
        cur = max(v, cur + v)
        best = max(best, cur)
    return best


def count_window(a, x):               # subarrays with sum == x, ALL values positive
    left = s = count = 0
    for right in range(len(a)):
        s += a[right]
        while s > x:
            s -= a[left]
            left += 1
        if s == x:
            count += 1
    return count


# ---------- 13. Interactive I/O (GAME tasks) ----------
# The bot answers only after it gets your move, so:
#   - NEVER use sys.stdin.buffer.read() (it waits for the end of input forever)
#   - read ONE line at a time, and FLUSH after every line you print.
def read_line():
    line = sys.stdin.readline()
    return line.strip() if line else None   # None = the judge closed the input


def say(*parts):
    sys.stdout.write(" ".join(map(str, parts)) + "\n")
    sys.stdout.flush()
# Example loop:
#   while True:
#       msg = read_line()
#       if msg is None or msg == "END": break
#       say(my_move(msg))


# ---------- 14. 2D prefix sums ----------
# P[i][j] = sum of g[0..i-1][0..j-1].  Rectangle sum, inclusive, 0-indexed:
def prefix2d(g):
    n, m = len(g), len(g[0])
    P = [[0] * (m + 1) for _ in range(n + 1)]
    for i in range(n):
        row, prev, cur = g[i], P[i], P[i + 1]
        for j in range(m):
            cur[j + 1] = row[j] + cur[j] + prev[j + 1] - prev[j]
    return P


def rect_sum(P, r1, c1, r2, c2):
    return P[r2 + 1][c2 + 1] - P[r1][c2 + 1] - P[r2 + 1][c1] + P[r1][c1]
# Char grid (count '*'): prefix2d([[1 if ch == '*' else 0 for ch in row] for row in grid])


# ---------- 15. Iterative DFS + connected components ----------
# No recursion (deep recursion crashes Python). comp[v] = component number of v.
def components(adj):
    n = len(adj)
    comp = [-1] * n
    count = 0
    for s in range(n):
        if comp[s] != -1:
            continue
        comp[s] = count
        stack = [s]
        while stack:
            u = stack.pop()
            for v in adj[u]:
                if comp[v] == -1:
                    comp[v] = count
                    stack.append(v)
        count += 1
    return count, comp


# ======================= ADVANCED =======================

# ---------- 16. Segment tree (point set, range query) ----------
# Any associative op: min, max, sum (identity 0), gcd (identity 0) ...
#   st = SegTree(a, min, float('inf'));  st.set(i, v);  st.query(l, r)  (inclusive)
class SegTree:
    def __init__(self, a, op, identity):
        self.n = n = len(a)
        self.op, self.e = op, identity
        self.t = [identity] * (2 * n)
        self.t[n:] = a
        for i in range(n - 1, 0, -1):
            self.t[i] = op(self.t[2 * i], self.t[2 * i + 1])

    def set(self, i, v):
        i += self.n
        self.t[i] = v
        while i > 1:
            i //= 2
            self.t[i] = self.op(self.t[2 * i], self.t[2 * i + 1])

    def query(self, l, r):            # inclusive
        resl = resr = self.e
        l += self.n
        r += self.n + 1
        while l < r:
            if l & 1:
                resl = self.op(resl, self.t[l]); l += 1
            if r & 1:
                r -= 1; resr = self.op(self.t[r], resr)
            l //= 2; r //= 2
        return self.op(resl, resr)


# ---------- 17. Lazy segment tree (range add, range sum + range min) ----------
class LazySeg:
    def __init__(self, a):
        self.n = n = len(a)
        self.sm = [0] * (4 * n)
        self.mn = [0] * (4 * n)
        self.lz = [0] * (4 * n)
        self._build(1, 0, n - 1, a)

    def _build(self, x, l, r, a):
        stack = [(x, l, r, False)]                 # iterative build
        while stack:
            x, l, r, done = stack.pop()
            if l == r:
                self.sm[x] = self.mn[x] = a[l]
            elif done:
                self._pull(x)
            else:
                m = (l + r) // 2
                stack += [(x, l, r, True), (2 * x, l, m, False), (2 * x + 1, m + 1, r, False)]

    def _pull(self, x):
        self.sm[x] = self.sm[2 * x] + self.sm[2 * x + 1]
        self.mn[x] = min(self.mn[2 * x], self.mn[2 * x + 1])

    def _apply(self, x, l, r, v):
        self.sm[x] += v * (r - l + 1)
        self.mn[x] += v
        self.lz[x] += v

    def _push(self, x, l, r):
        if self.lz[x]:
            m = (l + r) // 2
            self._apply(2 * x, l, m, self.lz[x])
            self._apply(2 * x + 1, m + 1, r, self.lz[x])
            self.lz[x] = 0

    def add(self, ql, qr, v, x=1, l=0, r=None):    # a[ql..qr] += v (recursion depth ~log n)
        if r is None:
            r = self.n - 1
        if qr < l or r < ql:
            return
        if ql <= l and r <= qr:
            self._apply(x, l, r, v)
            return
        self._push(x, l, r)
        m = (l + r) // 2
        self.add(ql, qr, v, 2 * x, l, m)
        self.add(ql, qr, v, 2 * x + 1, m + 1, r)
        self._pull(x)

    def query(self, ql, qr, x=1, l=0, r=None):     # returns (sum, min) of a[ql..qr]
        if r is None:
            r = self.n - 1
        if qr < l or r < ql:
            return 0, float('inf')
        if ql <= l and r <= qr:
            return self.sm[x], self.mn[x]
        self._push(x, l, r)
        m = (l + r) // 2
        s1, m1 = self.query(ql, qr, 2 * x, l, m)
        s2, m2 = self.query(ql, qr, 2 * x + 1, m + 1, r)
        return s1 + s2, min(m1, m2)


# ---------- 18. Sparse table (static range min, O(1) query) ----------
class SparseMin:
    def __init__(self, a):
        self.t = [list(a)]
        j = 1
        while (1 << j) <= len(a):
            prev, h = self.t[-1], 1 << (j - 1)
            self.t.append([min(prev[i], prev[i + h]) for i in range(len(a) - (1 << j) + 1)])
            j += 1

    def query(self, l, r):                         # inclusive
        k = (r - l + 1).bit_length() - 1
        return min(self.t[k][l], self.t[k][r - (1 << k) + 1])


# ---------- 19. LCA (binary lifting) + tree distance ----------
# adj: undirected tree, nodes 0..n-1.
class LCA:
    def __init__(self, adj, root=0):
        n = len(adj)
        self.LOG = max(1, n.bit_length())
        self.depth = [0] * n
        up0 = [root] * n
        seen = [False] * n
        seen[root] = True
        q = deque([root])
        while q:
            u = q.popleft()
            for v in adj[u]:
                if not seen[v]:
                    seen[v] = True
                    self.depth[v] = self.depth[u] + 1
                    up0[v] = u
                    q.append(v)
        self.up = [up0]
        for _ in range(1, self.LOG):
            prev = self.up[-1]
            self.up.append([prev[prev[v]] for v in range(n)])

    def lca(self, a, b):
        if self.depth[a] < self.depth[b]:
            a, b = b, a
        diff = self.depth[a] - self.depth[b]
        k = 0
        while diff:
            if diff & 1:
                a = self.up[k][a]
            diff >>= 1; k += 1
        if a == b:
            return a
        for k in range(self.LOG - 1, -1, -1):
            if self.up[k][a] != self.up[k][b]:
                a, b = self.up[k][a], self.up[k][b]
        return self.up[0][a]

    def dist(self, a, b):
        return self.depth[a] + self.depth[b] - 2 * self.depth[self.lca(a, b)]


# ---------- 20. KMP prefix function + pattern search ----------
def prefix_function(s):
    pi = [0] * len(s)
    for i in range(1, len(s)):
        k = pi[i - 1]
        while k and s[i] != s[k]:
            k = pi[k - 1]
        if s[i] == s[k]:
            k += 1
        pi[i] = k
    return pi


def find_all(text, pat):                           # start indices of pat in text
    pi = prefix_function(pat)
    res, k = [], 0
    for i, ch in enumerate(text):
        while k and ch != pat[k]:
            k = pi[k - 1]
        if ch == pat[k]:
            k += 1
        if k == len(pat):
            res.append(i - k + 1)
            k = pi[k - 1]
    return res


# ---------- 21. Z-function ----------
# z[i] = length of the longest common prefix of s and s[i:]  (z[0] = n)
def z_function(s):
    n = len(s)
    z = [0] * n
    if n:
        z[0] = n
    l = r = 0
    for i in range(1, n):
        if i < r:
            z[i] = min(r - i, z[i - l])
        while i + z[i] < n and s[z[i]] == s[i + z[i]]:
            z[i] += 1
        if i + z[i] > r:
            l, r = i, i + z[i]
    return z


# ---------- 22. String hashing (double mod, random base) ----------
#   h = StrHash(s);  h.get(l, r) == h.get(l2, r2)  ->  substrings s[l..r] equal (inclusive)
import random


class StrHash:
    M1, M2 = 1_000_000_007, 998_244_353

    def __init__(self, s):
        b = self.B = random.randint(256, 10**8)
        n = len(s)
        self.h1 = [0] * (n + 1); self.h2 = [0] * (n + 1)
        self.p1 = [1] * (n + 1); self.p2 = [1] * (n + 1)
        for i, ch in enumerate(s):
            c = ord(ch)
            self.h1[i + 1] = (self.h1[i] * b + c) % self.M1
            self.h2[i + 1] = (self.h2[i] * b + c) % self.M2
            self.p1[i + 1] = self.p1[i] * b % self.M1
            self.p2[i + 1] = self.p2[i] * b % self.M2

    def get(self, l, r):                           # inclusive
        r += 1
        return ((self.h1[r] - self.h1[l] * self.p1[r - l]) % self.M1,
                (self.h2[r] - self.h2[l] * self.p2[r - l]) % self.M2)


# ---------- 23. Matrix power (mod MOD) ----------
def mat_mul(A, B):
    n, m, p = len(A), len(B), len(B[0])
    C = [[0] * p for _ in range(n)]
    for i in range(n):
        Ai, Ci = A[i], C[i]
        for k in range(m):
            a = Ai[k]
            if a:
                Bk = B[k]
                for j in range(p):
                    Ci[j] = (Ci[j] + a * Bk[j]) % MOD
    return C


def mat_pow(A, e):
    R = [[int(i == j) for j in range(len(A))] for i in range(len(A))]
    while e:
        if e & 1:
            R = mat_mul(R, A)
        A = mat_mul(A, A)
        e >>= 1
    return R
# Fibonacci: F(n) = mat_pow([[1, 1], [1, 0]], n)[0][1]


# ---------- 24. Floyd-Warshall (all pairs, n <= ~400) ----------
# d: n x n matrix, d[i][i] = 0, INF where no edge. Changed in place.
def floyd(d):
    n = len(d)
    for k in range(n):
        dk = d[k]
        for i in range(n):
            dik = d[i][k]
            if dik == INF:
                continue
            di = d[i]
            for j in range(n):
                if dik + dk[j] < di[j]:
                    di[j] = dik + dk[j]
    return d


# ---------- 25. Kruskal MST (uses DSU, section 7) ----------
# edges: list of (w, u, v). Returns (total weight, chosen edges); fewer than n-1 edges = not connected.
def kruskal(n, edges):
    ds = DSU(n)
    total, used = 0, []
    for w, u, v in sorted(edges):
        if ds.union(u, v):
            total += w
            used.append((u, v, w))
    return total, used


# ---------- 26. SCC (Kosaraju, iterative) ----------
# Returns (count, comp). Components are numbered in topological order of the condensed graph.
def scc(adj):
    n = len(adj)
    radj = [[] for _ in range(n)]
    for u in range(n):
        for v in adj[u]:
            radj[v].append(u)
    order, seen = [], [False] * n
    for s in range(n):
        if seen[s]:
            continue
        seen[s] = True
        stack = [(s, 0)]
        while stack:
            u, i = stack.pop()
            if i < len(adj[u]):
                stack.append((u, i + 1))
                v = adj[u][i]
                if not seen[v]:
                    seen[v] = True
                    stack.append((v, 0))
            else:
                order.append(u)
    comp, count = [-1] * n, 0
    for s in reversed(order):
        if comp[s] != -1:
            continue
        comp[s] = count
        stack = [s]
        while stack:
            u = stack.pop()
            for v in radj[u]:
                if comp[v] == -1:
                    comp[v] = count
                    stack.append(v)
        count += 1
    return count, comp


# ---------- 27. Max flow (Dinic) ----------
#   f = Dinic(n); f.add_edge(u, v, cap); f.max_flow(s, t)
# Also: bipartite matching = source -> left (cap 1), left -> right (1), right -> sink (1).
class Dinic:
    def __init__(self, n):
        self.n = n
        self.g = [[] for _ in range(n)]            # edge = [to, cap, index of reverse edge]

    def add_edge(self, u, v, cap):
        self.g[u].append([v, cap, len(self.g[v])])
        self.g[v].append([u, 0, len(self.g[u]) - 1])

    def max_flow(self, s, t):
        flow = 0
        while True:
            level = [-1] * self.n
            level[s] = 0
            q = deque([s])
            while q:
                u = q.popleft()
                for v, cap, _ in self.g[u]:
                    if cap > 0 and level[v] < 0:
                        level[v] = level[u] + 1
                        q.append(v)
            if level[t] < 0:
                return flow
            it = [0] * self.n
            while True:                            # iterative blocking flow
                path, u = [], s
                while u != t:
                    advanced = False
                    while it[u] < len(self.g[u]):
                        v, cap, _ = self.g[u][it[u]]
                        if cap > 0 and level[v] == level[u] + 1:
                            path.append((u, it[u]))
                            u = v
                            advanced = True
                            break
                        it[u] += 1
                    if not advanced:
                        if u == s:
                            break
                        level[u] = -1              # dead end
                        u, _ = path.pop()
                        it[u] += 1
                if u != t:
                    break
                f = min(self.g[x][i][1] for x, i in path)
                for x, i in path:
                    e = self.g[x][i]
                    e[1] -= f
                    self.g[e[0]][e[2]][1] += f
                flow += f


# ---------- 28. Geometry: cross, convex hull, polygon area ----------
def cross(o, a, b):                                # > 0: o->a->b turns left (counter-clockwise)
    return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])


def convex_hull(pts):                              # counter-clockwise, no collinear points
    pts = sorted(set(pts))
    if len(pts) <= 2:
        return pts
    lower, upper = [], []
    for p in pts:
        while len(lower) >= 2 and cross(lower[-2], lower[-1], p) <= 0:
            lower.pop()
        lower.append(p)
    for p in reversed(pts):
        while len(upper) >= 2 and cross(upper[-2], upper[-1], p) <= 0:
            upper.pop()
        upper.append(p)
    return lower[:-1] + upper[:-1]


def area2(poly):                                   # TWICE the area (stays an integer)
    s = 0
    for i in range(len(poly)):
        x1, y1 = poly[i]
        x2, y2 = poly[(i + 1) % len(poly)]
        s += x1 * y2 - x2 * y1
    return abs(s)


# ---------- 29. LIS (strictly increasing), O(n log n) ----------
# For non-decreasing, use bisect_right instead of bisect_left.
from bisect import bisect_left, bisect_right


def lis_length(a):
    tails = []
    for v in a:
        i = bisect_left(tails, v)
        if i == len(tails):
            tails.append(v)
        else:
            tails[i] = v
    return len(tails)


# ---------- 30. Factorization (smallest prime factor) + extended gcd ----------
def spf_sieve(n):                                  # spf[x] = smallest prime factor of x
    spf = list(range(n + 1))
    i = 2
    while i * i <= n:
        if spf[i] == i:
            for j in range(i * i, n + 1, i):
                if spf[j] == j:
                    spf[j] = i
        i += 1
    return spf


def factorize(x, spf):                             # primes with repetition, ascending
    res = []
    while x > 1:
        res.append(spf[x])
        x //= spf[x]
    return res


def ext_gcd(a, b):                                 # returns (g, x, y) with a*x + b*y = g
    x0, y0, x1, y1 = 1, 0, 0, 1
    while b:
        q = a // b
        a, b = b, a - q * b
        x0, x1 = x1, x0 - q * x1
        y0, y1 = y1, y0 - q * y1
    return a, x0, y0


# ======================= MORE =======================

# ---------- 31. NTT polynomial multiply (mod 998244353) ----------
# Coefficients mod 998244353. For exact (non-mod) products the true values must stay below it.
NTT_MOD, NTT_G = 998244353, 3


def _ntt(a, invert):
    n = len(a)
    j = 0
    for i in range(1, n):
        bit = n >> 1
        while j & bit:
            j ^= bit
            bit >>= 1
        j |= bit
        if i < j:
            a[i], a[j] = a[j], a[i]
    length = 2
    while length <= n:
        w = pow(NTT_G, (NTT_MOD - 1) // length, NTT_MOD)
        if invert:
            w = pow(w, NTT_MOD - 2, NTT_MOD)
        half = length // 2
        ws = [1] * half
        for k in range(1, half):
            ws[k] = ws[k - 1] * w % NTT_MOD
        for i in range(0, n, length):
            for k in range(half):
                u = a[i + k]
                v = a[i + k + half] * ws[k] % NTT_MOD
                a[i + k] = (u + v) % NTT_MOD
                a[i + k + half] = (u - v) % NTT_MOD
        length <<= 1
    if invert:
        inv_n = pow(n, NTT_MOD - 2, NTT_MOD)
        for i in range(n):
            a[i] = a[i] * inv_n % NTT_MOD


def ntt_multiply(a, b):                            # coefficient lists, lowest degree first
    if not a or not b:
        return []
    need = len(a) + len(b) - 1
    n = 1
    while n < need:
        n <<= 1
    fa = [x % NTT_MOD for x in a] + [0] * (n - len(a))
    fb = [x % NTT_MOD for x in b] + [0] * (n - len(b))
    _ntt(fa, False); _ntt(fb, False)
    for i in range(n):
        fa[i] = fa[i] * fb[i] % NTT_MOD
    _ntt(fa, True)
    return fa[:need]


# ---------- 32. Bellman-Ford + negative cycle ----------
# edges: (u, v, w), weights may be negative. Returns (dist, has_negative_cycle_reachable_from_s).
def bellman_ford(n, edges, s):
    dist = [INF] * n
    dist[s] = 0
    for _ in range(n - 1):
        changed = False
        for u, v, w in edges:
            if dist[u] != INF and dist[u] + w < dist[v]:
                dist[v] = dist[u] + w
                changed = True
        if not changed:
            return dist, False
    for u, v, w in edges:
        if dist[u] != INF and dist[u] + w < dist[v]:
            return dist, True
    return dist, False


# ---------- 33. Bridges + articulation points (undirected, iterative) ----------
# edges: list of (u, v); parallel edges are handled. Returns (bridge edge indices, cut vertices).
def bridges_and_cuts(n, edges):
    adj = [[] for _ in range(n)]
    for i, (u, v) in enumerate(edges):
        adj[u].append((v, i))
        adj[v].append((u, i))
    tin, low = [-1] * n, [0] * n
    is_cut, bridges, timer = [False] * n, [], 0
    for root in range(n):
        if tin[root] != -1:
            continue
        tin[root] = low[root] = timer; timer += 1
        children = 0
        stack = [[root, -1, 0]]                    # node, parent edge, next neighbour index
        while stack:
            top = stack[-1]
            u, pe, i = top
            if i < len(adj[u]):
                top[2] += 1
                v, eid = adj[u][i]
                if eid == pe:
                    continue
                if tin[v] == -1:
                    tin[v] = low[v] = timer; timer += 1
                    stack.append([v, eid, 0])
                    if u == root:
                        children += 1
                else:
                    low[u] = min(low[u], tin[v])
            else:
                stack.pop()
                if stack:
                    p = stack[-1][0]
                    low[p] = min(low[p], low[u])
                    if low[u] > tin[p]:
                        bridges.append(pe)
                    if p != root and low[u] >= tin[p]:
                        is_cut[p] = True
        if children > 1:
            is_cut[root] = True
    return sorted(bridges), [v for v in range(n) if is_cut[v]]


# ---------- 34. Trie (prefix counting) ----------
class Trie:
    def __init__(self):
        self.nxt = [{}]                            # children of each node
        self.pass_cnt = [0]                        # words passing through the node
        self.end_cnt = [0]                         # words ending at the node

    def insert(self, word):
        x = 0
        for ch in word:
            if ch not in self.nxt[x]:
                self.nxt[x][ch] = len(self.nxt)
                self.nxt.append({}); self.pass_cnt.append(0); self.end_cnt.append(0)
            x = self.nxt[x][ch]
            self.pass_cnt[x] += 1
        self.end_cnt[x] += 1

    def _walk(self, s):
        x = 0
        for ch in s:
            if ch not in self.nxt[x]:
                return -1
            x = self.nxt[x][ch]
        return x

    def count_prefix(self, prefix):                # inserted words starting with prefix
        x = self._walk(prefix)
        return 0 if x < 0 else (self.pass_cnt[x] if prefix else sum(self.end_cnt))

    def count_word(self, word):                    # how many times word was inserted
        x = self._walk(word)
        return 0 if x < 0 else self.end_cnt[x]


# ---------- 35. Coordinate compression ----------
# Returns (ranks, values): ranks[i] = position of a[i] among the sorted distinct values.
def compress(a):
    values = sorted(set(a))
    pos = {v: i for i, v in enumerate(values)}
    return [pos[v] for v in a], values


# ---------- 36. Ternary search (integer argmax of a unimodal f) ----------
# f must strictly increase then strictly decrease. For a minimum, pass lambda x: -f(x).
def ternary_max(lo, hi, f):
    while hi - lo > 2:
        m1 = lo + (hi - lo) // 3
        m2 = hi - (hi - lo) // 3
        if f(m1) < f(m2):
            lo = m1 + 1
        else:
            hi = m2 - 1
    return max(range(lo, hi + 1), key=f)
# Real numbers: repeat 100 times: m1, m2 at thirds; if f(m1) < f(m2): lo = m1 else hi = m2


# ---------- 37. Miller-Rabin primality (deterministic up to 2^64) ----------
def is_prime(n):
    if n < 2:
        return False
    for p in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        if n % p == 0:
            return n == p
    d, r = n - 1, 0
    while d % 2 == 0:
        d //= 2; r += 1
    for a in (2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37):
        x = pow(a, d, n)
        if x == 1 or x == n - 1:
            continue
        for _ in range(r - 1):
            x = x * x % n
            if x == n - 1:
                break
        else:
            return False
    return True


# ---------- 38. CRT (any moduli) + Euler totient ----------
# x = a1 (mod m1) and x = a2 (mod m2). Returns (x, lcm) or None if impossible. Uses ext_gcd (30).
def crt(a1, m1, a2, m2):
    g, p, _ = ext_gcd(m1, m2)
    if (a2 - a1) % g:
        return None
    l = m1 // g * m2
    t = (a2 - a1) // g * p % (m2 // g)
    return (a1 + m1 * t) % l, l
# More equations: fold them, (x, m) = crt(x, m, a3, m3), ...


def totient(n):                                    # count of 1..n coprime with n, O(sqrt n)
    result, p = n, 2
    while p * p <= n:
        if n % p == 0:
            while n % p == 0:
                n //= p
            result -= result // p
        p += 1
    if n > 1:
        result -= result // n
    return result


# ---------- 39. Segment intersection + point in polygon (integer points) ----------
# Uses cross (28). Points are (x, y) tuples.
def on_segment(p, a, b):
    return (cross(a, b, p) == 0 and min(a[0], b[0]) <= p[0] <= max(a[0], b[0])
            and min(a[1], b[1]) <= p[1] <= max(a[1], b[1]))


def segments_intersect(p1, p2, p3, p4):            # True if they share any point (touching counts)
    d1, d2 = cross(p3, p4, p1), cross(p3, p4, p2)
    d3, d4 = cross(p1, p2, p3), cross(p1, p2, p4)
    if ((d1 > 0) != (d2 > 0)) and d1 != 0 and d2 != 0 and ((d3 > 0) != (d4 > 0)) and d3 != 0 and d4 != 0:
        return True
    return on_segment(p1, p3, p4) or on_segment(p2, p3, p4) or on_segment(p3, p1, p2) or on_segment(p4, p1, p2)


def point_in_polygon(p, poly):                     # 1 inside, 0 on the boundary, -1 outside
    inside = False
    for i in range(len(poly)):
        a, b = poly[i], poly[(i + 1) % len(poly)]
        if on_segment(p, a, b):
            return 0
        if (a[1] > p[1]) != (b[1] > p[1]):
            if (cross(a, b, p) > 0) == (b[1] > a[1]):
                inside = not inside
    return 1 if inside else -1


# ---------- 40. Manacher (longest palindrome) ----------
# Returns (start, length) of a longest palindromic substring, in O(n).
def manacher(s):
    n = len(s)
    if n == 0:
        return 0, 0
    d1 = [0] * n                                   # odd palindromes centred at i: radius
    l, r = 0, -1
    for i in range(n):
        k = 1 if i > r else min(d1[l + r - i], r - i + 1)
        while i - k >= 0 and i + k < n and s[i - k] == s[i + k]:
            k += 1
        d1[i] = k
        if i + k - 1 > r:
            l, r = i - k + 1, i + k - 1
    d2 = [0] * n                                   # even palindromes centred between i-1 and i
    l, r = 0, -1
    for i in range(n):
        k = 0 if i > r else min(d2[l + r - i + 1], r - i + 1)
        while i - k - 1 >= 0 and i + k < n and s[i - k - 1] == s[i + k]:
            k += 1
        d2[i] = k
        if i + k - 1 > r:
            l, r = i - k, i + k - 1
    best = (0, 1)
    for i in range(n):
        if 2 * d1[i] - 1 > best[1]:
            best = (i - d1[i] + 1, 2 * d1[i] - 1)
        if 2 * d2[i] > best[1]:
            best = (i - d2[i], 2 * d2[i])
    return best


# ---------- Solution ----------
def main():
    # (For GAME tasks, delete the next line and use section 13 instead.)
    data = sys.stdin.buffer.read().split()
    idx = 0
    n = int(data[idx]); idx += 1
    a = [int(x) for x in data[idx:idx + n]]; idx += n

    out = []
    # solve here

    sys.stdout.write("\n".join(map(str, out)) + "\n")


main()
