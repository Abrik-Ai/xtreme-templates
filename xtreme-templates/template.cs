// IEEEXtreme 20.0 - team template (C#)
// Uses its own heap, so it works even without .NET 6 PriorityQueue.
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
//  34  Trie (prefix counting)                   39  Segment intersection + point in polygon
//  35  Coordinate compression                   40  Manacher (longest palindrome)
//   +  MinHeap class (used by Dijkstra)
using System;
using System.Collections.Generic;
using System.IO;
using System.Text;

public class Program
{
    const long INF = long.MaxValue / 4;

    // ---------- 1. Fast input: read everything, hand out tokens ----------
    // Input is read on the first call only, so GAME tasks (section 13) still work
    // as long as you never call Next().
    static string[] tokens;
    static int pos = 0;
    static string Next()
    {
        if (tokens == null)
            tokens = Console.In.ReadToEnd().Split(new[] { ' ', '\n', '\r', '\t' }, StringSplitOptions.RemoveEmptyEntries);
        return tokens[pos++];
    }
    static int NextInt() { return int.Parse(Next()); }
    static long NextLong() { return long.Parse(Next()); }
    // For text lines instead: string[] lines = Console.In.ReadToEnd().Split('\n');

    // ---------- 2. Prefix sums ----------
    // p[i] = a[0] + ... + a[i-1];  sum of a[l..r] (0-indexed, inclusive) = p[r+1] - p[l]
    static long[] Prefix(long[] a)
    {
        var p = new long[a.Length + 1];
        for (int i = 0; i < a.Length; i++) p[i + 1] = p[i] + a[i];
        return p;
    }

