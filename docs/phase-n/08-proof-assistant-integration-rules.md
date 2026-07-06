# Proof Assistant Integration Rules

Version: 1.0 Draft

Status: Executable Mathematics

---

# Purpose

This document defines the canonical integration rules between the Zanistarast Machine Verification Specification and supported proof assistants.

Its objective is to ensure deterministic interoperability while preserving semantic equivalence across independent formal verification systems.

The integration layer standardizes communication.

It does not modify mathematical meaning.

---

# Dependencies

This document depends upon

- Machine Verification Specification
- Cross-Backend Verification Framework
- First Executable Deterministic Runtime
- Certified Mathematical Core

Every integration process shall preserve compatibility with these foundations.

---

# Supported Proof Assistants

The canonical integration layer currently supports

- Lean
- Coq
- Isabelle/HOL
- Agda

Future proof assistants may be added after satisfying the Certified Scientific Core compatibility requirements.

---

# Integration Workflow

Canonical Formal Representation

↓

Backend Adapter

↓

Backend Translation

↓

Proof Construction

↓

Proof Verification

↓

Proof Certificate

↓

Certificate Normalization

↓

Cross-Backend Verification

↓

Machine Certification

---

# Integration Objectives

The integration layer shall preserve

- semantic equivalence,
- deterministic execution,
- traceability,
- reproducibility,
- certification integrity.

Backend-specific implementation details remain isolated within backend adapters.

---

# Backend Adapter

Each proof assistant shall expose a Backend Adapter.

The Backend Adapter is responsible for

- receiving Canonical Formal Representations,
- translating formal objects,
- invoking backend verification,
- collecting proof artifacts,
- returning normalized verification results.

The Backend Adapter shall not modify mathematical semantics.

---

# Certificate Normalization

Every Proof Certificate shall be transformed into a Canonical Certificate Representation (CCR).

The CCR contains

- CSS-ID,
- Backend Identifier,
- Proof Identifier,
- Verification Status,
- Certificate Metadata,
- Verification Timestamp.

The CCR is backend-independent.

---

# Integration Inputs

Every integration request contains

- Canonical Formal Representation,
- Machine Verification Identifier,
- Runtime Identifier,
- Backend Configuration,
- Verification Rule Set.

Each request is immutable after submission.

---

# Integration Lifecycle

Every Proof Assistant Integration Session follows the canonical lifecycle.

Integration Request

↓

Canonical Validation

↓

Backend Selection

↓

Backend Translation

↓

Proof Construction

↓

Proof Verification

↓

Certificate Generation

↓

Certificate Normalization

↓

Cross-Backend Verification

↓

Machine Certification

↓

Integration Completed

Each stage shall complete successfully before the next stage begins.

---

# Backend Capability Model

Every Backend Adapter shall publish a Backend Capability Profile.

The profile includes

- Backend Identifier,
- Backend Version,
- Supported Logic,
- Supported Language Features,
- Supported Proof Features,
- Supported Automation,
- Supported Certificate Format.

Capability Profiles are immutable during a verification session.

---

# Backend Negotiation

Before proof generation,

the Runtime performs Backend Capability Negotiation.

Negotiation determines

- compatible backend,
- supported proof features,
- supported verification rules,
- translation strategy.

Negotiation shall be deterministic.

---

# Translation Contract

Backend translation shall preserve

- mathematical meaning,
- logical dependencies,
- theorem identifiers,
- CSS-ID references,
- proof obligations.

Translation shall never

- strengthen assumptions,
- weaken assumptions,
- modify theorem statements,
- alter proof intent.

---

# Proof Artifact Exchange

Each backend returns

- Proof Certificate,
- Diagnostic Report,
- Proof Metadata,
- Execution Metadata.

Artifacts are normalized before entering Cross-Backend Verification.

Backend-specific artifacts remain archived.

---

# Version Compatibility

Every integration session records

- Backend Version,
- Adapter Version,
- Runtime Version,
- Machine Verification Version.

Verification results remain valid only for the recorded version set.

Future executions may produce new certification versions.

---

# Integration Trace

Every integration session generates an immutable Integration Trace.

