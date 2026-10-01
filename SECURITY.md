# Security Policy

## Supported versions

Abhilekh is pre-1.0. Only the latest release (and `main`) receives security fixes.

## Reporting a vulnerability

Please **do not open a public issue** for security problems.

Use GitHub's private reporting instead: open the repository's **Security** tab and choose **Report a vulnerability**
(https://github.com/AarambhDevHub/abhilekh/security/advisories/new).

Please include:

- What you found and why it matters
- Steps to reproduce, or a small proof of concept
- The version or commit you tested, and your operating system
- Any suggested fix

## What to expect

- An acknowledgement within about 7 days.
- A status update when the issue is confirmed or ruled out.
- A fix and a coordinated disclosure. You will be credited in the release notes unless you prefer not to be.

This is a small open-source project run by volunteers, so please be patient.

## Scope

Examples of issues that are in scope:

- Corruption or loss of the event log through crafted input
- Path traversal or arbitrary file access through tool arguments or core note names
- Authentication or `Origin` validation bypass in the HTTP transport
- Panics or resource exhaustion reachable from a tool call
- Secrets or private memory written to logs or sent over the network unexpectedly

Out of scope:

- Anything requiring an attacker who already has write access to your `~/.abhilekh` folder or your account
- Vulnerabilities in dependencies with no practical impact here (please report those upstream)
- The fact that recalled memory can contain untrusted text. This is by design: every result carries its `source`, and agents should treat memory as data, not instructions

## Privacy by design

- Everything stays on the local machine by default. There is no telemetry.
- The HTTP transport (when added) binds to `127.0.0.1` only and requires a token.
