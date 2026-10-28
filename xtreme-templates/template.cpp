// IEEEXtreme 20.0 - team template (C++17, GCC)
//
// CONTENTS
//   1  Fast I/O (in main, at the bottom)  9  Sieve + gcd / lcm
//   2  Prefix sums (1D)                  10  Fenwick tree (sums with updates)
//   3  Binary search on answer           11  Topological sort
//   4  BFS on a grid                     12  Kadane + sliding window
//   5  BFS on a graph + path             13  Interactive I/O (GAME tasks)
//   6  Dijkstra                          14  2D prefix sums
//   7  DSU / union-find                  15  Iterative DFS + components
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
//  34  Trie (prefix counting, 'a'-'z')          39  Segment intersection + point in polygon
//  35  Coordinate compression                   40  Manacher (longest palindrome)
#include <bits/stdc++.h>
using namespace std;
using ll = long long;
const ll INF = LLONG_MAX / 4;

// ---------- 2. Prefix sums ----------
// p[i] = a[0] + ... + a[i-1];  sum of a[l..r] (0-indexed, inclusive) = p[r+1] - p[l]
vector<ll> prefix(const vector<ll>& a) {
    vector<ll> p(a.size() + 1, 0);
    for (size_t i = 0; i < a.size(); i++) p[i + 1] = p[i] + a[i];
    return p;
}

// ---------- 3. Binary search on the answer ----------
// Smallest x in [lo, hi] with ok(x) true. ok must be F F F ... T T T.
template <class F>
ll first_true(ll lo, ll hi, F ok) {
    while (lo < hi) {
        ll mid = lo + (hi - lo) / 2;
        if (ok(mid)) hi = mid;
        else lo = mid + 1;
    }
    return lo;
}

// ---------- 4. BFS on a grid ----------
// Distances from (si, sj) over cells that are not '#'. -1 = unreachable.
const int DI[4] = {-1, 1, 0, 0}, DJ[4] = {0, 0, -1, 1};

vector<vector<int>> bfs_grid(const vector<string>& g, int si, int sj) {
    int n = g.size(), m = g[0].size();
    vector<vector<int>> dist(n, vector<int>(m, -1));
    queue<pair<int, int>> q;
    dist[si][sj] = 0;
    q.push({si, sj});
    while (!q.empty()) {
        auto [i, j] = q.front(); q.pop();
        for (int d = 0; d < 4; d++) {
            int ni = i + DI[d], nj = j + DJ[d];
            if (ni >= 0 && ni < n && nj >= 0 && nj < m && g[ni][nj] != '#' && dist[ni][nj] == -1) {
                dist[ni][nj] = dist[i][j] + 1;
                q.push({ni, nj});
            }
        }
    }
    return dist;
}

// ---------- 5. BFS on a graph (unweighted shortest path) ----------
// adj: nodes 0..n-1. Fills dist and par (par[v] = previous node on the path).
void bfs_graph(const vector<vector<int>>& adj, int s, vector<int>& dist, vector<int>& par) {
    int n = adj.size();
    dist.assign(n, -1);
    par.assign(n, -1);
    queue<int> q;
    dist[s] = 0;
    q.push(s);
    while (!q.empty()) {
        int u = q.front(); q.pop();
        for (int v : adj[u]) {
            if (dist[v] == -1) {
                dist[v] = dist[u] + 1;
                par[v] = u;
                q.push(v);
            }
        }
    }
}

vector<int> path_to(const vector<int>& par, int t) {
    vector<int> path;
    for (; t != -1; t = par[t]) path.push_back(t);
    reverse(path.begin(), path.end());
    return path;
}

// ---------- 6. Dijkstra (non-negative weights) ----------
// adj[u] = list of {v, w}
vector<ll> dijkstra(const vector<vector<pair<int, ll>>>& adj, int s) {
    vector<ll> dist(adj.size(), INF);
    priority_queue<pair<ll, int>, vector<pair<ll, int>>, greater<>> pq;
    dist[s] = 0;
    pq.push({0, s});
    while (!pq.empty()) {
        auto [d, u] = pq.top(); pq.pop();
        if (d > dist[u]) continue;  // old entry
        for (auto [v, w] : adj[u]) {
            if (d + w < dist[v]) {
                dist[v] = d + w;
                pq.push({dist[v], v});
            }
        }
    }
    return dist;
}

// ---------- 7. DSU / union-find ----------
struct DSU {
    vector<int> p, sz;
    DSU(int n) : p(n), sz(n, 1) { iota(p.begin(), p.end(), 0); }
    int find(int x) {
        while (p[x] != x) x = p[x] = p[p[x]];  // path halving
        return x;
    }
    bool unite(int a, int b) {
        a = find(a); b = find(b);
        if (a == b) return false;
        if (sz[a] < sz[b]) swap(a, b);
        p[b] = a;
        sz[a] += sz[b];
        return true;
    }
};