    // ---------- 3. Binary search on the answer ----------
    // Smallest x in [lo, hi] with ok(x) true. ok must be F F F ... T T T.
    static long FirstTrue(long lo, long hi, Func<long, bool> ok)
    {
        while (lo < hi)
        {
            long mid = lo + (hi - lo) / 2;
            if (ok(mid)) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }

    // ---------- 4. BFS on a grid ----------
    // Distances from (si, sj) over cells that are not '#'. -1 = unreachable.
    static readonly int[] DI = { -1, 1, 0, 0 };
    static readonly int[] DJ = { 0, 0, -1, 1 };

    static int[,] BfsGrid(string[] g, int si, int sj)
    {
        int n = g.Length, m = g[0].Length;
        var dist = new int[n, m];
        for (int i = 0; i < n; i++)
            for (int j = 0; j < m; j++) dist[i, j] = -1;
        var q = new Queue<int>();          // cell encoded as i * m + j
        dist[si, sj] = 0;
        q.Enqueue(si * m + sj);
        while (q.Count > 0)
        {
            int c = q.Dequeue(), i = c / m, j = c % m;
            for (int d = 0; d < 4; d++)
            {
                int ni = i + DI[d], nj = j + DJ[d];
                if (ni >= 0 && ni < n && nj >= 0 && nj < m && g[ni][nj] != '#' && dist[ni, nj] == -1)
                {
                    dist[ni, nj] = dist[i, j] + 1;
                    q.Enqueue(ni * m + nj);
                }
            }
        }
        return dist;
    }

    // ---------- 5. BFS on a graph (unweighted shortest path) ----------
    static void BfsGraph(List<int>[] adj, int s, out int[] dist, out int[] par)
    {
        int n = adj.Length;
        dist = new int[n];
        par = new int[n];
        for (int i = 0; i < n; i++) { dist[i] = -1; par[i] = -1; }
        var q = new Queue<int>();
        dist[s] = 0;
        q.Enqueue(s);
        while (q.Count > 0)
        {
            int u = q.Dequeue();
            foreach (int v in adj[u])
            {
                if (dist[v] == -1)
                {
                    dist[v] = dist[u] + 1;
                    par[v] = u;
                    q.Enqueue(v);
                }
            }
        }
    }

    static List<int> PathTo(int[] par, int t)
    {
        var path = new List<int>();
        for (; t != -1; t = par[t]) path.Add(t);
        path.Reverse();
        return path;
    }

    // ---------- Min-heap of (key, value) ----------
    class MinHeap
    {
        List<long> keys = new List<long>();
        List<int> vals = new List<int>();
        public int Count { get { return keys.Count; } }

        void Swap(int a, int b)
        {
            long tk = keys[a]; keys[a] = keys[b]; keys[b] = tk;
            int tv = vals[a]; vals[a] = vals[b]; vals[b] = tv;
        }

        public void Push(long key, int val)
        {
            keys.Add(key); vals.Add(val);
            int i = keys.Count - 1;
            while (i > 0)
            {
                int p = (i - 1) / 2;
                if (keys[p] <= keys[i]) break;
                Swap(p, i);
                i = p;
            }
        }

        public void Pop(out long key, out int val)
        {
            key = keys[0]; val = vals[0];
            int last = keys.Count - 1;
            Swap(0, last);
            keys.RemoveAt(last); vals.RemoveAt(last);
            int i = 0;
            while (true)
            {
                int l = 2 * i + 1, r = l + 1, s = i;
                if (l < keys.Count && keys[l] < keys[s]) s = l;
                if (r < keys.Count && keys[r] < keys[s]) s = r;
                if (s == i) break;
                Swap(s, i);
                i = s;
            }
        }
    }

    // ---------- 6. Dijkstra (non-negative weights) ----------
    // adj[u] = list of edges, each edge = new long[] { v, w }
    static long[] Dijkstra(List<long[]>[] adj, int s)
    {
        var dist = new long[adj.Length];
        for (int i = 0; i < dist.Length; i++) dist[i] = INF;
        var pq = new MinHeap();
        dist[s] = 0;
        pq.Push(0, s);
        while (pq.Count > 0)
        {
            long d; int u;
            pq.Pop(out d, out u);
            if (d > dist[u]) continue;  // old entry
            foreach (var e in adj[u])
            {
                int v = (int)e[0];
                long w = e[1];
                if (d + w < dist[v])
                {
                    dist[v] = d + w;
                    pq.Push(dist[v], v);
                }
            }
        }
        return dist;
    }

    // ---------- 7. DSU / union-find ----------
    class Dsu
    {
        int[] p, sz;
        public Dsu(int n)
        {
            p = new int[n]; sz = new int[n];
            for (int i = 0; i < n; i++) { p[i] = i; sz[i] = 1; }
        }
        public int Find(int x)
        {
            while (p[x] != x) { p[x] = p[p[x]]; x = p[x]; }  // path halving
            return x;
        }
        public bool Union(int a, int b)
        {
            a = Find(a); b = Find(b);
            if (a == b) return false;
            if (sz[a] < sz[b]) { int t = a; a = b; b = t; }
            p[b] = a;
            sz[a] += sz[b];
            return true;
        }
    }

    // ---------- 8. Modular math (MOD must be prime for the inverse) ----------
    const long MOD = 1000000007L;

    static long Power(long a, long b)
    {
        long r = 1;
        a = ((a % MOD) + MOD) % MOD;
        while (b > 0)
        {
            if ((b & 1) == 1) r = r * a % MOD;
            a = a * a % MOD;
            b >>= 1;
        }
        return r;
    }
    static long Inverse(long a) { return Power(a, MOD - 2); }

    static long[] fact, invFact;
    static void BuildFactorials(int n)
    {
        fact = new long[n + 1];
        invFact = new long[n + 1];
        fact[0] = 1;
        for (int i = 1; i <= n; i++) fact[i] = fact[i - 1] * i % MOD;
        invFact[n] = Inverse(fact[n]);
        for (int i = n; i > 0; i--) invFact[i - 1] = invFact[i] * i % MOD;
    }
    static long Ncr(int n, int r)
    {
        if (r < 0 || r > n) return 0;
        return fact[n] * invFact[r] % MOD * invFact[n - r] % MOD;
    }

    // ---------- 9. Sieve + gcd / lcm ----------
    static bool[] Sieve(int n)
    {
        var isPrime = new bool[n + 1];
        for (int i = 2; i <= n; i++) isPrime[i] = true;
        for (long i = 2; i * i <= n; i++)
            if (isPrime[i])
                for (long j = i * i; j <= n; j += i) isPrime[j] = false;
        return isPrime;
    }
    static long Gcd(long a, long b) { while (b != 0) { long t = a % b; a = b; b = t; } return Math.Abs(a); }
    static long Lcm(long a, long b) { return a / Gcd(a, b) * b; }

    // ---------- 10. Fenwick tree (range sums WITH updates) ----------
    // 0-indexed outside, 1-indexed inside.
    class Fenwick
    {
        long[] t;
        public Fenwick(int n) { t = new long[n + 1]; }
        public void Add(int i, long delta)            // a[i] += delta
        {
            for (i++; i < t.Length; i += i & -i) t[i] += delta;
        }
        public long PrefixSum(int i)                  // a[0] + ... + a[i-1]
        {
            long s = 0;
            for (; i > 0; i -= i & -i) s += t[i];
            return s;
        }
        public long RangeSum(int l, int r) { return PrefixSum(r + 1) - PrefixSum(l); }  // inclusive
    }

    // ---------- 11. Topological sort (Kahn) ----------
    // adj[u] = nodes that must come AFTER u. Returns an order, or empty if there is a cycle.
    static List<int> TopoSort(List<int>[] adj)
    {
        int n = adj.Length;
        var indeg = new int[n];
        for (int u = 0; u < n; u++) foreach (int v in adj[u]) indeg[v]++;
        var q = new Queue<int>();
        for (int u = 0; u < n; u++) if (indeg[u] == 0) q.Enqueue(u);
        var order = new List<int>();
        while (q.Count > 0)
        {
            int u = q.Dequeue();
            order.Add(u);
            foreach (int v in adj[u]) if (--indeg[v] == 0) q.Enqueue(v);
        }
        if (order.Count != n) order.Clear();
        return order;
    }

    // ---------- 12. Reminders: Kadane + sliding window ----------
    static long Kadane(long[] a)                      // max subarray sum, at least one element
    {
        long cur = 0, best = long.MinValue;
        foreach (long v in a)
        {
            cur = Math.Max(v, cur + v);
            best = Math.Max(best, cur);
        }
        return best;
    }

    static long CountWindow(long[] a, long x)         // subarrays with sum == x, ALL values positive
    {
        long s = 0, count = 0;
        int left = 0;
        for (int right = 0; right < a.Length; right++)
        {
            s += a[right];
            while (s > x) s -= a[left++];
            if (s == x) count++;
        }
        return count;
    }

    // ---------- 13. Interactive I/O (GAME tasks) ----------
    // The bot answers only after it gets your move: never call Next() (it reads ALL
    // input), read one line at a time, and flush after every move.
    static string ReadLine()                          // null = the judge closed the input
    {
        string s = Console.ReadLine();
        return s == null ? null : s.Trim();
    }
    static void Say(params object[] parts)
    {
        Console.Out.Write(string.Join(" ", parts) + "\n");
        Console.Out.Flush();
    }
    // Example loop:
    //   string msg;
    //   while ((msg = ReadLine()) != null && msg != "END") Say(MyMove(msg));

    // ---------- 14. 2D prefix sums ----------
    // P[i,j] = sum of g[0..i-1, 0..j-1]. Rectangle sum, inclusive, 0-indexed.
    static long[,] Prefix2D(long[,] g)
    {
        int n = g.GetLength(0), m = g.GetLength(1);
        var P = new long[n + 1, m + 1];
        for (int i = 0; i < n; i++)
            for (int j = 0; j < m; j++)
                P[i + 1, j + 1] = g[i, j] + P[i + 1, j] + P[i, j + 1] - P[i, j];
        return P;
    }
    static long RectSum(long[,] P, int r1, int c1, int r2, int c2)
    {
        return P[r2 + 1, c2 + 1] - P[r1, c2 + 1] - P[r2 + 1, c1] + P[r1, c1];
    }

    // ---------- 15. Iterative DFS + connected components ----------
    // comp[v] = component number of v; returns the number of components.
    static int Components(List<int>[] adj, out int[] comp)
    {
        int n = adj.Length, count = 0;
        comp = new int[n];
        for (int i = 0; i < n; i++) comp[i] = -1;
        var stack = new Stack<int>();
        for (int s = 0; s < n; s++)
        {
            if (comp[s] != -1) continue;
            comp[s] = count;
            stack.Push(s);
            while (stack.Count > 0)
            {
                int u = stack.Pop();
                foreach (int v in adj[u])
                    if (comp[v] == -1) { comp[v] = count; stack.Push(v); }
            }
            count++;
        }
        return count;
    }

    // ======================= ADVANCED =======================

    // ---------- 16. Segment tree (point set, range query) ----------
    // Any associative op: new SegTree<long>(a, Math.Min, long.MaxValue)  or  (x, y) => x + y, 0
    class SegTree<T>
    {
        int n; T[] t; Func<T, T, T> op; T e;
        public SegTree(T[] a, Func<T, T, T> op, T e)
        {
            n = a.Length; this.op = op; this.e = e;
            t = new T[2 * n];
            for (int i = 0; i < n; i++) t[n + i] = a[i];
            for (int i = n - 1; i > 0; i--) t[i] = op(t[2 * i], t[2 * i + 1]);
        }
        public void Set(int i, T v)
        {
            i += n; t[i] = v;
            for (i /= 2; i >= 1; i /= 2) t[i] = op(t[2 * i], t[2 * i + 1]);
        }
        public T Query(int l, int r)                  // inclusive
        {
            T resl = e, resr = e;
            for (l += n, r += n + 1; l < r; l /= 2, r /= 2)
            {
                if ((l & 1) == 1) resl = op(resl, t[l++]);
                if ((r & 1) == 1) resr = op(t[--r], resr);
            }
            return op(resl, resr);
        }
    }

    // ---------- 17. Lazy segment tree (range add, range sum + range min) ----------
    class LazySeg
    {
        int n; long[] sm, mn, lz;
        public LazySeg(long[] a)
        {
            n = a.Length; sm = new long[4 * n]; mn = new long[4 * n]; lz = new long[4 * n];
            Build(1, 0, n - 1, a);
        }
        void Build(int x, int l, int r, long[] a)
        {
            if (l == r) { sm[x] = mn[x] = a[l]; return; }
            int m = (l + r) / 2;
            Build(2 * x, l, m, a); Build(2 * x + 1, m + 1, r, a);
            Pull(x);
        }
        void Pull(int x) { sm[x] = sm[2 * x] + sm[2 * x + 1]; mn[x] = Math.Min(mn[2 * x], mn[2 * x + 1]); }
        void Apply(int x, int l, int r, long v) { sm[x] += v * (r - l + 1); mn[x] += v; lz[x] += v; }
        void Push(int x, int l, int r)
        {
            if (lz[x] != 0)
            {
                int m = (l + r) / 2;
                Apply(2 * x, l, m, lz[x]); Apply(2 * x + 1, m + 1, r, lz[x]);
                lz[x] = 0;
            }
        }
        public void Add(int ql, int qr, long v) { Add(ql, qr, v, 1, 0, n - 1); }   // a[ql..qr] += v
        void Add(int ql, int qr, long v, int x, int l, int r)
        {
            if (qr < l || r < ql) return;
            if (ql <= l && r <= qr) { Apply(x, l, r, v); return; }
            Push(x, l, r);
            int m = (l + r) / 2;
            Add(ql, qr, v, 2 * x, l, m); Add(ql, qr, v, 2 * x + 1, m + 1, r);
            Pull(x);
        }
        public long Sum(int ql, int qr) { long s, mi; Query(ql, qr, 1, 0, n - 1, out s, out mi); return s; }
        public long Min(int ql, int qr) { long s, mi; Query(ql, qr, 1, 0, n - 1, out s, out mi); return mi; }
        void Query(int ql, int qr, int x, int l, int r, out long s, out long mi)
        {
            if (qr < l || r < ql) { s = 0; mi = long.MaxValue; return; }
            if (ql <= l && r <= qr) { s = sm[x]; mi = mn[x]; return; }
            Push(x, l, r);
            int m = (l + r) / 2;
            long s1, m1, s2, m2;
            Query(ql, qr, 2 * x, l, m, out s1, out m1);
            Query(ql, qr, 2 * x + 1, m + 1, r, out s2, out m2);
            s = s1 + s2; mi = Math.Min(m1, m2);
        }
    }

    // ---------- 18. Sparse table (static range min, O(1) query) ----------
    class SparseMin
    {
        List<long[]> t = new List<long[]>();
        public SparseMin(long[] a)
        {
            t.Add((long[])a.Clone());
            for (int j = 1; (1 << j) <= a.Length; j++)
            {
                int len = a.Length - (1 << j) + 1, h = 1 << (j - 1);
                var row = new long[len];
                for (int i = 0; i < len; i++) row[i] = Math.Min(t[j - 1][i], t[j - 1][i + h]);
                t.Add(row);
            }
        }
        public long Query(int l, int r)               // inclusive
        {
            int k = 0;
            while ((2 << k) <= r - l + 1) k++;
            return Math.Min(t[k][l], t[k][r - (1 << k) + 1]);
        }
    }

    // ---------- 19. LCA (binary lifting) + tree distance ----------
    class LCA
    {
        int LOG; int[] depth; int[][] up;
        public LCA(List<int>[] adj, int root)
        {
            int n = adj.Length;
            LOG = 1;
            while ((1 << LOG) < n) LOG++;
            depth = new int[n];
            up = new int[LOG][];
            for (int k = 0; k < LOG; k++) { up[k] = new int[n]; for (int v = 0; v < n; v++) up[k][v] = root; }
            var seen = new bool[n];
            var q = new Queue<int>();
            seen[root] = true; q.Enqueue(root);
            while (q.Count > 0)
            {
                int u = q.Dequeue();
                foreach (int v in adj[u])
                    if (!seen[v]) { seen[v] = true; depth[v] = depth[u] + 1; up[0][v] = u; q.Enqueue(v); }
            }
            for (int k = 1; k < LOG; k++)
                for (int v = 0; v < n; v++) up[k][v] = up[k - 1][up[k - 1][v]];
        }
        public int Lca(int a, int b)
        {
            if (depth[a] < depth[b]) { int t = a; a = b; b = t; }
            int diff = depth[a] - depth[b];
            for (int k = 0; diff > 0; k++, diff >>= 1)
                if ((diff & 1) == 1) a = up[k][a];
            if (a == b) return a;
            for (int k = LOG - 1; k >= 0; k--)
                if (up[k][a] != up[k][b]) { a = up[k][a]; b = up[k][b]; }
            return up[0][a];
        }
        public int Dist(int a, int b) { return depth[a] + depth[b] - 2 * depth[Lca(a, b)]; }
    }

    // ---------- 20. KMP prefix function + pattern search ----------
    static int[] PrefixFunction(string s)
    {
        var pi = new int[s.Length];
        for (int i = 1; i < s.Length; i++)
        {
            int k = pi[i - 1];
            while (k > 0 && s[i] != s[k]) k = pi[k - 1];
            if (s[i] == s[k]) k++;
            pi[i] = k;
        }
        return pi;
    }
    static List<int> FindAll(string text, string pat)    // start indices of pat in text
    {
        var pi = PrefixFunction(pat);
        var res = new List<int>();
        int k = 0;
        for (int i = 0; i < text.Length; i++)
        {
            while (k > 0 && text[i] != pat[k]) k = pi[k - 1];
            if (text[i] == pat[k]) k++;
            if (k == pat.Length) { res.Add(i - k + 1); k = pi[k - 1]; }
        }
        return res;
    }

    // ---------- 21. Z-function ----------
    // z[i] = length of the longest common prefix of s and s.Substring(i)  (z[0] = n)
    static int[] ZFunction(string s)
    {
        int n = s.Length;
        var z = new int[n];
        if (n > 0) z[0] = n;
        for (int i = 1, l = 0, r = 0; i < n; i++)
        {
            if (i < r) z[i] = Math.Min(r - i, z[i - l]);
            while (i + z[i] < n && s[z[i]] == s[i + z[i]]) z[i]++;
            if (i + z[i] > r) { l = i; r = i + z[i]; }
        }
        return z;
    }

    // ---------- 22. String hashing (double mod, random base) ----------
    // h.Get(l, r) == h.Get(l2, r2)  ->  substrings equal (inclusive); Get returns one long key
    class StrHash
    {
        const long M1 = 1000000007, M2 = 998244353;
        long[] h1, h2, p1, p2;
        public StrHash(string s)
        {
            long b = new Random().Next(256, 100000000);
            int n = s.Length;
            h1 = new long[n + 1]; h2 = new long[n + 1]; p1 = new long[n + 1]; p2 = new long[n + 1];
            p1[0] = p2[0] = 1;
            for (int i = 0; i < n; i++)
            {
                h1[i + 1] = (h1[i] * b + s[i]) % M1;
                h2[i + 1] = (h2[i] * b + s[i]) % M2;
                p1[i + 1] = p1[i] * b % M1;
                p2[i + 1] = p2[i] * b % M2;
            }
        }
        public long Get(int l, int r)                 // inclusive
        {
            r++;
            long a = ((h1[r] - h1[l] * p1[r - l]) % M1 + M1) % M1;
            long c = ((h2[r] - h2[l] * p2[r - l]) % M2 + M2) % M2;
            return a * M2 + c;                        // unique key for the pair
        }
    }

    // ---------- 23. Matrix power (mod MOD) ----------
    static long[,] MatMul(long[,] A, long[,] B)
    {
        int n = A.GetLength(0), m = B.GetLength(0), p = B.GetLength(1);
        var C = new long[n, p];
        for (int i = 0; i < n; i++)
            for (int k = 0; k < m; k++)
                if (A[i, k] != 0)
                    for (int j = 0; j < p; j++) C[i, j] = (C[i, j] + A[i, k] * B[k, j]) % MOD;
        return C;
    }
    static long[,] MatPow(long[,] A, long e)
    {
        int n = A.GetLength(0);
        var R = new long[n, n];
        for (int i = 0; i < n; i++) R[i, i] = 1;
        for (; e > 0; e >>= 1)
        {
            if ((e & 1) == 1) R = MatMul(R, A);
            A = MatMul(A, A);
        }
        return R;
    }
    // Fibonacci: F(n) = MatPow(new long[,] { { 1, 1 }, { 1, 0 } }, n)[0, 1]

    // ---------- 24. Floyd-Warshall (all pairs, n <= ~400) ----------
    // d[i,i] = 0, INF where no edge. Changed in place.
    static void Floyd(long[,] d)
    {
        int n = d.GetLength(0);
        for (int k = 0; k < n; k++)
            for (int i = 0; i < n; i++)
            {
                if (d[i, k] >= INF) continue;
                for (int j = 0; j < n; j++)
                    if (d[k, j] < INF && d[i, k] + d[k, j] < d[i, j]) d[i, j] = d[i, k] + d[k, j];
            }
    }

    // ---------- 25. Kruskal MST (uses DSU, section 7) ----------
    // edges: each { w, u, v }. Returns the total weight (fewer than n-1 unions = not connected).
    static long Kruskal(int n, List<long[]> edges)
    {
        var sorted = new List<long[]>(edges);
        sorted.Sort((x, y) => x[0].CompareTo(y[0]));
        var ds = new Dsu(n);
        long total = 0;
        foreach (var e in sorted)
            if (ds.Union((int)e[1], (int)e[2])) total += e[0];
        return total;
    }

    // ---------- 26. SCC (Kosaraju, iterative) ----------
    // Returns the count; comp[v] = component (numbered in topological order of the condensed graph).
    static int Scc(List<int>[] adj, out int[] comp)
    {
        int n = adj.Length;
        var radj = new List<int>[n];
        for (int i = 0; i < n; i++) radj[i] = new List<int>();
        for (int u = 0; u < n; u++) foreach (int v in adj[u]) radj[v].Add(u);
        var order = new List<int>();
        var seen = new bool[n];
        var itr = new int[n];
        for (int s = 0; s < n; s++)
        {
            if (seen[s]) continue;
            seen[s] = true;
            var st = new Stack<int>();
            st.Push(s);
            while (st.Count > 0)
            {
                int u = st.Peek();
                if (itr[u] < adj[u].Count)
                {
                    int v = adj[u][itr[u]++];
                    if (!seen[v]) { seen[v] = true; st.Push(v); }
                }
                else { order.Add(u); st.Pop(); }
            }
        }
        comp = new int[n];
        for (int i = 0; i < n; i++) comp[i] = -1;
        int count = 0;
        var st2 = new Stack<int>();
        for (int idx = n - 1; idx >= 0; idx--)
        {
            int s = order[idx];
            if (comp[s] != -1) continue;
            comp[s] = count;
            st2.Push(s);
            while (st2.Count > 0)
            {
                int u = st2.Pop();
                foreach (int v in radj[u]) if (comp[v] == -1) { comp[v] = count; st2.Push(v); }
            }
            count++;
        }
        return count;
    }

    // ---------- 27. Max flow (Dinic) ----------
    // var f = new Dinic(n); f.AddEdge(u, v, cap); f.MaxFlow(s, t)
    // Bipartite matching = source -> left (cap 1), left -> right (1), right -> sink (1).
    class Dinic
    {
        List<int> to = new List<int>();
        List<long> cap = new List<long>();
        List<int>[] g;                                // edge indices; edge e's reverse is e ^ 1
        int[] level, it;
        public Dinic(int n)
        {
            g = new List<int>[n];
            for (int i = 0; i < n; i++) g[i] = new List<int>();
            level = new int[n]; it = new int[n];
        }
        public void AddEdge(int u, int v, long c)
        {
            g[u].Add(to.Count); to.Add(v); cap.Add(c);
            g[v].Add(to.Count); to.Add(u); cap.Add(0);
        }
        bool Bfs(int s, int t)
        {
            for (int i = 0; i < level.Length; i++) level[i] = -1;
            var q = new Queue<int>();
            level[s] = 0; q.Enqueue(s);
            while (q.Count > 0)
            {
                int u = q.Dequeue();
                foreach (int e in g[u])
                    if (cap[e] > 0 && level[to[e]] < 0) { level[to[e]] = level[u] + 1; q.Enqueue(to[e]); }
            }
            return level[t] >= 0;
        }
        long Dfs(int u, int t, long f)
        {
            if (u == t) return f;
            for (; it[u] < g[u].Count; it[u]++)
            {
                int e = g[u][it[u]], v = to[e];
                if (cap[e] > 0 && level[v] == level[u] + 1)
                {
                    long d = Dfs(v, t, Math.Min(f, cap[e]));
                    if (d > 0) { cap[e] -= d; cap[e ^ 1] += d; return d; }
                }
            }
            return 0;
        }
        public long MaxFlow(int s, int t)
        {
            long flow = 0;
            while (Bfs(s, t))
            {
                for (int i = 0; i < it.Length; i++) it[i] = 0;
                long f;
                while ((f = Dfs(s, t, long.MaxValue)) > 0) flow += f;
            }
            return flow;
        }
    }

    // ---------- 28. Geometry: cross, convex hull, polygon area ----------
    // Points are long[] { x, y }.
    static long Cross(long[] o, long[] a, long[] b)   // > 0: o->a->b turns left (counter-clockwise)
    {
        return (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
    }
    static List<long[]> ConvexHull(List<long[]> points)   // counter-clockwise, no collinear points
    {
        var pts = new List<long[]>(points);
        pts.Sort((p, q) => p[0] != q[0] ? p[0].CompareTo(q[0]) : p[1].CompareTo(q[1]));
        var uniq = new List<long[]>();
        foreach (var p in pts)
            if (uniq.Count == 0 || uniq[uniq.Count - 1][0] != p[0] || uniq[uniq.Count - 1][1] != p[1]) uniq.Add(p);
        if (uniq.Count <= 2) return uniq;
        var h = new List<long[]>();
        foreach (var p in uniq)
        {
            while (h.Count >= 2 && Cross(h[h.Count - 2], h[h.Count - 1], p) <= 0) h.RemoveAt(h.Count - 1);
            h.Add(p);
        }
        int lower = h.Count + 1;
        for (int i = uniq.Count - 2; i >= 0; i--)
        {
            var p = uniq[i];
            while (h.Count >= lower && Cross(h[h.Count - 2], h[h.Count - 1], p) <= 0) h.RemoveAt(h.Count - 1);
            h.Add(p);
        }
        h.RemoveAt(h.Count - 1);
        return h;
    }
    static long Area2(List<long[]> poly)              // TWICE the area (stays an integer)
    {
        long s = 0;
        for (int i = 0; i < poly.Count; i++)
        {
            var a = poly[i]; var b = poly[(i + 1) % poly.Count];
            s += a[0] * b[1] - b[0] * a[1];
        }
        return Math.Abs(s);
    }

    // ---------- 29. LIS (strictly increasing), O(n log n) ----------
    // For non-decreasing, change `tails[mid] < v` to `tails[mid] <= v`.
    static int LisLength(long[] a)
    {
        var tails = new List<long>();
        foreach (long v in a)
        {
            int lo = 0, hi = tails.Count;
            while (lo < hi)
            {
                int mid = (lo + hi) / 2;
                if (tails[mid] < v) lo = mid + 1; else hi = mid;
            }
            if (lo == tails.Count) tails.Add(v); else tails[lo] = v;
        }
        return tails.Count;
    }

    // ---------- 30. Factorization (smallest prime factor) + extended gcd ----------
    static int[] SpfSieve(int n)                      // spf[x] = smallest prime factor of x
    {
        var spf = new int[n + 1];
        for (int i = 0; i <= n; i++) spf[i] = i;
        for (long i = 2; i * i <= n; i++)
            if (spf[i] == i)
                for (long j = i * i; j <= n; j += i)
                    if (spf[j] == j) spf[j] = (int)i;
        return spf;
    }
    static List<int> Factorize(int x, int[] spf)      // primes with repetition, ascending
    {
        var res = new List<int>();
        while (x > 1) { res.Add(spf[x]); x /= spf[x]; }
        return res;
    }
    static long ExtGcd(long a, long b, out long x, out long y)   // returns g, with a*x + b*y = g
    {
        long x0 = 1, y0 = 0, x1 = 0, y1 = 1;
        while (b != 0)
        {
            long q = a / b, t;
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
    const long NTT_MOD = 998244353;
    static long PowM(long a, long e, long m)
    {
        long r = 1;
        a = ((a % m) + m) % m;
        for (; e > 0; e >>= 1, a = a * a % m) if ((e & 1) == 1) r = r * a % m;
        return r;
    }
    static void Ntt(long[] a, bool invert)
    {
        int n = a.Length;
        for (int i = 1, j = 0; i < n; i++)
        {
            int bit = n >> 1;
            for (; (j & bit) != 0; bit >>= 1) j ^= bit;
            j ^= bit;
            if (i < j) { long t = a[i]; a[i] = a[j]; a[j] = t; }
        }
        for (int len = 2; len <= n; len <<= 1)
        {
            long w = PowM(3, (NTT_MOD - 1) / len, NTT_MOD);
            if (invert) w = PowM(w, NTT_MOD - 2, NTT_MOD);
            int half = len / 2;
            var ws = new long[half];
            ws[0] = 1;
            for (int k = 1; k < half; k++) ws[k] = ws[k - 1] * w % NTT_MOD;
            for (int i = 0; i < n; i += len)
                for (int k = 0; k < half; k++)
                {
                    long u = a[i + k], v = a[i + k + half] * ws[k] % NTT_MOD;
                    a[i + k] = (u + v) % NTT_MOD;
                    a[i + k + half] = (u - v + NTT_MOD) % NTT_MOD;
                }
        }
        if (invert)
        {
            long invN = PowM(n, NTT_MOD - 2, NTT_MOD);
            for (int i = 0; i < n; i++) a[i] = a[i] * invN % NTT_MOD;
        }
    }
    static long[] NttMultiply(long[] a, long[] b)       // lowest degree first
    {
        if (a.Length == 0 || b.Length == 0) return new long[0];
        int need = a.Length + b.Length - 1, n = 1;
        while (n < need) n <<= 1;
        var fa = new long[n]; var fb = new long[n];
        for (int i = 0; i < a.Length; i++) fa[i] = ((a[i] % NTT_MOD) + NTT_MOD) % NTT_MOD;
        for (int i = 0; i < b.Length; i++) fb[i] = ((b[i] % NTT_MOD) + NTT_MOD) % NTT_MOD;
        Ntt(fa, false); Ntt(fb, false);
        for (int i = 0; i < n; i++) fa[i] = fa[i] * fb[i] % NTT_MOD;
        Ntt(fa, true);
        var res = new long[need];
        Array.Copy(fa, res, need);
        return res;
    }

    // ---------- 32. Bellman-Ford + negative cycle ----------
    // edges: each { u, v, w }, weights may be negative. neg = negative cycle reachable from s.
    static long[] BellmanFord(int n, List<long[]> edges, int s, out bool neg)
    {
        var dist = new long[n];
        for (int i = 0; i < n; i++) dist[i] = INF;
        dist[s] = 0;
        neg = false;
        for (int it = 0; it < n - 1; it++)
        {
            bool changed = false;
            foreach (var e in edges)
                if (dist[e[0]] < INF && dist[e[0]] + e[2] < dist[e[1]]) { dist[e[1]] = dist[e[0]] + e[2]; changed = true; }
            if (!changed) return dist;
        }
        foreach (var e in edges)
            if (dist[e[0]] < INF && dist[e[0]] + e[2] < dist[e[1]]) { neg = true; break; }
        return dist;
    }

    // ---------- 33. Bridges + articulation points (undirected, iterative) ----------
    // edges: each { u, v }; parallel edges are handled. Fills sorted bridge indices and cut vertices.
    static void BridgesAndCuts(int n, List<int[]> edges, out List<int> bridges, out List<int> cuts)
    {
        var adj = new List<int[]>[n];
        for (int i = 0; i < n; i++) adj[i] = new List<int[]>();
        for (int i = 0; i < edges.Count; i++)
        {
            adj[edges[i][0]].Add(new[] { edges[i][1], i });
            adj[edges[i][1]].Add(new[] { edges[i][0], i });
        }
        var tin = new int[n]; var low = new int[n]; var isCut = new bool[n];
        for (int i = 0; i < n; i++) tin[i] = -1;
        bridges = new List<int>(); cuts = new List<int>();
        int timer = 0;
        for (int root = 0; root < n; root++)
        {
            if (tin[root] != -1) continue;
            tin[root] = low[root] = timer++;
            int children = 0;
            var st = new List<int[]> { new[] { root, -1, 0 } };   // node, parent edge, next index
            while (st.Count > 0)
            {
                var top = st[st.Count - 1];
                int u = top[0], pe = top[1];
                if (top[2] < adj[u].Count)
                {
                    var ed = adj[u][top[2]++];
                    int v = ed[0], eid = ed[1];
                    if (eid == pe) continue;
                    if (tin[v] == -1)
                    {
                        tin[v] = low[v] = timer++;
                        st.Add(new[] { v, eid, 0 });
                        if (u == root) children++;
                    }
                    else low[u] = Math.Min(low[u], tin[v]);
                }
                else
                {
                    st.RemoveAt(st.Count - 1);
                    if (st.Count > 0)
                    {
                        int p = st[st.Count - 1][0];
                        low[p] = Math.Min(low[p], low[u]);
                        if (low[u] > tin[p]) bridges.Add(pe);
                        if (p != root && low[u] >= tin[p]) isCut[p] = true;
                    }
                }
            }
            if (children > 1) isCut[root] = true;
        }
        bridges.Sort();
        for (int v = 0; v < n; v++) if (isCut[v]) cuts.Add(v);
    }

    // ---------- 34. Trie (prefix counting) ----------
    class Trie
    {
        List<Dictionary<char, int>> nxt = new List<Dictionary<char, int>> { new Dictionary<char, int>() };
        List<int> pass = new List<int> { 0 }, end = new List<int> { 0 };
        public void Insert(string w)
        {
            int x = 0;
            foreach (char ch in w)
            {
                int y;
                if (!nxt[x].TryGetValue(ch, out y))
                {
                    y = nxt.Count; nxt[x][ch] = y;
                    nxt.Add(new Dictionary<char, int>()); pass.Add(0); end.Add(0);
                }
                x = y;
                pass[x]++;
            }
            end[x]++;
        }
        int Walk(string s)
        {
            int x = 0;
            foreach (char ch in s) if (!nxt[x].TryGetValue(ch, out x)) return -1;
            return x;
        }
        public int CountPrefix(string p)              // inserted words starting with p
        {
            if (p.Length == 0) { int s = 0; foreach (int e in end) s += e; return s; }
            int x = Walk(p);
            return x < 0 ? 0 : pass[x];
        }
        public int CountWord(string w) { int x = Walk(w); return x < 0 ? 0 : end[x]; }
    }

    // ---------- 35. Coordinate compression ----------
    // Returns ranks; values = sorted distinct values (ranks[i] = index of a[i] in values).
    static int[] Compress(long[] a, out long[] values)
    {
        var set = new SortedSet<long>(a);
        values = new long[set.Count];
        set.CopyTo(values);
        var r = new int[a.Length];
        for (int i = 0; i < a.Length; i++) r[i] = Array.BinarySearch(values, a[i]);
        return r;
    }

    // ---------- 36. Ternary search (integer argmax of a unimodal f) ----------
    // f must strictly increase then strictly decrease. For a minimum, negate f.
    static long TernaryMax(long lo, long hi, Func<long, long> f)
    {
        while (hi - lo > 2)
        {
            long m1 = lo + (hi - lo) / 3, m2 = hi - (hi - lo) / 3;
            if (f(m1) < f(m2)) lo = m1 + 1; else hi = m2 - 1;
        }
        long best = lo;
        for (long x = lo + 1; x <= hi; x++) if (f(x) > f(best)) best = x;
        return best;
    }
    // Real numbers: repeat 100 times: m1, m2 at thirds; if (f(m1) < f(m2)) lo = m1; else hi = m2;

    // ---------- 37. Miller-Rabin primality (deterministic up to 2^64) ----------
    // MulMod avoids 128-bit math (works on any .NET); fast path when m < 2^32.
    static ulong AddMod(ulong a, ulong b, ulong m) { return a >= m - b ? a - (m - b) : a + b; }
    static ulong MulMod(ulong a, ulong b, ulong m)
    {
        a %= m; b %= m;
        if (m <= uint.MaxValue) return a * b % m;
        ulong r = 0;
        for (; b > 0; b >>= 1, a = AddMod(a, a, m)) if ((b & 1) == 1) r = AddMod(r, a, m);
        return r;
    }
    static ulong PowMod(ulong a, ulong e, ulong m)
    {
        ulong r = 1 % m;
        for (a %= m; e > 0; e >>= 1, a = MulMod(a, a, m)) if ((e & 1) == 1) r = MulMod(r, a, m);
        return r;
    }
    static bool IsPrime(ulong n)
    {
        if (n < 2) return false;
        ulong[] bases = { 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37 };
        foreach (ulong p in bases) if (n % p == 0) return n == p;
        ulong d = n - 1; int r = 0;
        while (d % 2 == 0) { d /= 2; r++; }
        foreach (ulong a in bases)
        {
            ulong x = PowMod(a, d, n);
            if (x == 1 || x == n - 1) continue;
            bool comp = true;
            for (int i = 0; i < r - 1 && comp; i++) { x = MulMod(x, x, n); if (x == n - 1) comp = false; }
            if (comp) return false;
        }
        return true;
    }

    // ---------- 38. CRT (any moduli) + Euler totient ----------
    // x = a1 (mod m1), x = a2 (mod m2). Returns false if impossible; else x in [0, l), l = lcm.
    // Uses ExtGcd (30). Assumes the lcm fits in a long.
    static bool Crt(long a1, long m1, long a2, long m2, out long x, out long l)
    {
        long p, q;
        long g = ExtGcd(m1, m2, out p, out q);
        x = 0; l = 0;
        if ((a2 - a1) % g != 0) return false;
        long m = m2 / g;
        long diff = (a2 - a1) / g % m; if (diff < 0) diff += m;
        long pp = p % m; if (pp < 0) pp += m;
        ulong t = MulMod((ulong)diff, (ulong)pp, (ulong)m);
        ulong L = (ulong)(m1 / g) * (ulong)m2;
        ulong a = (ulong)(((a1 % m1) + m1) % m1);
        x = (long)((a + MulMod((ulong)m1, t, L)) % L);
        l = (long)L;
        return true;
    }
    static long Totient(long n)                       // count of 1..n coprime with n, O(sqrt n)
    {
        long result = n;
        for (long p = 2; p * p <= n; p++)
            if (n % p == 0)
            {
                while (n % p == 0) n /= p;
                result -= result / p;
            }
        if (n > 1) result -= result / n;
        return result;
    }

    // ---------- 39. Segment intersection + point in polygon (integer points { x, y }) ----------
    // Uses Cross (28).
    static bool OnSegment(long[] p, long[] a, long[] b)
    {
        return Cross(a, b, p) == 0 && Math.Min(a[0], b[0]) <= p[0] && p[0] <= Math.Max(a[0], b[0])
            && Math.Min(a[1], b[1]) <= p[1] && p[1] <= Math.Max(a[1], b[1]);
    }
    static bool SegmentsIntersect(long[] p1, long[] p2, long[] p3, long[] p4)   // touching counts
    {
        long d1 = Cross(p3, p4, p1), d2 = Cross(p3, p4, p2), d3 = Cross(p1, p2, p3), d4 = Cross(p1, p2, p4);
        if (((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0)) && ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0))) return true;
        return OnSegment(p1, p3, p4) || OnSegment(p2, p3, p4) || OnSegment(p3, p1, p2) || OnSegment(p4, p1, p2);
    }
    static int PointInPolygon(long[] p, List<long[]> poly)   // 1 inside, 0 boundary, -1 outside
    {
        bool inside = false;
        for (int i = 0; i < poly.Count; i++)
        {
            var a = poly[i]; var b = poly[(i + 1) % poly.Count];
            if (OnSegment(p, a, b)) return 0;
            if ((a[1] > p[1]) != (b[1] > p[1]) && (Cross(a, b, p) > 0) == (b[1] > a[1])) inside = !inside;
        }
        return inside ? 1 : -1;
    }

    // ---------- 40. Manacher (longest palindrome) ----------
    // Returns { start, length } of a longest palindromic substring, in O(n).
    static int[] Manacher(string s)
    {
        int n = s.Length;
        if (n == 0) return new[] { 0, 0 };
        var d1 = new int[n]; var d2 = new int[n];
        for (int i = 0, l = 0, r = -1; i < n; i++)
        {
            int k = i > r ? 1 : Math.Min(d1[l + r - i], r - i + 1);
            while (i - k >= 0 && i + k < n && s[i - k] == s[i + k]) k++;
            d1[i] = k;
            if (i + k - 1 > r) { l = i - k + 1; r = i + k - 1; }
        }
        for (int i = 0, l = 0, r = -1; i < n; i++)
        {
            int k = i > r ? 0 : Math.Min(d2[l + r - i + 1], r - i + 1);
            while (i - k - 1 >= 0 && i + k < n && s[i - k - 1] == s[i + k]) k++;
            d2[i] = k;
            if (i + k - 1 > r) { l = i - k; r = i + k - 1; }
        }
        int bs = 0, bl = 1;
        for (int i = 0; i < n; i++)
        {
            if (2 * d1[i] - 1 > bl) { bs = i - d1[i] + 1; bl = 2 * d1[i] - 1; }
            if (2 * d2[i] > bl) { bs = i - d2[i]; bl = 2 * d2[i]; }
        }
        return new[] { bs, bl };
    }

    // ---------- Solution ----------
    public static void Main()
    {
        var sb = new StringBuilder();   // sb.Append(x).Append('\n');

        int n = NextInt();
        var a = new long[n];
        for (int i = 0; i < n; i++) a[i] = NextLong();

        // solve here

        Console.Out.Write(sb.ToString());
    }
}
