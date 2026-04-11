#!/usr/bin/env python3
import argparse
import numpy as np
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import matplotlib.ticker as ticker

COLORS = ["#4fc3f7", "#ef9a9a", "#a5d6a7", "#ce93d8", "#ffcc80"]

def parse(path):
    baseline = None
    primes, times = [], []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            left, right = line.split(" - ")
            p, t = int(left), float(right)
            if p == 0:
                baseline = t
            else:
                primes.append(p)
                times.append(t)
    return primes, times, baseline

def min_segment(primes, times, pct=0.01, smooth_window=15):
    arr = np.array(times)
    half = smooth_window // 2
    smoothed = np.array([
        np.median(arr[max(0, i - half): i + half + 1])
        for i in range(len(arr))
    ])
    mn = smoothed.min()
    threshold = mn * (1 + pct)
    min_idx = int(np.argmin(smoothed))
    lo, hi = min_idx, min_idx
    while lo > 0 and smoothed[lo - 1] < threshold:
        lo -= 1
    while hi < len(smoothed) - 1 and smoothed[hi + 1] < threshold:
        hi += 1
    return primes[lo], primes[hi], min(times), min(times) * (1 + pct)

parser = argparse.ArgumentParser(description="Plot execution time measurements")
parser.add_argument("--files", nargs="+", required=True, metavar="FILE[:LABEL]")
parser.add_argument("--output", default="measurements.png")
parser.add_argument("--pct", type=float, default=1.0)
parser.add_argument("--smooth", type=int, default=15)
args = parser.parse_args()

pct = args.pct / 100.0

fig, ax = plt.subplots(figsize=(14, 7))
fig.patch.set_facecolor("#0f1117")
ax.set_facecolor("#0f1117")

ax_r = ax.twinx()
ax_r.set_facecolor("#0f1117")
ax_r.tick_params(colors="#888888", labelsize=8)
for spine in ax_r.spines.values():
    spine.set_edgecolor("#333333")

band_boundaries = []   # (x, color, stagger_index)
right_yticks = []
deferred_min = []

for i, file_spec in enumerate(args.files):
    if ":" in file_spec:
        path, label = file_spec.rsplit(":", 1)
    else:
        path = file_spec
        label = path.split("/")[-1].replace(".measure", "")

    color = COLORS[i % len(COLORS)]
    primes, times, baseline = parse(path)

    ax.plot(primes, times, color=color, linewidth=0.9, alpha=0.85, label=label)

    x0, x1, mn, threshold = min_segment(primes, times, pct=pct, smooth_window=args.smooth)
    ax.axvspan(x0, x1, alpha=0.12, color=color)
    for xb in (x0, x1):
        ax.axvline(xb, color=color, linewidth=0.8, linestyle="--", alpha=0.4)
        band_boundaries.append((xb, color))
    ax.axhline(threshold, color=color, linewidth=0.7, linestyle=":", alpha=0.5)

    if baseline is not None:
        ax.axhline(baseline, color=color, linewidth=1.2, linestyle="--", alpha=0.7,
                   label=f"{label} baseline")
        right_yticks.append((baseline, f"{baseline:.1f}", color))

    right_yticks.append((threshold, f"{threshold:.1f}  (+{args.pct:.4g}%)", color))

    min_idx = times.index(min(times))
    ax.scatter(primes[min_idx], times[min_idx], color=color, s=80, zorder=5)
    deferred_min.append((primes[min_idx], times[min_idx], color, i))

# --- min annotations: direction based on vertical position in plot ---
y_lo, y_hi = ax.get_ylim()
y_range = y_hi - y_lo
for (mx, my, color, i) in deferred_min:
    rel = (my - y_lo) / y_range          # 0 = bottom, 1 = top
    if rel < 0.35:                        # dot near bottom → annotate upward
        xytext = (45, 40)
    else:                                 # dot near top → annotate downward
        xytext = (45, -45)
    ax.annotate(
        f"min  {mx:,} → {my:.1f} ms",
        xy=(mx, my),
        xytext=xytext, textcoords="offset points",
        color=color, fontsize=8,
        arrowprops=dict(arrowstyle="->", color=color, lw=0.8),
    )

# --- band boundary labels: secondary x-axis on top with angled ticks ---
band_boundaries.sort(key=lambda t: t[0])
ax_top = ax.secondary_xaxis('top')
ax_top.set_xlim(ax.get_xlim())
ax_top.set_xticks([xb for xb, _ in band_boundaries])
ax_top.set_xticklabels([f"{int(xb):,}" for xb, _ in band_boundaries],
                        rotation=40, ha="left", fontsize=7)
ax_top.tick_params(length=4, width=0.8, direction="out")
for spine in ax_top.spines.values():
    spine.set_visible(False)
# colour each tick label to match its series
for tick, (_, color) in zip(ax_top.get_xticklabels(), band_boundaries):
    tick.set_color(color)
    tick.set_fontweight("bold")

# --- right Y axis ---
ax_r.set_ylim(ax.get_ylim())
ax_r.set_yticks([t[0] for t in right_yticks])
ax_r.set_yticklabels([t[1] for t in right_yticks])
for tick, (_, _, color) in zip(ax_r.get_yticklabels(), right_yticks):
    tick.set_color(color)
    tick.set_fontweight("bold")
    tick.set_fontsize(8)

ax.set_xlim(left=0)
ax.xaxis.set_major_formatter(ticker.FuncFormatter(lambda v, _: f"{int(v):,}"))
ax.yaxis.set_major_formatter(ticker.FuncFormatter(lambda v, _: f"{v:.1f}"))
ax.tick_params(colors="#888888", labelsize=8)
for spine in ax.spines.values():
    spine.set_edgecolor("#333333")
ax.grid(color="#222222", linewidth=0.5)

ax.set_title("Execution time vs prime", color="white", fontsize=13, pad=12)
ax.set_xlabel("Prime", color="#aaaaaa", fontsize=10)
ax.set_ylabel("Time (ms)", color="#aaaaaa", fontsize=10)
ax_r.set_ylabel("Baseline / threshold (ms)", color="#666666", fontsize=9)

ax.legend(
    facecolor="#1a1a2e", edgecolor="#333333", labelcolor="white", fontsize=9,
    loc="upper center", bbox_to_anchor=(0.5, -0.1),
    ncol=4, borderaxespad=0,
)

plt.tight_layout()
plt.savefig(args.output, dpi=150, bbox_inches="tight")
print(f"Saved → {args.output}")