// ---------- 8. Modular math (MOD must be prime for the inverse) ----------
const ll MOD = 1'000'000'007LL;

ll power(ll a, ll b, ll m = MOD) {
    ll r = 1;
    a %= m;
    if (a < 0) a += m;
    while (b > 0) {
        if (b & 1) r = r * a % m;
        a = a * a % m;
        b >>= 1;
    }
    return r;
}
ll inverse(ll a) { return power(a, MOD - 2); }

vector<ll> fact, inv_fact;
void build_factorials(int n) {
    fact.assign(n + 1, 1);
    inv_fact.assign(n + 1, 1);
    for (int i = 1; i <= n; i++) fact[i] = fact[i - 1] * i % MOD;
    inv_fact[n] = inverse(fact[n]);
    for (int i = n; i > 0; i--) inv_fact[i - 1] = inv_fact[i] * i % MOD;
}
ll ncr(int n, int r) {
    if (r < 0 || r > n) return 0;
    return fact[n] * inv_fact[r] % MOD * inv_fact[n - r] % MOD;
}

// ---------- 9. Sieve + gcd / lcm ----------
// gcd: std::gcd(a, b);  lcm: a / gcd(a, b) * b  (divide first to avoid overflow)
vector<bool> sieve(int n) {
    vector<bool> is_prime(n + 1, true);
    is_prime[0] = false;
    if (n >= 1) is_prime[1] = false;
    for (ll i = 2; i * i <= n; i++)
        if (is_prime[i])
            for (ll j = i * i; j <= n; j += i) is_prime[j] = false;
    return is_prime;
}

// ---------- 10. Fenwick tree (range sums WITH updates) ----------
// 0-indexed outside, 1-indexed inside.
struct Fenwick {
    int n;
    vector<ll> t;
    Fenwick(int n) : n(n), t(n + 1, 0) {}
    void add(int i, ll delta) {               // a[i] += delta
        for (i++; i <= n; i += i & -i) t[i] += delta;
    }
    ll prefix_sum(int i) {                    // a[0] + ... + a[i-1]
        ll s = 0;
        for (; i > 0; i -= i & -i) s += t[i];
        return s;
    }
    ll range_sum(int l, int r) { return prefix_sum(r + 1) - prefix_sum(l); }  // inclusive
};

// ---------- 11. Topological sort (Kahn) ----------
// adj[u] = nodes that must come AFTER u. Returns an order, or empty if there is a cycle.
vector<int> topo_sort(const vector<vector<int>>& adj) {
    int n = adj.size();
    vector<int> indeg(n, 0), order;
    for (int u = 0; u < n; u++)
        for (int v : adj[u]) indeg[v]++;
    queue<int> q;
    for (int u = 0; u < n; u++)
        if (indeg[u] == 0) q.push(u);
    while (!q.empty()) {
        int u = q.front(); q.pop();
        order.push_back(u);
        for (int v : adj[u])
            if (--indeg[v] == 0) q.push(v);
    }
    if ((int)order.size() != n) order.clear();
    return order;
}

// ---------- 12. Reminders: Kadane + sliding window ----------
ll kadane(const vector<ll>& a) {              // max subarray sum, at least one element
    ll cur = 0, best = LLONG_MIN;
    for (ll v : a) {
        cur = max(v, cur + v);
        best = max(best, cur);
    }
    return best;
}

ll count_window(const vector<ll>& a, ll x) {  // subarrays with sum == x, ALL values positive
    ll s = 0, count = 0;
    size_t left = 0;
    for (size_t right = 0; right < a.size(); right++) {
        s += a[right];
        while (s > x) s -= a[left++];
        if (s == x) count++;
    }
    return count;
}

// ---------- 13. Interactive I/O (GAME tasks) ----------
// The bot answers only after it gets your move, so FLUSH after every line.
// cin.tie(nullptr) in main stops the automatic flush, so say() flushes itself.
// Reading with cin >> x or getline(cin, s) one move at a time is fine.
template <class... T>
void say(const T&... parts) {
    int k = 0;
    ((cout << (k++ ? " " : "") << parts), ...);   // spaces between, none at the end
    cout << '\n' << flush;
}
// Example loop:
//   string msg;
//   while (getline(cin, msg) && msg != "END") say(my_move(msg));

// ---------- 14. 2D prefix sums ----------
// P[i][j] = sum of g[0..i-1][0..j-1]. Rectangle sum, inclusive, 0-indexed.
vector<vector<ll>> prefix2d(const vector<vector<ll>>& g) {
    int n = g.size(), m = g[0].size();
    vector<vector<ll>> P(n + 1, vector<ll>(m + 1, 0));
    for (int i = 0; i < n; i++)
        for (int j = 0; j < m; j++)
            P[i + 1][j + 1] = g[i][j] + P[i + 1][j] + P[i][j + 1] - P[i][j];
    return P;
}
ll rect_sum(const vector<vector<ll>>& P, int r1, int c1, int r2, int c2) {
    return P[r2 + 1][c2 + 1] - P[r1][c2 + 1] - P[r2 + 1][c1] + P[r1][c1];
}

