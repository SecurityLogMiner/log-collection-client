# Log Collection Backend

Add additional log sources from to be sent to a centralized server

## Table of Contents

- [Project Management](#project-management)
    - [Branch Types](#branch-types)
    - [Workflow](#workflow)
    - [Creating Issues](#creating-issues)
- [Getting Started](#getting-started)
    - [System Design](https://github.com/SecurityLogMiner/log-collection-backend/blob/features/log-collection-backend.drawio.png)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)
- [Contact](#contact)

## Project Management

### Branch Types
- Prototype: The main branch where well tested code resides.
- Development: Created from the Prototype branch, acting as a buffer between the
  prototype and feature branches. This branch requires 4 approvals before merging
  the dev-api branch into it.
- DevApi: Created from the Development branch. Unlike the Development branch,
  this branch does not require 4 approvals. It will act as a buffer branch
  between Development and the following Features branch (see below).
- Features: This branch is where the FeatureName branches will be pushed and fetched
  from until the target features are complete. 
- FeatureName: These branches will be named by their feature, and pushed into the
  Features branch and pulled into the other feature branches so all team members
  have the same codebase while working on their own protion of the product.

### Workflow

**1: Clone the Repository**
1. **Clone the Repository:** Each developer clones the central repository to their local machine using `git clone <repository_url>`.

**2: Create FeatureName Branches**
1. **Create a FeatureName Branch:** When working on a new feature or bug fix, each developer creates a new branch using `git checkout -b features/your-feature-name`. This keeps their changes isolated from the main codebase.

**3: Develop and Commit Changes**
1. **Work on Features:** Developers make code changes and commit them to their FeatureName branch using `git add` and `git commit`.

**4: Pull Updates from the closest upstream branch**
1. **Stay Up to Date:** Regularly, developers should pull the latest changes from the parent branch into their FeatureName branches using `git pull origin features`.

**5: Resolve Conflicts (if any)**
1. **Conflict Resolution:** If there are conflicts, developers resolve them by editing the conflicting files and then committing the resolved changes.

**6: Push FeatureName Branches**
1. **Push FeatureName Branches:** Developers push their FeatureName branches to the Features branch using `git push origin features/your-feature-name`.

**7: Create Pull Requests**
1. **Create Pull Requests:** When a developer finishes working on a feature or bug fix, they create a pull request (PR). This initiates a code review process.

**8: Review Code**
1. **Review Code:** Other developers review the code changes in the PR, provide feedback, and suggest improvements.

**9: Make Changes (if necessary)**
1. **Iterate if Needed:** If there are suggested changes, the developer makes the necessary adjustments in their feature branch and pushes the changes to the PR.

**10: Merge into DevApi Branch**
1. **Merge into Main:** Once the code in the PR is approved, it can be merged into the DevApi branch.

**11: Clean Up**
1. **Delete FeatureName Branch:** After merging, the developer can delete the FeatureName branch both locally and in the central repository if that feature is complete.

**12: Rinse and Repeat**
1. **Repeat:** Developers continue working on new features or bug fixes by creating new feature branches and following the same process.

### Creating Issues
Mark issues with relevant tags.

## Getting Started
Install Rust on your local machine. Use the following link to get setup quickly:
[rust setup](https://www.rust-lang.org/tools/install)

## Usage
TBD

## Contributing
TBD

## License
Apache 2.0

## Acknowledgments
Syn Ack Fin

## Contact
Discord, if you know, you know

[Back to top](#table-of-contents)


