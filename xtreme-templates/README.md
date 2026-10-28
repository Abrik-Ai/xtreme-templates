# IEEEXtreme 20.0 templates: how to choose

Five files, one per language, all with the same 40 numbered sections:
`template.py`, `template.cpp`, `template.js`, `template.rs`, `template.cs`.

Plus `cheat_sheet.txt` (about 7 KB, plain text): time limits by input size, which section
to use for which problem, formulas, a pitfall checklist, input shapes, a stress-test loop
and contest strategy. Upload it as an extra file so you can read it in locked tasks.

**Size limit: 100 KB for your whole upload.** Each full file is 34–44 KB, so upload only the
languages your team uses, and delete sections you don't want.

| File | Full size |
| --- | --- |
| template.py | 34 KB |
| template.cpp | 34 KB |
| template.js | 34 KB |
| template.rs | 41 KB |
| template.cs | 44 KB |

Python + C++ in full = about 68 KB, which fits.

## Conventions (all languages)

- Positions start at **0**. Ranges `query(l, r)` include **both ends**.
- Graphs: nodes `0..n-1`, adjacency lists.
- Points: `(x, y)` with integer coordinates. Areas are returned **doubled** so they stay integers.
- `MOD = 1_000_000_007` unless the section says otherwise (NTT uses 998244353).

## Priority

- **A** = keep: comes up often.
- **B** = useful in medium problems.
- **C** = only for hard problems.

## All sections

| # | Section | Use it when… | Priority | Needs section |
| --- | --- | --- | --- | --- |
| 1 | Fast input / output | always | A | – |
| 2 | Prefix sums (1D) | many range-sum questions, no updates | A | – |
| 3 | Binary search on answer | "smallest X such that…", like Factory Machines | A | – |
| 4 | BFS on a grid | shortest path in a maze, flood fill | A | JS: Queue |
| 5 | BFS on a graph + path | unweighted shortest path, print the route | A | JS: Queue |
| 6 | Dijkstra | weighted shortest path, weights ≥ 0 | A | JS: Queue/MinHeap · C#: MinHeap |
| 7 | DSU / union-find | joining groups, connectivity | A | – |
| 8 | Modular math | "answer mod 10⁹+7", nCr | A | – |
| 9 | Sieve + gcd / lcm | primes up to ~10⁷, divisibility | A | – |
| 10 | Fenwick tree | range sums **with** updates | B | – |
| 11 | Topological sort | order with dependencies, Course Schedule | B | JS: Queue |
| 12 | Kadane + sliding window | max subarray; count subarrays with sum x (positive values) | A | – |
| 13 | Interactive I/O | **game tasks** (talking to a bot) | A | – |
| 14 | 2D prefix sums | rectangle sums in a grid, Forest Queries | B | – |
| 15 | Iterative DFS + components | counting connected groups | A | – |
| 16 | Segment tree | range min/max/sum **with** point updates | B | – |
| 17 | Lazy segment tree | add to a whole range, then query sums/mins | C | – |
| 18 | Sparse table | many range-min queries, no updates | B | – |
| 19 | LCA + tree distance | distance between two nodes of a tree | B | JS: Queue |
| 20 | KMP | find a pattern in a long text | B | – |
| 21 | Z-function | prefix matches, periods of a string | C | – |
| 22 | String hashing | compare substrings fast | B | JS: mulm (top of ADVANCED) |
| 23 | Matrix power | recurrences with huge n (Fibonacci of 10¹⁸) | C | 8 (MOD) · JS: mulm |
| 24 | Floyd–Warshall | all-pairs shortest paths, n ≤ ~400 | B | Py: INF (in 6) |
| 25 | Kruskal MST | cheapest way to connect everything | B | 7 |
| 26 | SCC | groups where everyone reaches everyone (directed) | C | – |
| 27 | Max flow (Dinic) | flows, bipartite matching, min cut | C | JS: Queue |
| 28 | Geometry: hull, area | convex hull, polygon area | B | – |
| 29 | LIS | longest increasing subsequence, n up to 2·10⁵ | B | – |
| 30 | Factorization + ext. gcd | prime factors fast; a·x + b·y = g | B | – |
| 31 | NTT | multiply polynomials, count sums of pairs | C | C++: 8 (power) · JS: mulm |
| 32 | Bellman–Ford | negative weights, detect negative cycles | B | Py: INF (in 6) |
| 33 | Bridges + cut vertices | "which road/city disconnects the network?" | C | – |
| 34 | Trie | many words, prefix questions | B | – |
| 35 | Coordinate compression | huge values used as indices | B | – |
| 36 | Ternary search | max/min of a function that rises then falls | B | – |
| 37 | Miller–Rabin | is a number up to 10¹⁸ prime? | B | – |
| 38 | CRT + totient | systems of remainders; coprime counting | C | 30 (ext. gcd) · C#: 37 (MulMod) |
| 39 | Segments + point in polygon | do segments cross? is a point inside? | C | 28 (cross; C++/Rust: Pt type) |
| 40 | Manacher | longest palindrome in a string | C | – |

"Needs section" lists what you must **keep** if you keep that section. If a line names a language
("JS: …", "Py: …", "C#: …"), the dependency applies only in that language. The helper classes Queue and MinHeap
(JS) and MinHeap (C#) are near the top of those files.

## Suggested trims

- **Small set (about 9–13 KB per language):** sections 1–15.
- **Balanced set:** 1–16, 18–20, 22, 24, 25, 28–30, 32, 34–37.
- **Everything:** fine if you upload only one or two languages.

## Game tasks (section 13)

Never read all input at once in a game task. Read one line, answer, flush, repeat:

- **Python / Rust:** delete the fast-input line at the start of `main`.
- **JS / C#:** fast input is lazy, so it's safe as long as you don't call `next()` / `Next()`.
