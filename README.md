<div align="center">

  <h1>Aetrox</h1>
  <p><strong>Unstoppable force. Absolute precision.</strong></p>

  <p>
    <a href="https://omnisyntax.com"><img src="https://img.shields.io/badge/website-omnisyntax.com-00E5FF.svg?style=flat-square&color=09090B" alt="Website"></a>
    <img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg?style=flat-square" alt="License">
    <img src="https://img.shields.io/badge/status-alpha-orange.svg?style=flat-square" alt="Status">
    <img src="https://img.shields.io/badge/platform-cross--platform-success.svg?style=flat-square" alt="Platform">
  </p>

</div>

---

## ⚡ What is Aetrox?

**Aetrox** is a next-generation, high-performance programming language engineered for absolute speed, strict type safety, and zero-latency developer tooling. Built on a robust Rust compiler backend, Aetrox bridges the gap between low-level system performance and elite developer ergonomics.

---

## 💡 Syntax Preview

Writing Aetrox (`.ax`) code is clean, expressive, and strictly typed out of the box. No boilerplate, no memory leaks.

```ax
// A simple high-performance function in Aetrox
fn calculate_metrics(limit: Int) -> Int {
    let mut total = 0
    let mut i = 0
    
    while i < limit {
        total = total + i
        i = i + 1
    }
    
    print(total)
    return total
}