The trace records

- backend selection,
- translation events,
- proof execution,
- certificate normalization,
- backend diagnostics,
- integration completion.

Integration Traces are append-only.

---

# Integration Contracts

Every Proof Assistant Integration Session shall satisfy the following contracts.

Contract PA1

Every integration session references exactly one Canonical Formal Representation.

Contract PA2

Every backend receives an equivalent formal specification.

Contract PA3

Every generated Proof Certificate references exactly one CSS-ID.

Contract PA4

Every normalized certificate preserves the semantic content of the original backend certificate.

Contract PA5

Every Machine Certification references all participating normalized certificates.

Violation of any Integration Contract immediately terminates Machine Certification.

---

# Integration Invariants

The integration layer preserves the following invariants.

Invariant 1

Backend adapters shall never modify certified mathematical semantics.

Invariant 2

Certificate normalization shall preserve proof validity.

Invariant 3

Every backend execution shall remain independently reproducible.

Invariant 4

Every Integration Trace shall be immutable.

Invariant 5

Cross-Backend Verification shall compare only normalized representations.

Invariant 6

Every integration session shall preserve deterministic execution.

---

# Backend Independence Principle

Each backend performs verification independently.

No backend may rely upon

- intermediate proofs,
- internal certificates,
- implementation details

produced by another backend.

Agreement is established exclusively through

Canonical Formal Representation,

Canonical Certificate Representation,

and

Cross-Backend Verification.

---

# Certificate Integrity

Every Canonical Certificate Representation shall contain

- CSS-ID,
- Backend Identifier,
- Proof Identifier,
- Verification Status,
- Certificate Hash,
- Verification Timestamp.

Certificate Hashes uniquely identify certified proof artifacts.

Modification of any certificate invalidates Machine Certification.

---

# Integration Failure Handling

When an integration failure occurs,

the Runtime shall

- preserve completed backend executions,
- archive diagnostic reports,
- terminate dependent integration stages,
- retain every normalized artifact generated before failure.

Historical integration data shall never be removed.

---

# Deterministic Interoperability

Equivalent

- Canonical Formal Representations,
- Runtime Configurations,
- Backend Configurations,

shall always produce equivalent

- Proof Certificates,
- Canonical Certificate Representations,
- Cross-Backend Verification Reports,
- Machine Certification Results.

This property defines deterministic interoperability.

---

# Soundness Objective

The Proof Assistant Integration Rules are sound if

- every backend receives a semantically equivalent Canonical Formal Representation,
- every Proof Certificate is independently verifiable,
- every Canonical Certificate Representation preserves proof validity,
- every Cross-Backend Verification compares equivalent mathematical objects,
- every Machine Certification is supported by certified proof artifacts.

No integration process shall introduce hidden mathematical assumptions.

---

# Completeness Objective

The integration framework is complete when every certified mathematical artifact may be verified through one or more supported proof assistants while preserving

- deterministic execution,
- semantic equivalence,
- certification traceability,
- backend independence.

Future proof assistants may extend this framework provided they satisfy the Certified Scientific Core compatibility requirements.

---

# Canonical Integration Pipeline

Canonical Formal Representation

↓

Backend Capability Negotiation

↓

Backend Translation

↓

Proof Construction

↓

Proof Verification

↓

Proof Certificate

↓

Canonical Certificate Representation

↓

Cross-Backend Verification

↓

Machine Certification

↓

Certified Scientific Registry

This pipeline defines the canonical integration workflow.

---

# Reference Integration Policy

Reference implementations shall preserve

- deterministic interoperability,
- backend independence,
- reproducibility,
- semantic preservation,
- certification integrity.

Implementation-specific optimizations are permitted provided these properties remain unchanged.

---

# Future Work

The next formal document establishes the

Verification Engine Algorithms.

Its objective is to formally define

- scheduling algorithms,
- dependency resolution algorithms,
- verification graph traversal,
- certification algorithms,
- deterministic execution algorithms,
- replay algorithms,
- conflict resolution algorithms.

These algorithms will become the executable computational core of the Verification Engine.

---

# End of File


