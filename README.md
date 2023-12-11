# yet another fine system monitor

[Glances](https://glances.readthedocs.io/) is a very nice system monitor.  I
find it strikes an optimal balance between density and legibility, and it's my
go-to monitor for watching utilization of my compute nodes etc.

However, Glances is quite slow — it usually uses about 25% of a CPU core just to
display the system monitor dashboard.

This is my attempt to write yet another fine system monitor, modeled after
Glances but with

- fewer features
- more bugs
- much less resource usage

It should work in fallback sysmon mode on most platforms supported by sysmon,
crossterm, and ratatui.  As soon as #2 is completed, it will be at its best on
Linux, since sysmon takes a least-common-denominator approach; I intend to
support it indefinitely as a fallback, but the primary development will directly
use Linux procfs.
