#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: ./scripts/new_problem.sh <problem_id> [function_name]

Example:
  ./scripts/new_problem.sh p3 two_sum
USAGE
}

if [[ $# -lt 1 ]]; then
  usage
  exit 1
fi

start_ts=$(date +%s)

pid="$1"
func="${2:-solve}"

solution_path="src/solutions/${pid}.rs"
test_path="tests/${pid}.rs"
cases_dir="testcases/${pid}"
cases_path="${cases_dir}/cases.json"
mod_path="src/solutions/mod.rs"
log_path="metrics/scaffold.csv"

if [[ -e "${solution_path}" || -e "${test_path}" || -e "${cases_path}" ]]; then
  echo "One or more target files already exist for ${pid}."
  echo "Refusing to overwrite:"
  echo "  ${solution_path}"
  echo "  ${test_path}"
  echo "  ${cases_path}"
  exit 1
fi

mkdir -p "${cases_dir}"

if ! grep -q "pub mod ${pid};" "${mod_path}"; then
  echo "pub mod ${pid};" >> "${mod_path}"
fi

cat > "${solution_path}" <<EOF
pub struct Solution;

impl Solution {
    pub fn ${func}(/* TODO */) -> /* TODO */ {
        todo!()
    }
}
EOF

cat > "${test_path}" <<EOF
mod common;

use common::load_cases;
use leetcode_rs::solutions::${pid}::Solution;
use serde::Deserialize;

#[derive(Deserialize)]
struct Input {
    // TODO: define input fields
}

#[test]
fn ${pid}_${func}() {
    let cases = load_cases::<Input, /* TODO */>("testcases/${pid}/cases.json");
    for case in cases.cases {
        let got = Solution::${func}(/* TODO */);
        assert_eq!(got, case.output, "case {}", case.id);
    }
}
EOF

cat > "${cases_path}" <<'EOF'
{
  "cases": [
    {
      "id": "case1",
      "input": {},
      "output": null
    }
  ]
}
EOF

end_ts=$(date +%s)
duration=$((end_ts - start_ts))
timestamp=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

if [[ ! -f "${log_path}" ]]; then
  echo "timestamp,problem_id,duration_seconds" > "${log_path}"
fi
echo "${timestamp},${pid},${duration}" >> "${log_path}"

echo "Created:"
echo "  ${solution_path}"
echo "  ${mod_path}"
echo "  ${cases_path}"
echo "  ${test_path}"
echo "Logged duration (${duration}s) to ${log_path}"
