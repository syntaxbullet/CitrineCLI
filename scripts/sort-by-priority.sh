#! bin/bash
citrine list | grep -E 'priority: [0-9]+' | grep -E '[0-9]+' | sort -nr