// ---------- 15. Iterative DFS + connected components ----------
// comp[v] = component number of v; returns the number of components.
int components(const vector<vector<int>>& adj, vector<int>& comp) {
    int n = adj.size(), count = 0;
    comp.assign(n, -1);
    for (int s = 0; s < n; s++) {
        if (comp[s] != -1) continue;
        comp[s] = count;
        vector<int> st = {s};
        while (!st.empty()) {
            int u = st.back(); st.pop_back();
            for (int v : adj[u])
                if (comp[v] == -1) { comp[v] = count; st.push_back(v); }
        }
        count++;
    }
    return count;
}

// ======================= ADVANCED =======================

// ---------- 16. Segment tree (point set, range query) ----------
// Any associative op. Example: SegTree<ll> st(a, [](ll x, ll y){ return min(x, y); }, LLONG_MAX);
template <class T>
struct SegTree {
    int n;
    vector<T> t;
    function<T(T, T)> op;
    T e;
    SegTree(const vector<T>& a, function<T(T, T)> op, T e) : n(a.size()), t(2 * a.size(), e), op(op), e(e) {
        for (int i = 0; i < n; i++) t[n + i] = a[i];
        for (int i = n - 1; i > 0; i--) t[i] = op(t[2 * i], t[2 * i + 1]);
    }
    void set(int i, T v) {
        for (t[i += n] = v; i > 1; i /= 2) t[i / 2] = op(t[i & ~1], t[i | 1]);
    }
    T query(int l, int r) {                      // inclusive
        T resl = e, resr = e;
        for (l += n, r += n + 1; l < r; l /= 2, r /= 2) {
            if (l & 1) resl = op(resl, t[l++]);
            if (r & 1) resr = op(t[--r], resr);
        }
        return op(resl, resr);
    }
};

// ---------- 17. Lazy segment tree (range add, range sum + range min) ----------
struct LazySeg {
    int n;
    vector<ll> sm, mn, lz;
    LazySeg(const vector<ll>& a) : n(a.size()), sm(4 * a.size()), mn(4 * a.size()), lz(4 * a.size()) { build(1, 0, n - 1, a); }
    void build(int x, int l, int r, const vector<ll>& a) {
        if (l == r) { sm[x] = mn[x] = a[l]; return; }
        int m = (l + r) / 2;
        build(2 * x, l, m, a); build(2 * x + 1, m + 1, r, a);
        pull(x);
    }
    void pull(int x) { sm[x] = sm[2 * x] + sm[2 * x + 1]; mn[x] = min(mn[2 * x], mn[2 * x + 1]); }
    void apply(int x, int l, int r, ll v) { sm[x] += v * (r - l + 1); mn[x] += v; lz[x] += v; }
    void push(int x, int l, int r) {
        if (lz[x]) {
            int m = (l + r) / 2;
            apply(2 * x, l, m, lz[x]); apply(2 * x + 1, m + 1, r, lz[x]);
            lz[x] = 0;
        }
    }
    void add(int ql, int qr, ll v) { add(ql, qr, v, 1, 0, n - 1); }       // a[ql..qr] += v
    void add(int ql, int qr, ll v, int x, int l, int r) {
        if (qr < l || r < ql) return;
        if (ql <= l && r <= qr) { apply(x, l, r, v); return; }
        push(x, l, r);
        int m = (l + r) / 2;
        add(ql, qr, v, 2 * x, l, m); add(ql, qr, v, 2 * x + 1, m + 1, r);
        pull(x);
    }
    pair<ll, ll> query(int ql, int qr) { return query(ql, qr, 1, 0, n - 1); }  // {sum, min}
    pair<ll, ll> query(int ql, int qr, int x, int l, int r) {
        if (qr < l || r < ql) return {0, LLONG_MAX};
        if (ql <= l && r <= qr) return {sm[x], mn[x]};
        push(x, l, r);
        int m = (l + r) / 2;
        auto a = query(ql, qr, 2 * x, l, m), b = query(ql, qr, 2 * x + 1, m + 1, r);
        return {a.first + b.first, min(a.second, b.second)};
    }
};

// ---------- 18. Sparse table (static range min, O(1) query) ----------
struct SparseMin {
    vector<vector<ll>> t;
    SparseMin(const vector<ll>& a) {
        t.push_back(a);
        for (int j = 1; (1 << j) <= (int)a.size(); j++) {
            int len = a.size() - (1 << j) + 1, h = 1 << (j - 1);
            vector<ll> row(len);
            for (int i = 0; i < len; i++) row[i] = min(t[j - 1][i], t[j - 1][i + h]);
            t.push_back(row);
        }
    }
    ll query(int l, int r) {                     // inclusive
        int k = 31 - __builtin_clz(r - l + 1);
        return min(t[k][l], t[k][r - (1 << k) + 1]);
    }
};

