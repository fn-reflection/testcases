#!/usr/bin/env bash
set -euo pipefail

MYSQL_ARGS=(
  --host=127.0.0.1
  --port=53306
  --user=root
  --password=root
  --database=sandbox
  --skip-column-names
  --raw
)

construct_query() {
  local order_by="$1"
  cat <<SQL
SELECT id, title, category, text1, CONCAT(text2, text3)
FROM plan_change_test
WHERE title REGEXP 'あかさたな'
AND category IN ('category1')
ORDER BY ${order_by}
LIMIT 44800;
SQL
}

run_case() {
  local order_by="$1"
  mysql "${MYSQL_ARGS[@]}" <<SQL > "log/${order_by}_explain.txt"
EXPLAIN FORMAT=JSON
$(construct_query "$order_by")\G
SQL

{
  time mysql "${MYSQL_ARGS[@]}" 2> /dev/null <<SQL
$(construct_query "$order_by")
SQL
} 2> "log/${order_by}_time.txt"

  mysql "${MYSQL_ARGS[@]}" <<SQL > "log/${order_by}_trace.txt"
SET SESSION optimizer_trace='enabled=on', end_markers_in_json=on;
$(construct_query "$order_by")
SELECT trace FROM information_schema.optimizer_trace\G
SQL
}

run_case "title"
run_case "REVERSE(title)"
