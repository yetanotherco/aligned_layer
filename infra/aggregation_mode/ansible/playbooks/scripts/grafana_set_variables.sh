#!/bin/bash
# grafana_set_variables.sh
# Sets dashboard variables in a Grafana dashboard via the Grafana API.
#
# Usage:
#   ./grafana_set_variables.sh <grafana_url> <admin_password> <dashboard_uid_or_title> [key=value ...]
#
# The third argument can be either:
#   - A dashboard UID (e.g., "a66a5480-6a60-4b87-9d29-4f0f446edafd")
#   - A dashboard title to search for (e.g., "Aggregation Mode")
#
# Example:
#   ./grafana_set_variables.sh "http://localhost:3000" "admin123" "a66a5480-6a60-4b87-9d29-4f0f446edafd" \
#       "payments_contract=0x1234..." \
#       "proof_aggregator_contract=0x5678..." \
#       "proof_aggregator_wallet=0xabcd..."

set -e

GRAFANA_URL="${1}"
ADMIN_PASSWORD="${2}"
DASHBOARD_ID="${3}"
shift 3

if [ -z "$GRAFANA_URL" ] || [ -z "$ADMIN_PASSWORD" ] || [ -z "$DASHBOARD_ID" ]; then
    echo "Usage: $0 <grafana_url> <admin_password> <dashboard_uid_or_title> [key=value ...]"
    exit 1
fi

AUTH="admin:${ADMIN_PASSWORD}"

# Check if DASHBOARD_ID looks like a UID (contains dashes in UUID format)
if [[ "$DASHBOARD_ID" =~ ^[a-f0-9]{8}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{4}-[a-f0-9]{12}$ ]]; then
    # It's a UID, use it directly
    DASHBOARD_UID="$DASHBOARD_ID"
    echo "Using dashboard UID: ${DASHBOARD_UID}"
else
    # List all dashboards and find by title
    DASHBOARD_TITLE="$DASHBOARD_ID"
    SEARCH_RESULT=$(curl -s -u "${AUTH}" "${GRAFANA_URL}/api/search?type=dash-db")

    # Find dashboard UID by matching title
    DASHBOARD_UID=$(echo "${SEARCH_RESULT}" | jq -r --arg title "$DASHBOARD_TITLE" '.[] | select(.title == $title) | .uid' | head -1)

    if [ -z "$DASHBOARD_UID" ]; then
        echo "Error: Dashboard '${DASHBOARD_ID}' not found"
        exit 1
    fi

    echo "Found dashboard '${DASHBOARD_ID}' with UID: ${DASHBOARD_UID}"
fi

# Get full dashboard definition
DASHBOARD_RESPONSE=$(curl -s -u "${AUTH}" "${GRAFANA_URL}/api/dashboards/uid/${DASHBOARD_UID}")

# Extract dashboard and meta from response
DASHBOARD=$(echo "${DASHBOARD_RESPONSE}" | jq '.dashboard')
FOLDER_ID=$(echo "${DASHBOARD_RESPONSE}" | jq '.meta.folderId')

if [ "$DASHBOARD" = "null" ]; then
    echo "Error: Could not retrieve dashboard definition"
    exit 1
fi

# Process each key=value pair
for ARG in "$@"; do
    KEY="${ARG%%=*}"
    VALUE="${ARG#*=}"

    if [ -z "$KEY" ] || [ -z "$VALUE" ]; then
        echo "Warning: Skipping invalid argument '${ARG}'"
        continue
    fi

    echo "Setting variable '${KEY}' = '${VALUE}'"

    # Check if variable already exists in templating.list
    EXISTING_VAR=$(echo "${DASHBOARD}" | jq --arg key "$KEY" '.templating.list // [] | map(select(.name == $key)) | length')

    if [ "$EXISTING_VAR" -gt 0 ]; then
        # Update existing variable
        DASHBOARD=$(echo "${DASHBOARD}" | jq --arg key "$KEY" --arg val "$VALUE" '
            .templating.list = [
                .templating.list[] |
                if .name == $key then
                    .query = $val | .current = {"text": $val, "value": $val}
                else
                    .
                end
            ]
        ')
    else
        # Add new variable
        NEW_VAR=$(jq -n --arg key "$KEY" --arg val "$VALUE" '{
            "name": $key,
            "type": "constant",
            "hide": 2,
            "query": $val,
            "current": {"text": $val, "value": $val},
            "skipUrlSync": false
        }')

        DASHBOARD=$(echo "${DASHBOARD}" | jq --argjson var "$NEW_VAR" '
            .templating.list = (.templating.list // []) + [$var]
        ')
    fi
done

# Remove id and version to allow update (Grafana uses uid for identification)
DASHBOARD=$(echo "${DASHBOARD}" | jq 'del(.id) | del(.version)')

# Create the save payload
PAYLOAD=$(jq -n \
    --argjson dashboard "$DASHBOARD" \
    --argjson folderId "$FOLDER_ID" \
    '{
        "dashboard": $dashboard,
        "folderId": $folderId,
        "overwrite": true
    }')

# Save the dashboard
SAVE_RESPONSE=$(curl -s -u "${AUTH}" \
    -H "Content-Type: application/json" \
    -X POST \
    "${GRAFANA_URL}/api/dashboards/db" \
    -d "${PAYLOAD}")

# Check for errors
STATUS=$(echo "${SAVE_RESPONSE}" | jq -r '.status // empty')
if [ "$STATUS" = "success" ]; then
    echo "Dashboard updated successfully"
else
    ERROR_MSG=$(echo "${SAVE_RESPONSE}" | jq -r '.message // "Unknown error"')
    echo "Error updating dashboard: ${ERROR_MSG}"
    exit 1
fi