// ---------- 19. LCA (binary lifting) + tree distance ----------
struct LCA {
    int LOG;
    vector<int> depth;
    vector<vector<int>> up;
    LCA(const vector<vector<int>>& adj, int root = 0) {
        int n = adj.size();
        LOG = 1;
        while ((1 << LOG) < n) LOG++;
        depth.assign(n, 0);
        up.assign(LOG, vector<int>(n, root));
        vector<bool> seen(n, false);
        queue<int> q;
        q.push(root); seen[root] = true;
        while (!q.empty()) {
            int u = q.front(); q.pop();
            for (int v : adj[u])
                if (!seen[v]) { seen[v] = true; depth[v] = depth[u] + 1; up[0][v] = u; q.push(v); }
        }
        for (int k = 1; k < LOG; k++)
            for (int v = 0; v < n; v++) up[k][v] = up[k - 1][up[k - 1][v]];
    }
    int lca(int a, int b) {
        if (depth[a] < depth[b]) swap(a, b);
        int diff = depth[a] - depth[b];
        for (int k = 0; diff; k++, diff >>= 1)
            if (diff & 1) a = up[k][a];
        if (a == b) return a;
        for (int k = LOG - 1; k >= 0; k--)
            if (up[k][a] != up[k][b]) { a = up[k][a]; b = up[k][b]; }
        return up[0][a];
    }
    int dist(int a, int b) { return depth[a] + depth[b] - 2 * depth[lca(a, b)]; }
};

// ---------- 20. KMP prefix function + pattern search ----------
vector<int> prefix_function(const string& s) {
    vector<int> pi(s.size(), 0);
    for (size_t i = 1; i < s.size(); i++) {
        int k = pi[i - 1];
        while (k && s[i] != s[k]) k = pi[k - 1];
        if (s[i] == s[k]) k++;
        pi[i] = k;
    }
    return pi;
}
vector<int> find_all(const string& text, const string& pat) {   // start indices
    vector<int> pi = prefix_function(pat), res;
    int k = 0;
    for (int i = 0; i < (int)text.size(); i++) {
        while (k && text[i] != pat[k]) k = pi[k - 1];
        if (text[i] == pat[k]) k++;
        if (k == (int)pat.size()) { res.push_back(i - k + 1); k = pi[k - 1]; }
    }
    return res;
}

// ---------- 21. Z-function ----------
// z[i] = length of the longest common prefix of s and s.substr(i)  (z[0] = n)
vector<int> z_function(const string& s) {
    int n = s.size();
    vector<int> z(n, 0);
    if (n) z[0] = n;
    for (int i = 1, l = 0, r = 0; i < n; i++) {
        if (i < r) z[i] = min(r - i, z[i - l]);
        while (i + z[i] < n && s[z[i]] == s[i + z[i]]) z[i]++;
        if (i + z[i] > r) { l = i; r = i + z[i]; }
    }
    return z;
}

// ---------- 22. String hashing (double mod, random base) ----------
// h.get(l, r) == h.get(l2, r2)  ->  substrings equal (inclusive)
struct StrHash {
    static constexpr ll M1 = 1'000'000'007, M2 = 998'244'353;
    vector<ll> h1, h2, p1, p2;
    StrHash(const string& s) {
        mt19937 rng(chrono::steady_clock::now().time_since_epoch().count());
        ll b = uniform_int_distribution<ll>(256, 100000000)(rng);
        int n = s.size();
        h1.assign(n + 1, 0); h2.assign(n + 1, 0); p1.assign(n + 1, 1); p2.assign(n + 1, 1);
        for (int i = 0; i < n; i++) {
            ll c = (unsigned char)s[i];
            h1[i + 1] = (h1[i] * b + c) % M1;
            h2[i + 1] = (h2[i] * b + c) % M2;
            p1[i + 1] = p1[i] * b % M1;
            p2[i + 1] = p2[i] * b % M2;
        }
    }
    pair<ll, ll> get(int l, int r) {             // inclusive
        r++;
        ll a = (h1[r] - h1[l] * p1[r - l]) % M1, c = (h2[r] - h2[l] * p2[r - l]) % M2;
        return {(a + M1) % M1, (c + M2) % M2};
    }
};

// ---------- 23. Matrix power (mod MOD) ----------
using Mat = vector<vector<ll>>;
Mat mat_mul(const Mat& A, const Mat& B) {
    int n = A.size(), m = B.size(), p = B[0].size();
    Mat C(n, vector<ll>(p, 0));
    for (int i = 0; i < n; i++)
        for (int k = 0; k < m; k++)
            if (A[i][k])
                for (int j = 0; j < p; j++) C[i][j] = (C[i][j] + A[i][k] * B[k][j]) % MOD;
    return C;
}
Mat mat_pow(Mat A, ll e) {
    int n = A.size();
    Mat R(n, vector<ll>(n, 0));
    for (int i = 0; i < n; i++) R[i][i] = 1;
    for (; e > 0; e >>= 1) {
        if (e & 1) R = mat_mul(R, A);
        A = mat_mul(A, A);
    }
    return R;
}
// Fibonacci: F(n) = mat_pow({{1, 1}, {1, 0}}, n)[0][1]

