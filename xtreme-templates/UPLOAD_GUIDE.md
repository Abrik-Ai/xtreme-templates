# How to use these templates for IEEEXtreme 20.0

**Upload deadline: Thu 29 Oct, 23:59 UTC = Fri 30 Oct, 01:59 Cyprus time.**
Contest: Sat 31 Oct, 00:00 UTC (02:00 Cyprus time), for 24 hours.

---

## 0. Read this first: plagiarism

IEEE checks every submission with plagiarism-detection tools and can disqualify teams
**during or after** the contest. If two teams' code is "unreasonably similar", **all teams
involved** can be disqualified.

These templates are a **starting point**, not something to upload as-is:

- **Rewrite what you keep in your own style.** Renaming variables is not enough: the tools
  compare code structure, not names. Rewrite the short sections (fast input, BFS, binary
  search, DSU, prefix sums) from scratch.
- **Delete what you won't use.** Less shared code means less risk.
- **Don't share your final version with other teams.** No code or solution talk between
  teams during the 24 hours.
- **No AI in locked-workspace tasks.** In game tasks, use AI for ideas only, and write the
  code yourselves.
- **Cite every outside source** in a code comment with its URL, including your template:
  `# From our team template (uploaded before the 29 Oct deadline)`

---

## 1. Get the files

### Option A: git

```bash
git clone https://github.com/Abrik-Ai/xtreme-templates.git
cd xtreme-templates
```

If the repo is private, accept the GitHub invitation first (check your email or
github.com/notifications), and log in to GitHub on that computer.

### Option B: no git

On the repo page, click **Code → Download ZIP**, then unzip it.

### What's inside

| File | What it is |
| --- | --- |
| `template.py` / `.cpp` / `.js` / `.rs` / `.cs` | 40 sections per language |
| `README.md` | what each section is for, its priority (A/B/C) and its dependencies |
| `cheat_sheet.txt` | time limits, which section to use, formulas, pitfalls, contest strategy |
| `UPLOAD_GUIDE.md` | this file |

---

## 2. Choose your languages and sections

1. Pick the **1–2 languages** your team will actually use. The upload limit is **100 KB in
   total**, and each full template file is 34–44 KB.
2. Open `README.md` and decide which sections to keep:
   - **A** = keep. **B** = keep if you understand it. **C** = only for hard problems.
   - Check the **"Needs section"** column before deleting anything. Example: Kruskal (25)
     needs DSU (7), and CRT (38) needs extended gcd (30).
3. Delete everything else.

---

## 3. Rewrite and test on your own computer

Rewrite the sections you kept, then test each one on a real problem:

| Section | Test on (cses.fi/problemset) |
| --- | --- |
| 1 fast I/O | any problem |
| 3 binary search on answer | Factory Machines |
| 4 grid BFS | Counting Rooms, Labyrinth |
| 5 graph BFS + path | Message Route |
| 6 Dijkstra | Shortest Routes I |
| 7 DSU | Building Roads |
| 2 / 12 prefix sums, sliding window | Static Range Sum Queries, Subarray Sums I |

How to run each language (with the example input saved as `in.txt`):

```bash
python3 template.py < in.txt                                   # Python (submit with PyPy3)
g++ -std=c++17 -O2 -o sol template.cpp && ./sol < in.txt       # C++
node template.js < in.txt                                      # JavaScript
rustc -O -o sol template.rs && ./sol < in.txt                  # Rust
mcs template.cs && mono template.exe < in.txt                  # C# (or use dotnet)
```

---

## 4. Upload to CS Academy

The official announcement says you can do three things at
**https://csacademy.com/workspace/template**:

| What | What to put there |
| --- | --- |
| **Starting code**: loads automatically in every new task | only the short part: fast I/O + the `main` skeleton (section 1). Keep it small. |
| **Code library**: up to **100 KB** in total | your rewritten template file(s) |
| **Extra files**: the **only** code you can copy from in locked tasks | your template file(s) + `cheat_sheet.txt` |

Steps:

1. **Log in to CS Academy** with the account you will use in the contest.
   - If you're not sure which account will be used, **do it for every team member's account**.
2. Open **csacademy.com/workspace/template**.
3. Set the **starting code** for each language you'll use.
4. Add your **template file(s)** and **cheat_sheet.txt** as extra files.
5. Check that the total is **under 100 KB**.
6. Finish **before the deadline**. Changes cannot be made once a task has started.

> The exact buttons on that page may differ from this description. If something is unclear,
> ask your proctor or email **IEEEXtreme@ieee.org** before the deadline, not on contest day.

---

## 5. Check that it works (do this before the deadline)

1. Open a practice task: **csacademy.com/ieeextreme-practice/tasks**.
2. Check that your **starting code appears** automatically.
3. Check that your **extra files are visible** and that you can **copy from them** into the editor.
4. Submit one easy problem using your template, to be sure it compiles and runs on the judge.

---

## 6. Contest day checklist

- [ ] Template uploaded and tested, in every team member's account if unsure
- [ ] Local copy of the files on each laptop, in case the internet is slow
- [ ] Proctor of Record confirmed by email
- [ ] Logged in before 02:00 Cyprus time (00:00 UTC)
- [ ] Chargers, food, a place to sleep in shifts
- [ ] Everyone agrees: no code shared between teams, no AI in locked tasks, cite every source

Good luck!
