import os
import shutil
import subprocess
import json

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

        child = subprocess.Popen(["cargo", "run", "--example", os.path.splitext(t)[0]])#, "--release"])
        child.communicate()
        if child.returncode == 0:
          results[t] = True
      except Exception as e:
        print("Failed", e)
        continue
    os.chdir("..")

  #shutil.rmtree(folder)

  return results

all_checks = { url: run_checks(url) for url in urls }

with open("results.json", "w") as f:
  json.dump(all_checks, f)
