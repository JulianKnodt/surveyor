files = [
  "header.md",
  "test_info.md",
]

output = "index.md"

out = []

for f in files:
  with open(f) as file:
    out.append(file.read())

out_md = "\n".join(out)

with open(output, "w") as out:
  out.write(out_md)
