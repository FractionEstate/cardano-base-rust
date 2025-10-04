#!/bin/bash

# Function to fix a single markdown file
fix_md_file() {
    local file="$1"
    local tmp_file="${file}.tmp"
    
    # Use awk to properly handle blank lines around headers and lists
    awk '
    BEGIN {
        in_list = 0
        in_code = 0
        prev_blank = 0
    }
    
    # Track code blocks
    /^```/ {
        if (!in_code) {
            # Entering code block - ensure blank line before
            if (NR > 1 && !prev_blank && prev_line != "" && prev_line !~ /^```/) {
                print ""
            }
            in_code = 1
        } else {
            # Exiting code block
            in_code = 0
            print
            # Ensure blank line after if next line exists and is not blank
            getline next_line
            if (next_line != "" && next_line !~ /^```/ && next_line !~ /^$/) {
                print ""
            }
            print next_line
            prev_line = next_line
            prev_blank = (next_line == "")
            next
        }
    }
    
    # Track headers
    /^#{1,6} / {
        if (NR > 1 && !prev_blank && prev_line != "") {
            print ""
        }
        print
        prev_blank = 0
        prev_line = $0
        need_blank_after = 1
        next
    }
    
    # Track lists
    /^[[:space:]]*[-*+]  / || /^[[:space:]]*[0-9]+\. / {
        if (!in_list && NR > 1 && !prev_blank && prev_line != "") {
            print ""
        }
        in_list = 1
        print
        prev_blank = 0
        prev_line = $0
        next
    }
    
    # Non-list line after list
    {
        if (in_list && !(/^[[:space:]]*[-*+] / || /^[[:space:]]*[0-9]+\. / || /^$/)) {
            in_list = 0
            if (!prev_blank && prev_line != "") {
                print ""
            }
        }
        
        # Add blank after header if needed
        if (need_blank_after && $0 != "" && $0 !~ /^#/) {
            print ""
            need_blank_after = 0
        }
        
        # Remove multiple consecutive blanks
        if ($0 == "" && prev_blank) {
            next
        }
        
        print
        prev_blank = ($0 == "")
        prev_line = $0
    }
    ' "$file" > "$tmp_file"
    
    # Apply additional fixes with sed
    sed -i 's|\( \)\(https\?://[^ )>\]]*\)\( \)|\1<\2>\3|g' "$tmp_file"
    sed -i 's|\( \)\([a-zA-Z0-9._-]*@[a-zA-Z0-9._-]*\.[a-zA-Z]*\)\( \)|\1<\2>\3|g' "$tmp_file"
    
    # Move temp file to original
    mv "$tmp_file" "$file"
    echo "Fixed: $file"
}

# Fix specific files
for file in \
    "README.md" \
    "CLEANUP_COMPLETE.md" \
    "DOCUMENTATION_CLEANUP_SUMMARY.md" \
    "docs/README.md" \
    "docs/index.md" \
    "docs/audit.md" \
    "docs/Home.md" \
    "VRF_TEST_FIX_SUMMARY.md" \
    "AUDIT_EXECUTIVE_SUMMARY.md" \
    "AUDIT_FINAL_REPORT.md" \
    "COMPREHENSIVE_AUDIT_LINE_BY_LINE.md" \
    "CROSS_VALIDATION_REPORT.md" \
    "CROSS_VALIDATION_SUMMARY.md" \
    "CROSS_VALIDATION_TEST_PLAN.md"
do
    if [ -f "$file" ]; then
        fix_md_file "$file"
    fi
done

echo "Markdown linting fixes applied!"
