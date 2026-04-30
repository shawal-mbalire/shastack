#!/bin/bash
echo "-----------------------------------"
echo "shastack Security Scan"
echo "-----------------------------------"
echo "Scanning workspace for secrets..."
grep -r "SHA_KEY" . && echo "WARNING: Potential secret leakage!"
echo "Scan complete."
