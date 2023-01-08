import os
import shutil
import subprocess

urls = ["https://github.com/JulianKnodt/regular_spatial_hash"]
tests = [
  "calc_test.rs",
  "2D_spatial_query_test.rs",
]

def run_checks(url):
  os.system(f"git clone {url}")
  folder = url.split("/")[-1]

  examples_folder = os.path.join(folder, "examples")
  results = { t: False for t in tests }

  if os.path.exists(examples_folder):
    os.chdir(folder)
    for t in tests:
      try:
        test_file = os.path.join("examples", t)
        if not os.path.exists(test_file): continue

        out = subprocess.check_output(["cargo", "run", "--example", os.path.splitext(t)[0], "--release"])
        print(out)
      except Exception:
        continue
    os.chdir("..")

  shutil.rmtree(folder)

for url in urls: run_checks(url)
