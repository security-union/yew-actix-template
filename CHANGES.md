# Changelog
All notable changes to this project will be documented in this file.

## [1.4.0] - Unreleased
## Changed
- Added integration tests for the backend. This change ensures that the backend is tested in a more realistic environment, providing more confidence in the application's functionality.

(PR [#28](https://github.com/security-union/yew-actix-template/pull/25))

## [1.3.0] - 2023-11-24
### Changed
- Added versioning and changelog.
- Transitioned from cargo actions to Docker Compose actions, enabling Docker Compose to run on CI. This change enhances the continuous integration process by leveraging the capabilities of Docker Compose for building and testing the application.
- Upgraded Yew to a newer version, ensuring the frontend framework is up-to-date with the latest features and improvements.
- Replaced PostgreSQL with SQLx for database interactions. This shift to SQLx modernizes the database access layer, providing more flexibility and asynchronous support.

(PR [#26](https://github.com/security-union/yew-actix-template/pull/26))
(PR [#27](https://github.com/security-union/yew-actix-template/pull/27))