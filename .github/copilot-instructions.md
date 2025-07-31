# AI Development Human-AI Collaboration Rules Index

## Core Behavioral Rules
0. You are skilled at calling appropriate tools to complete various tasks
1. You will choose the right moment to ask users questions after completing dialogue output, such as whether to add backend capabilities, whether to open preview, whether to deploy, etc.
2. You will first read the current project's README.md and develop according to the current project's instructions. If it doesn't exist, you will generate a README.md file after generating the project
3. During development, by default, output all project code in the current directory, first check the files in the current directory
4. During development preview, if the project itself depends on backend database collections and cloud functions, you can prioritize deploying the backend first and then preview the frontend
5. Interactive feedback rules: Actively dialogue with users to clarify when requirements are unclear, prioritize using automated tools interactiveDialog to complete configuration. Must use interactiveDialog to get user confirmation before executing high-risk operations. Keep messages concise and mark status with emojis.
6. If it involves real-time communication such as real-time battles, you can use the real-time database watch capability of cloud development

## Workflow

<workflow>
0. Please note! You must follow the rules below, and each step must be confirmed by me before proceeding to the next step;
1. If you judge that my input raises a new requirement, you can work independently according to the standard software engineering approach below, and only ask me when needed. You can use the interactiveDialog tool to collect information
2. Whenever I input new requirements, in order to standardize requirement quality and acceptance criteria, you will first clarify the problem and requirements, then proceed to the next stage
3. Requirements documentation and acceptance criteria design: First complete the requirement design, describe it according to the EARS simple requirement syntax method. If you judge that the requirement involves frontend pages, you can also determine the design style and color scheme in advance in the requirements, confirm requirement details with me, and after final confirmation, finalize the requirements, then proceed to the next stage, save in `specs/spec_name/requirements.md`, reference format as follows

```markdown
# Requirements Document

## Introduction

Requirement description

## Requirements

### Requirement 1 - Requirement Name

**User Story:** User story content

#### Acceptance Criteria

1. Use EARS descriptive clauses While <optional precondition>, when <optional trigger>, the <system name> shall <system response>, for example When selecting "mute", the laptop should suppress all audio output.
2. ...
...
```
4. Technical solution design: After completing the requirement design, you will conduct technical solution design for the requirements based on the current technical architecture and previously confirmed requirements, concise but accurately describing the technical architecture (such as architecture, technology stack, technology selection, database/interface design, testing strategy, security), use mermaid diagrams when necessary, confirm with me clearly, save in `specs/spec_name/design.md`, then proceed to the next stage
5. Task breakdown: After completing the technical solution design, you will break down specific tasks based on the requirements document and technical solution, confirm with me clearly, save in `specs/spec_name/tasks.md`, then proceed to the next stage, start formally executing tasks, and update task status in time, run as independently as possible during execution to ensure efficiency and quality

Task reference format as follows

``` markdown
# Implementation Plan

- [ ] 1. Task Information
  - Specific things to do
  - ...
  - _Requirements: Related requirement point numbers

```
</workflow>

