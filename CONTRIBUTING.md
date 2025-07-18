# Contributing to Odo

Thank you for your interest in contributing to Odo! This document outlines the GitFlow workflow we use and how to contribute effectively.

## GitFlow Workflow

We use a GitFlow workflow for this project:

### Main Branches

- `main` - The production branch. This branch contains the stable, released code.
- `develop` - The main development branch. All feature branches are merged here.

### Supporting Branches

- `feature/xxx` - Feature branches for developing new features
- `hotfix/xxx` - Hotfix branches for urgent production fixes
- `release/x.x.x` - Release branches for preparing new versions

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/YOUR-USERNAME/Odo.git`
3. Set up upstream: `git remote add upstream https://github.com/ORIGINAL-OWNER/Odo.git`
4. Create a feature branch from develop: `git checkout -b feature/your-feature-name develop`

## Development Process

### Creating a Feature

1. Create a feature branch from develop:
   ```
   git checkout develop
   git pull
   git checkout -b feature/your-feature-name
   ```

2. Develop your feature and commit changes
3. Push your branch: `git push -u origin feature/your-feature-name`
4. When complete, open a pull request against the `develop` branch

### Creating a Hotfix

1. Create a hotfix branch from main:
   ```
   git checkout main
   git pull
   git checkout -b hotfix/issue-description
   ```

2. Fix the issue and commit changes
3. Push your branch: `git push -u origin hotfix/issue-description`
4. When complete, open pull requests against both `main` and `develop` branches

### Creating a Release

1. Create a release branch from develop:
   ```
   git checkout develop
   git pull
   git checkout -b release/x.x.x
   ```

2. Make release preparations (version bumps, documentation updates)
3. Fix any bugs found during testing
4. When ready, merge to both `main` and `develop`, and tag the version on main

## Commit Guidelines

- Use clear, descriptive messages
- Reference issue numbers when applicable
- Keep commits focused on single concerns

## Code Style

- Follow Rust style guidelines
- Write idiomatic Rust
- Include appropriate documentation
- Add tests for new functionality

## Pull Request Process

1. Ensure your code builds and tests pass
2. Update documentation as needed
3. Request review from maintainers
4. Address review feedback
5. Once approved, maintainers will merge your changes

Thank you for contributing!