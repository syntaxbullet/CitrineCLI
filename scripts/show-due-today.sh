#! bin/bash
citrine list | grep -E 'due: [0-9]{4}-[0-9]{2}-[0-9]{2}' | grep -E '[0-9]{4}-[0-9]{2}-[0-9]{2}' | sort -nr