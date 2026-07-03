# Chapter 15 — Proof Sketches

Version: 1.0

---

# Purpose

This chapter provides structured proof sketches for the core theorems introduced in Book I.

These proof sketches are intended as a bridge between informal mathematical reasoning and future formal proofs in Lean, Coq, Isabelle/HOL and Agda.

---

# Proof Sketch — Theorem 14.1

## Assumptions

- Definition of Yek
- Core Axiom 6
- Core Axiom 8
- Lemma 13.3

## Goal

Show that the deterministic chain preserves the originating Yek.

## Derivation

1. Hebûn admits the entity without modifying Yek.
2. Zanabûn constructs knowledge while preserving Yek.
3. Mabûn constructs a model preserving Yek.
4. Rabûn executes the model without altering Yek.
5. Rasterast verifies the execution without altering Yek.

## Conclusion

The originating Yek is preserved throughout the deterministic chain.

---

# Proof Sketch — Theorem 14.2

## Assumptions

- Functions and Domains
- Core Axiom 10
- Lemma 13.4

## Goal

Show that the deterministic composition is itself deterministic.

## Derivation

Every component function is deterministic.

The composition of deterministic functions remains deterministic whenever every component is defined.

## Conclusion

The complete deterministic chain is deterministic.

---

# Proof Sketch — Theorem 14.3

## Assumptions

- Universe of Discourse
- Functions and Domains
- Lemma 13.5

## Goal

Show that every verified object belongs to the mathematical universe.

## Derivation

Every function maps objects within the defined universe.

Function composition therefore never leaves the universe.

## Conclusion

Every Rasterast-verified object belongs to U.

---

# Proof Sketch — Theorem 14.4

## Assumptions

- Dependency Chain
- Core Axiom 7
- Lemma 13.4

## Goal

Show that removing a layer makes verification undefined.

## Derivation

Each layer depends on the previous layer.

Removing any layer breaks the composition.

An undefined composition cannot produce a verified result.

## Conclusion

Rasterast verification is undefined whenever a required layer is absent.

---

# Proof Sketch — Theorem 14.5

## Assumptions

- Definition of Yek
- Core Axiom 1
- Core Axiom 10
- Lemma 13.3

## Goal

Show that identical Yek values cannot produce two different verified outputs under identical assumptions.

## Derivation

Yek is unique.

The chain is deterministic.

Deterministic computation preserves uniqueness.

## Conclusion

A single Yek cannot produce multiple verified outputs under identical assumptions.

---

# Scientific Review Note

These proof sketches are not formal proofs.

They identify the assumptions, logical structure and conclusions that will later be translated into machine-checked proofs.



