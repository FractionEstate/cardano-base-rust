#!/usr/bin/env python3
"""
Fix all markdown linting issues across the repository.
"""

import re
import os
from pathlib import Path

def fix_markdown_file(filepath):
    """Fix common markdown linting issues in a file."""
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()

    original_content = content
    lines = content.split('\n')
    fixed_lines = []

    i = 0
    while i < len(lines):
        line = lines[i]
        prev_line = lines[i-1] if i > 0 else ''
        next_line = lines[i+1] if i < len(lines)-1 else ''

        # Fix MD022: Headers should be surrounded by blank lines
        if re.match(r'^#{1,6}\s+', line):
            # Add blank line before header if previous line is not blank and not start of file
            if i > 0 and prev_line.strip() != '':
                fixed_lines.append('')
            fixed_lines.append(line)
            # Add blank line after header if next line is not blank
            if i < len(lines)-1 and next_line.strip() != '' and not re.match(r'^#{1,6}\s+', next_line):
                i += 1
                if i < len(lines):
                    fixed_lines.append('')
            i += 1
            continue

        # Fix MD032: Lists should be surrounded by blank lines
        is_list_item = re.match(r'^[\s]*[-*+]\s+', line) or re.match(r'^[\s]*\d+\.\s+', line)
        prev_is_list = re.match(r'^[\s]*[-*+]\s+', prev_line) or re.match(r'^[\s]*\d+\.\s+', prev_line)
        next_is_list = re.match(r'^[\s]*[-*+]\s+', next_line) or re.match(r'^[\s]*\d+\.\s+', next_line)

        if is_list_item:
            # Add blank line before first list item
            if not prev_is_list and prev_line.strip() != '':
                fixed_lines.append('')
            fixed_lines.append(line)
            # Add blank line after last list item
            if not next_is_list and next_line.strip() != '':
                fixed_lines.append(line)
                fixed_lines.append('')
                i += 1
                continue
        else:
            fixed_lines.append(line)

        i += 1

    # Fix MD031: Fenced code blocks should be surrounded by blank lines
    result_lines = []
    for i, line in enumerate(fixed_lines):
        prev_line = fixed_lines[i-1] if i > 0 else ''
        next_line = fixed_lines[i+1] if i < len(fixed_lines)-1 else ''

        # Check for code fence start/end
        if line.strip().startswith('```'):
            # Add blank line before code fence if needed
            if i > 0 and prev_line.strip() != '' and not prev_line.strip().startswith('```'):
                result_lines.append('')
            result_lines.append(line)
            # Add blank line after code fence if needed
            if i < len(fixed_lines)-1 and next_line.strip() != '' and not next_line.strip().startswith('```'):
                result_lines.append(line)
                result_lines.append('')
                continue
        else:
            result_lines.append(line)

    content = '\n'.join(result_lines)

    # Fix MD034: Bare URLs should be in angle brackets
    content = re.sub(r'(\s)(https?://[^\s\)>\]]+)(\s)', r'\1<\2>\3', content)
    content = re.sub(r'(\s)([\w\.-]+@[\w\.-]+\.\w+)(\s)', r'\1<\2>\3', content)

    # Fix MD012: Multiple consecutive blank lines
    content = re.sub(r'\n\n\n+', '\n\n', content)

    # Fix MD040: Fenced code blocks should have a language
    content = re.sub(r'\n```\n', '\n```bash\n', content)

    # Only write if changed
    if content != original_content:
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)
        return True
    return False

def main():
    """Fix all markdown files in the repository."""
    repo_root = Path(__file__).parent
    fixed_count = 0

    # Find all markdown files (excluding target directory)
    for md_file in repo_root.rglob('*.md'):
        if 'target' in md_file.parts:
            continue

        try:
            if fix_markdown_file(md_file):
                print(f"Fixed: {md_file.relative_to(repo_root)}")
                fixed_count += 1
        except Exception as e:
            print(f"Error fixing {md_file}: {e}")

    print(f"\nTotal files fixed: {fixed_count}")

if __name__ == '__main__':
    main()
