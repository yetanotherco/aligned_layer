1. **Security vulnerabilities** - Label by criticality (Critical/High/Medium/Low)
   - Solidity: e.g. reentrancy, access control, integer issues, etc.
   - Rust: e.g. unsafe blocks, error handling, panics, etc.
   - Web/API: e.g. SQL injection, auth bypass, input validation, sensitive data exposure, CORS/CSRF, etc.

2. **Potential bugs** - Logic errors, edge cases, incorrect behavior, race conditions

3. **Performance issues** - Only significant: e.g. O(n²) on unbounded input, N+1 queries, unbounded memory growth

4. **Simplicity** - Prefer simple, readable code over clever abstractions

Guidelines:
- Do NOT suggest micro-optimizations or premature abstractions
- Always prefer simplicity over complexity when performance gains are marginal
- Focus on real issues, not hypothetical improvements
- Be concise and actionable
