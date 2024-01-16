#! /bin/bash
tag=$1
citrine list | grep -E "tags:.*$tag" 