# Security Policy

This document describes how security is handled for the Unit09 project and how
to responsibly report vulnerabilities.

## 1. Supported versions

Unit09 is an evolving project. The maintainers will typically provide security
fixes for:

- The latest stable release (for example, `0.x.y`)
- The development branch (`main`) when appropriate

Older releases may not receive security fixes unless a specific agreement is
made. When in doubt, open a private report and the maintainers will clarify
whether a given version is in scope.

## 2. What to report

You should report issues that may:

- Expose user funds or sensitive information
- Allow unauthorized access to infrastructure or services
- Enable remote code execution or arbitrary transaction signing
- Break integrity guarantees for on-chain data or off-chain processing
- Enable privilege escalation or bypass of authentication and authorization
- Cause significant denial-of-service conditions that are hard to mitigate

Examples of issues that are normally **not** considered security
vulnerabilities:

- Crashes caused by malformed local configuration
- Lack of rate limiting on non-sensitive endpoints in local development setups
- Issues in third-party dependencies that are not used in a security-sensitive
  context

If you are unsure whether something is a security issue, report it privately
anyway — the maintainers will review and respond.

## 3. How to report a vulnerability

### 3.1. Private disclosure

Please use one of the following private channels:

- Security email: `security@unit09.org`
- GitHub private security advisory (if enabled for the repository)

Do **not** open a public GitHub issue for suspected security vulnerabilities.

### 3.2. What to include

To help the maintainers triage and reproduce the issue, include:

- A clear description of the problem
- Affected components (for example, API, worker, contract)
- Steps to reproduce, including configuration details if relevant
- Any proof-of-concept code or scripts
- Impact assessment (what you think an attacker could do)
- Your environment details: versions of Unit09, Solana cluster, OS, etc.

If you believe the issue is time-sensitive or actively exploited, mention that
explicitly in the subject line or opening paragraph.

## 4. What to expect

After you report a vulnerability:

1. You should receive an acknowledgment within a reasonable time frame.
2. The maintainers will investigate and attempt to reproduce the issue.
3. If confirmed, they will:
   - Develop a fix or mitigation.
   - Plan a coordinated release and disclosure timeline.
   - Optionally ask you for additional details or verification.

The maintainers may keep some details private until users have had a chance
to upgrade or apply mitigations.

## 5. Public disclosure

Once a fix is available, the maintainers may publish:

- A changelog entry describing the fix
- A security advisory summarizing the impact and mitigation steps
- Optional additional technical details for transparency and learning

If you reported the vulnerability, you may be credited by name or handle
if you wish, subject to your consent and any relevant policies.

## 6. Safe harbor

The Unit09 project aims to support responsible security research.

We will not pursue legal action against researchers who:

- Make a good-faith effort to comply with this policy
- Do not intentionally harm users or infrastructure
- Avoid privacy violations and unnecessary data access
- Report vulnerabilities promptly and keep them confidential until a fix
  is available

Activities that are clearly malicious, destructive, or illegal are not
protected by this policy.

## 7. Hardening guidelines

If you operate your own Unit09 deployment, consider applying the following
defense-in-depth measures:

- Restrict access to API and worker endpoints with authentication and network
  controls.
- Use separate Solana keypairs and RPC endpoints for development, staging,
  and production.
- Monitor logs and metrics for unusual spikes in job volume or error rates.
- Configure limits for repository size, pipeline depth, and concurrency.
- Keep dependencies and container images up to date.

## 8. Questions

If you have questions about the security model, trust boundaries, or best
practices for deploying Unit09, you can:

- Open a non-sensitive discussion in the issue tracker or discussion board.
- Contact the maintainers via the security email for higher-level guidance.

Thank you for helping keep the Unit09 ecosystem safe.