// ---------- 24. Floyd-Warshall (all pairs, n <= ~400) ----------
// d[i][i] = 0, INF where no edge. Changed in place.
void floyd(vector<vector<ll>>& d) {
    int n = d.size();
    for (int k = 0; k < n; k++)
        for (int i = 0; i < n; i++) {
            if (d[i][k] >= INF) continue;
            for (int j = 0; j < n; j++)
                if (d[k][j] < INF && d[i][k] + d[k][j] < d[i][j]) d[i][j] = d[i][k] + d[k][j];
        }
}

// ---------- 25. Kruskal MST (uses DSU, section 7) ----------
// edges: {w, u, v}. Returns the total weight (fewer than n-1 unions = not connected).
ll kruskal(int n, vector<array<ll, 3>> edges) {
    sort(edges.begin(), edges.end());
    DSU ds(n);
    ll total = 0;
    for (auto& [w, u, v] : edges)
        if (ds.unite(u, v)) total += w;
    return total;
}

// ---------- 26. SCC (Kosaraju, iterative) ----------
// Returns the count; comp[v] = component (numbered in topological order of the condensed graph).
int scc(const vector<vector<int>>& adj, vector<int>& comp) {
    int n = adj.size();
    vector<vector<int>> radj(n);
    for (int u = 0; u < n; u++) for (int v : adj[u]) radj[v].push_back(u);
    vector<int> order;
    vector<bool> seen(n, false);
    for (int s = 0; s < n; s++) {
        if (seen[s]) continue;
        seen[s] = true;
        vector<pair<int, int>> st = {{s, 0}};
        while (!st.empty()) {
            auto& [u, i] = st.back();
            if (i < (int)adj[u].size()) {
                int v = adj[u][i++];
                if (!seen[v]) { seen[v] = true; st.push_back({v, 0}); }
            } else {
                order.push_back(u);
                st.pop_back();
            }
        }
    }
    comp.assign(n, -1);
    int count = 0;
    for (int idx = n - 1; idx >= 0; idx--) {
        int s = order[idx];
        if (comp[s] != -1) continue;
        comp[s] = count;
        vector<int> st = {s};
        while (!st.empty()) {
            int u = st.back(); st.pop_back();
            for (int v : radj[u]) if (comp[v] == -1) { comp[v] = count; st.push_back(v); }
        }
        count++;
    }
    return count;
}

// ---------- 27. Max flow (Dinic) ----------
// Dinic f(n); f.add_edge(u, v, cap); f.max_flow(s, t)
// Bipartite matching = source -> left (cap 1), left -> right (1), right -> sink (1).
struct Dinic {
    struct Edge { int to; ll cap; int rev; };
    vector<vector<Edge>> g;
    vector<int> level, it;
    Dinic(int n) : g(n), level(n), it(n) {}
    void add_edge(int u, int v, ll cap) {
        g[u].push_back({v, cap, (int)g[v].size()});
        g[v].push_back({u, 0, (int)g[u].size() - 1});
    }
    bool bfs(int s, int t) {
        fill(level.begin(), level.end(), -1);
        queue<int> q;
        level[s] = 0; q.push(s);
        while (!q.empty()) {
            int u = q.front(); q.pop();
            for (auto& e : g[u])
                if (e.cap > 0 && level[e.to] < 0) { level[e.to] = level[u] + 1; q.push(e.to); }
        }
        return level[t] >= 0;
    }
    ll dfs(int u, int t, ll f) {
        if (u == t) return f;
        for (int& i = it[u]; i < (int)g[u].size(); i++) {
            Edge& e = g[u][i];
            if (e.cap > 0 && level[e.to] == level[u] + 1) {
                ll d = dfs(e.to, t, min(f, e.cap));
                if (d > 0) { e.cap -= d; g[e.to][e.rev].cap += d; return d; }
            }
        }
        return 0;
    }
    ll max_flow(int s, int t) {
        ll flow = 0;
        while (bfs(s, t)) {
            fill(it.begin(), it.end(), 0);
            while (ll f = dfs(s, t, LLONG_MAX)) flow += f;
        }
        return flow;
    }
};

