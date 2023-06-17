import re
import subprocess

r = subprocess.run(["xrandr"], capture_output=True).stdout.decode()
m = subprocess.run(["xrandr", "--listmonitors"], capture_output=True).stdout.decode()

r = r.split("\n")[1:-1]

def get_output_name(line: str) -> str:
    return line.split(" ")[0]

data = [
    {
        "name": get_output_name(i),
        "modes": [],
        "preferred": None,
        "selected": None,
        "pos": pos
    } for pos, i in enumerate(r) if i[0] != " "
]

starts = [i["pos"] for i in data]
cur = 0

for pos, line in enumerate(r):
    if pos not in starts:
        res = re.search(r"(?P<width>\d+)x(?P<height>\d+)", line)
        ref = re.findall(r"(\d+\.\d+)(\*|\s*)(\+|\s*)", line.strip())
        data[cur - 1]["modes"].append({
            "width": res.group("width"),
            "height": res.group("height"),
            "rates": [i[0] for i in ref],
        })
        sel = [p for p, i in enumerate(ref) if i[1] == "*"]
        pref = [p for p, i in enumerate(ref) if i[2] == "+"]
        if len(sel) > 0:
            data[cur - 1]["selected"] = [len(data[cur - 1]["modes"]) - 1, sel]
        if len(pref) > 0:
            data[cur - 1]["preferred"] = [len(data[cur - 1]["modes"]) - 1, pref]
    else:
        cur += 1

for d in data:
    del d["pos"]

print(data)
