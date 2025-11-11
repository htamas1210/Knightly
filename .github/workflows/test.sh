#!/usr/bin/env bash
set -e

# --- ARGUMENT CHECK ---
if [ -z "$1" ]; then
  echo "Usage: $0 <project_path>"
  exit 1
fi

# --- SETUP VARIABLES ---
PROJECT_PATH="$1"
PROJECT_NAME=$(basename "$PROJECT_PATH")
ROOT_DIR=$(git rev-parse --show-toplevel)

LOG_FILE="${PROJECT_NAME}_test.log"
FINAL_LOG="${ROOT_DIR}/test_data.log"

# --- MOVE TO PROJECT DIRECTORY ---
echo ">>> Running tests for project: $PROJECT_NAME"
cd "$PROJECT_PATH" || { echo "Error: Could not cd into $PROJECT_PATH"; exit 1; }

# --- RUN TESTS ---
cargo test --verbose | tee full_test_output.log

# --- EXTRACT TEST SECTION ---
# Create the log file with the project name as the first line
echo "$PROJECT_NAME" > "$LOG_FILE"

# Then append only the lines between "running X test(s)" and the first empty line
awk '/^running [0-9]+ test[s]?$/,/^$/' full_test_output.log >> "$LOG_FILE"

# --- APPEND TO GLOBAL LOG (in repo root) ---
if [[ $(git rev-parse --abbrev-ref HEAD) == "master" ]]; then 
  echo "master" >> $FINAL_LOG
fi
cat "$LOG_FILE" >> "$FINAL_LOG"

# --- SUMMARY ---
echo ">>> Test output extracted to $PROJECT_PATH/$LOG_FILE"
echo ">>> Appended to $FINAL_LOG"