// ---------- 28. Geometry: cross, convex hull, polygon area ----------
using Pt = pair<ll, ll>;
ll cross(Pt o, Pt a, Pt b) {                        // > 0: o->a->b turns left (counter-clockwise)
    return (a.first - o.first) * (b.second - o.second) - (a.second - o.second) * (b.first - o.first);
}
vector<Pt> convex_hull(vector<Pt> pts) {           // counter-clockwise, no collinear points
    sort(pts.begin(), pts.end());
    pts.erase(unique(pts.begin(), pts.end()), pts.end());
    if (pts.size() <= 2) return pts;
    vector<Pt> h(2 * pts.size());
    int k = 0;
    for (size_t i = 0; i < pts.size(); i++) {
        while (k >= 2 && cross(h[k - 2], h[k - 1], pts[i]) <= 0) k--;
        h[k++] = pts[i];
    }
    for (int i = pts.size() - 2, lo = k + 1; i >= 0; i--) {
        while (k >= lo && cross(h[k - 2], h[k - 1], pts[i]) <= 0) k--;
        h[k++] = pts[i];
    }
    h.resize(k - 1);
    return h;
}
ll area2(const vector<Pt>& poly) {                // TWICE the area (stays an integer)
    ll s = 0;
    for (size_t i = 0; i < poly.size(); i++) {
        Pt a = poly[i], b = poly[(i + 1) % poly.size()];
        s += a.first * b.second - b.first * a.second;
    }
    return llabs(s);
}

// ---------- 29. LIS (strictly increasing), O(n log n) ----------
// For non-decreasing, use upper_bound instead of lower_bound.
int lis_length(const vector<ll>& a) {
    vector<ll> tails;
    for (ll v : a) {
        auto it = lower_bound(tails.begin(), tails.end(), v);
        if (it == tails.end()) tails.push_back(v);
        else *it = v;
    }
    return tails.size();
}

// ---------- 30. Factorization (smallest prime factor) + extended gcd ----------
vector<int> spf_sieve(int n) {                   // spf[x] = smallest prime factor of x
    vector<int> spf(n + 1);
    iota(spf.begin(), spf.end(), 0);
    for (ll i = 2; i * i <= n; i++)
        if (spf[i] == i)
            for (ll j = i * i; j <= n; j += i)
                if (spf[j] == j) spf[j] = i;
    return spf;
}
vector<int> factorize(int x, const vector<int>& spf) {   // primes with repetition, ascending
    vector<int> res;
    while (x > 1) { res.push_back(spf[x]); x /= spf[x]; }
    return res;
}
ll ext_gcd(ll a, ll b, ll& x, ll& y) {           // returns g, with a*x + b*y = g
    ll x0 = 1, y0 = 0, x1 = 0, y1 = 1;
    while (b) {
        ll q = a / b, t;
        t = a - q * b; a = b; b = t;
        t = x0 - q * x1; x0 = x1; x1 = t;
        t = y0 - q * y1; y0 = y1; y1 = t;
    }
    x = x0; y = y0;
    return a;
}

// ======================= MORE =======================

// ---------- 31. NTT polynomial multiply (mod 998244353) ----------
// Coefficients mod 998244353. For exact (non-mod) products the true values must stay below it.
const ll NTT_MOD = 998244353, NTT_G = 3;
void ntt(vector<ll>& a, bool invert) {
    int n = a.size();
    for (int i = 1, j = 0; i < n; i++) {
        int bit = n >> 1;
        for (; j & bit; bit >>= 1) j ^= bit;
        j ^= bit;
        if (i < j) swap(a[i], a[j]);
    }
    for (int len = 2; len <= n; len <<= 1) {
        ll w = power(NTT_G, (NTT_MOD - 1) / len, NTT_MOD);
        if (invert) w = power(w, NTT_MOD - 2, NTT_MOD);
        for (int i = 0; i < n; i += len) {
            ll wn = 1;
            for (int k = 0; k < len / 2; k++) {
                ll u = a[i + k], v = a[i + k + len / 2] * wn % NTT_MOD;
                a[i + k] = (u + v) % NTT_MOD;
                a[i + k + len / 2] = (u - v + NTT_MOD) % NTT_MOD;
                wn = wn * w % NTT_MOD;
            }
        }
    }
    if (invert) {
        ll inv_n = power(n, NTT_MOD - 2, NTT_MOD);
        for (ll& x : a) x = x * inv_n % NTT_MOD;
    }
}
vector<ll> ntt_multiply(vector<ll> a, vector<ll> b) {   // lowest degree first
    if (a.empty() || b.empty()) return {};
    int need = a.size() + b.size() - 1, n = 1;
    while (n < need) n <<= 1;
    for (ll& x : a) x = ((x % NTT_MOD) + NTT_MOD) % NTT_MOD;
    for (ll& x : b) x = ((x % NTT_MOD) + NTT_MOD) % NTT_MOD;
    a.resize(n); b.resize(n);
    ntt(a, false); ntt(b, false);
    for (int i = 0; i < n; i++) a[i] = a[i] * b[i] % NTT_MOD;
    ntt(a, true);
    a.resize(need);
    return a;
}

// ---------- 32. Bellman-Ford + negative cycle ----------
// edges: {u, v, w}, weights may be negative. neg = negative cycle reachable from s.
vector<ll> bellman_ford(int n, const vector<array<ll, 3>>& edges, int s, bool& neg) {
    vector<ll> dist(n, INF);
    dist[s] = 0;
    neg = false;
    for (int it = 0; it < n - 1; it++) {
        bool changed = false;
        for (auto& [u, v, w] : edges)
            if (dist[u] < INF && dist[u] + w < dist[v]) { dist[v] = dist[u] + w; changed = true; }
        if (!changed) return dist;
    }
    for (auto& [u, v, w] : edges)
        if (dist[u] < INF && dist[u] + w < dist[v]) { neg = true; break; }
    return dist;
}

