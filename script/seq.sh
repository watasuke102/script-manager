#!/usr/bin/env bash
seq 5 | xargs -I {} bash -c "echo {}; sleep 1"
# seq inf | xargs -I {} bash -c "echo {}; sleep 1"
