# Nerdle Solver

A developing challenge I undertook to learn Rust. The goal is to provide a solver for [Nerdle](https://nerdlegame.com/).

## Introduction

Nerdle is a math-based puzzle game where players must guess the correct equation. This project aims to create a solver for Nerdle using Rust, allowing users to find solutions efficiently.

## Status

This project is currently in development during my spare time. Explanations of the implemented features and algorithms will be provided in this README.md file as the project progresses.


## Elements developped

### Arena

The choice was made here to have a representation of ADT flattened with the arena. This data structure is suitable for deta structures like AST or, here, the order of expressions that have to be evaluated.