// ---------- 33. Bridges + articulation points (undirected, iterative) ----------
// edges: {u, v}; parallel edges are handled. Fills bridge edge indices and cut vertices (sorted).
void bridges_and_cuts(int n, const vector<pair<int, int>>& edges, vector<int>& bridges, vector<int>& cuts) {
    vector<vector<pair<int, int>>> adj(n);
    for (int i = 0; i < (int)edges.size(); i++) {
        adj[edges[i].first].push_back({edges[i].second, i});
        adj[edges[i].second].push_back({edges[i].first, i});
    }
    vector<int> tin(n, -1), low(n, 0);
    vector<bool> is_cut(n, false);
    bridges.clear(); cuts.clear();
    int timer = 0;
    for (int root = 0; root < n; root++) {
        if (tin[root] != -1) continue;
        tin[root] = low[root] = timer++;
        int children = 0;
        vector<array<int, 3>> st = {{root, -1, 0}};   // node, parent edge, next index
        while (!st.empty()) {
            auto& top = st.back();
            int u = top[0], pe = top[1];
            if (top[2] < (int)adj[u].size()) {
                auto [v, eid] = adj[u][top[2]++];
                if (eid == pe) continue;
                if (tin[v] == -1) {
                    tin[v] = low[v] = timer++;
                    st.push_back({v, eid, 0});
                    if (u == root) children++;
                } else low[u] = min(low[u], tin[v]);
            } else {
                st.pop_back();
                if (!st.empty()) {
                    int p = st.back()[0];
                    low[p] = min(low[p], low[u]);
                    if (low[u] > tin[p]) bridges.push_back(pe);
                    if (p != root && low[u] >= tin[p]) is_cut[p] = true;
                }
            }
        }
        if (children > 1) is_cut[root] = true;
    }
    sort(bridges.begin(), bridges.end());
    for (int v = 0; v < n; v++) if (is_cut[v]) cuts.push_back(v);
}

// ---------- 34. Trie (prefix counting, lowercase 'a'-'z') ----------
struct Trie {
    vector<array<int, 26>> nxt;
    vector<int> pass_cnt, end_cnt;
    Trie() { new_node(); }
    int new_node() { nxt.push_back({}); nxt.back().fill(-1); pass_cnt.push_back(0); end_cnt.push_back(0); return nxt.size() - 1; }
    void insert(const string& w) {
        int x = 0;
        for (char ch : w) {
            int c = ch - 'a';
            if (nxt[x][c] == -1) { int y = new_node(); nxt[x][c] = y; }
            x = nxt[x][c];
            pass_cnt[x]++;
        }
        end_cnt[x]++;
    }
    int walk(const string& s) {
        int x = 0;
        for (char ch : s) { x = nxt[x][ch - 'a']; if (x == -1) return -1; }
        return x;
    }
    int count_prefix(const string& p) {      // inserted words starting with p
        if (p.empty()) { int s = 0; for (int e : end_cnt) s += e; return s; }
        int x = walk(p);
        return x < 0 ? 0 : pass_cnt[x];
    }
    int count_word(const string& w) { int x = walk(w); return x < 0 ? 0 : end_cnt[x]; }
};

// ---------- 35. Coordinate compression ----------
// Returns ranks; `values` gets the sorted distinct values (ranks[i] = index of a[i] in values).
vector<int> compress(const vector<ll>& a, vector<ll>& values) {
    values = a;
    sort(values.begin(), values.end());
    values.erase(unique(values.begin(), values.end()), values.end());
    vector<int> r(a.size());
    for (size_t i = 0; i < a.size(); i++) r[i] = lower_bound(values.begin(), values.end(), a[i]) - values.begin();
    return r;
}

// ---------- 36. Ternary search (integer argmax of a unimodal f) ----------
// f must strictly increase then strictly decrease. For a minimum, negate f.
template <class F>
ll ternary_max(ll lo, ll hi, F f) {
    while (hi - lo > 2) {
        ll m1 = lo + (hi - lo) / 3, m2 = hi - (hi - lo) / 3;
        if (f(m1) < f(m2)) lo = m1 + 1;
        else hi = m2 - 1;
    }
    ll best = lo;
    for (ll x = lo + 1; x <= hi; x++) if (f(x) > f(best)) best = x;
    return best;
}
// Real numbers: repeat 100 times: m1, m2 at thirds; if (f(m1) < f(m2)) lo = m1; else hi = m2;

