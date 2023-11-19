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
- Release: The main branch where well tested code resides.
- Development: Features that have been tested will continue to merge to this upstream. 
- Features: Meaningful addtions to the codebase are tested and merged upstream.
- Working: Where lessons are learned.

### Workflow

**1: Clone the Repository**
1. Each developer clones the central repository to their local machine using `git clone <repository_url>`.

**2: Create Working Branch**
1. When working on a new feature or bug fix, each developer creates a new branch using `git checkout -b <working_branch_name>`. This keeps their changes isolated from the main codebase.

**3: Develop and Commit Changes**
1. Developers make code changes and commit them to their working branch using `git add` and `git commit`.

**6: Push Changes Branches**
1. Developers push their branch updates to their working branch using `git push origin working-branch-name`.

**7: Create Pull Requests**
1. When a developer finishes working on a feature or bug fix, they create a pull request (PR). This initiates a code review process.

**8: Review Code**
1. Repo admin review the code changes in the PR, provide feedback, and suggest improvements.

**9: Make Changes (if necessary)**
1. **Iterate if Needed:** If there are suggested changes, the developer makes the necessary adjustments in their working branch and pushes the changes to the PR.

**10: Merge into Features Branch**
1. Once the code in the PR is approved, it can be merged into the features branch.

**11: Admin Clean Up**
1. After merging, the developer **can** delete the FeatureName branch on the central repository if complete.

**12: Rinse and Repeat**
1. Developers continue working on new features or bug fixes by creating new working branches and following the same process.

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


