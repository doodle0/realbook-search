#!/bin/bash
# Documentation Audit Script
# Run this before milestone completions to ensure docs are aligned

echo "🔍 Running documentation audit..."
echo "================================"
echo ""

# 1. Check package versions
echo "1️⃣  Package Versions:"
echo "   Actual:"
grep -E "rocket|yew|reqwest" api/Cargo.toml ui/Cargo.toml | grep version | head -3 | sed 's/^/      /'
echo ""

# 2. Check edition
echo "2️⃣  Rust Edition:"
echo "   Actual:"
grep "edition" api/Cargo.toml ui/Cargo.toml | uniq | sed 's/^/      /'
echo "   Documented:"
grep -h "Edition 202" README.md CLAUDE.md | head -2 | sed 's/^/      /'
echo ""

# 3. Check port numbers
echo "3️⃣  Port Configuration:"
grep "port.*808" ui/Trunk.toml | sed 's/^/      /'
echo ""

# 4. Check API endpoints
echo "4️⃣  API Endpoints:"
grep "#\[get" api/src/controller.rs | sed 's/^/      /'
echo ""

# 5. Check data count
echo "5️⃣  Data Statistics:"
ACTUAL_COUNT=$(grep -c '"title"' api/resources/realbook.json 2>/dev/null || echo "0")
DOC_COUNT=$(grep -o '1,[0-9]*' README.md 2>/dev/null | head -1 || echo "N/A")
echo "      Actual entries: $ACTUAL_COUNT"
echo "      Documented: $DOC_COUNT"
echo ""

# 6. Check project status
echo "6️⃣  Project Status:"
grep -i "phase.*complete\|current status:" README.md CLAUDE.md | head -3 | sed 's/^/      /'
echo ""

# 7. Check for common issues
echo "7️⃣  Common Issues Check:"
ISSUES=0

# Check if API_BASE_URL is correctly documented (should be in api.rs not main.rs)
if grep -q "API_BASE_URL.*main.rs" CLAUDE.md 2>/dev/null; then
    echo "      ⚠️  API_BASE_URL docs reference main.rs (should be api.rs)"
    ISSUES=$((ISSUES + 1))
fi

# Check if edition matches between code and docs
CODE_EDITION=$(grep "edition" api/Cargo.toml | head -1 | grep -o "202[0-9]")
DOC_EDITION=$(grep -o "Edition 202[0-9]" README.md | head -1 | grep -o "202[0-9]")
if [ "$CODE_EDITION" != "$DOC_EDITION" ]; then
    echo "      ⚠️  Edition mismatch: Code=$CODE_EDITION, Docs=$DOC_EDITION"
    ISSUES=$((ISSUES + 1))
fi

if [ $ISSUES -eq 0 ]; then
    echo "      ✅ No issues found"
fi
echo ""

echo "================================"
echo "✅ Audit complete!"
echo ""
echo "📋 For full audit checklist, see: docs/AUDIT.md"
echo "🔧 Found issues? Fix them and run this script again."