// ---------- 37. Miller-Rabin primality (deterministic up to 2^64) ----------
using ull = unsigned long long;
ull mulmod(ull a, ull b, ull m) { return (unsigned __int128)a * b % m; }
ull powmod(ull a, ull e, ull m) {
    ull r = 1;
    for (a %= m; e; e >>= 1, a = mulmod(a, a, m)) if (e & 1) r = mulmod(r, a, m);
    return r;
}
bool is_prime(ull n) {
    if (n < 2) return false;
    for (ull p : {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37}) if (n % p == 0) return n == p;
    ull d = n - 1; int r = 0;
    while (d % 2 == 0) { d /= 2; r++; }
    for (ull a : {2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37}) {
        ull x = powmod(a, d, n);
        if (x == 1 || x == n - 1) continue;
        bool comp = true;
        for (int i = 0; i < r - 1 && comp; i++) { x = mulmod(x, x, n); if (x == n - 1) comp = false; }
        if (comp) return false;
    }
    return true;
}

// ---------- 38. CRT (any moduli) + Euler totient ----------
// x = a1 (mod m1), x = a2 (mod m2). Returns false if impossible; else x in [0, l), l = lcm.
// Uses ext_gcd (30). Assumes the lcm fits in long long.
bool crt(ll a1, ll m1, ll a2, ll m2, ll& x, ll& l) {
    ll p, q, g = ext_gcd(m1, m2, p, q);
    if ((a2 - a1) % g != 0) return false;
    ll m = m2 / g;
    __int128 t = (__int128)((a2 - a1) / g % m) * (p % m) % m;
    if (t < 0) t += m;
    l = m1 / g * m2;
    __int128 r = ((__int128)a1 + (__int128)m1 * t) % l;
    if (r < 0) r += l;
    x = (ll)r;
    return true;
}
// More equations: fold them, crt(x, l, a3, m3, x, l) ...
ll totient(ll n) {                               // count of 1..n coprime with n, O(sqrt n)
    ll result = n;
    for (ll p = 2; p * p <= n; p++)
        if (n % p == 0) {
            while (n % p == 0) n /= p;
            result -= result / p;
        }
    if (n > 1) result -= result / n;
    return result;
}

// ---------- 39. Segment intersection + point in polygon (integer points) ----------
// Uses Pt and cross (28).
bool on_segment(Pt p, Pt a, Pt b) {
    return cross(a, b, p) == 0 && min(a.first, b.first) <= p.first && p.first <= max(a.first, b.first)
        && min(a.second, b.second) <= p.second && p.second <= max(a.second, b.second);
}
bool segments_intersect(Pt p1, Pt p2, Pt p3, Pt p4) {   // touching counts
    ll d1 = cross(p3, p4, p1), d2 = cross(p3, p4, p2), d3 = cross(p1, p2, p3), d4 = cross(p1, p2, p4);
    if (((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0)) && ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0))) return true;
    return on_segment(p1, p3, p4) || on_segment(p2, p3, p4) || on_segment(p3, p1, p2) || on_segment(p4, p1, p2);
}
int point_in_polygon(Pt p, const vector<Pt>& poly) {    // 1 inside, 0 boundary, -1 outside
    bool inside = false;
    for (size_t i = 0; i < poly.size(); i++) {
        Pt a = poly[i], b = poly[(i + 1) % poly.size()];
        if (on_segment(p, a, b)) return 0;
        if ((a.second > p.second) != (b.second > p.second))
            if ((cross(a, b, p) > 0) == (b.second > a.second)) inside = !inside;
    }
    return inside ? 1 : -1;
}

// ---------- 40. Manacher (longest palindrome) ----------
// Returns {start, length} of a longest palindromic substring, in O(n).
pair<int, int> manacher(const string& s) {
    int n = s.size();
    if (n == 0) return {0, 0};
    vector<int> d1(n), d2(n);
    for (int i = 0, l = 0, r = -1; i < n; i++) {
        int k = i > r ? 1 : min(d1[l + r - i], r - i + 1);
        while (i - k >= 0 && i + k < n && s[i - k] == s[i + k]) k++;
        d1[i] = k;
        if (i + k - 1 > r) { l = i - k + 1; r = i + k - 1; }
    }
    for (int i = 0, l = 0, r = -1; i < n; i++) {
        int k = i > r ? 0 : min(d2[l + r - i + 1], r - i + 1);
        while (i - k - 1 >= 0 && i + k < n && s[i - k - 1] == s[i + k]) k++;
        d2[i] = k;
        if (i + k - 1 > r) { l = i - k; r = i + k - 1; }
    }
    pair<int, int> best = {0, 1};
    for (int i = 0; i < n; i++) {
        if (2 * d1[i] - 1 > best.second) best = {i - d1[i] + 1, 2 * d1[i] - 1};
        if (2 * d2[i] > best.second) best = {i - d2[i], 2 * d2[i]};
    }
    return best;
}

// ---------- 1. Fast I/O + solution ----------
int main() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);

    int n;
    cin >> n;
    vector<ll> a(n);
    for (auto& x : a) cin >> x;

    // solve here; print with '\n', never endl

    return 0;
}
