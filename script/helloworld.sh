#!/usr/bin/env bash
seq inf | xargs -I {} bash -c "echo Hello {}; echo World {}; sleep 1"
