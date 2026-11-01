# Spec Delta

## Purpose

Allows Orbis customers to obtain authenticated release downloads, configure, and run the Orbis API on infrastructure they operate, using supported native packages or Docker without requiring Orbis to host the service.

## ADDED Requirements

### Requirement: Release publishes supported API server distributions
The system SHALL publish versioned API server artifacts with each production release for Linux x64 and ARM64, Windows x64, macOS Apple Silicon, and multi-architecture Linux Docker. Each downloadable artifact SHALL identify its version and target platform and provide integrity verification metadata.

#### Scenario: Release artifacts are available
- **WHEN** a production release completes successfully
- **THEN** its GitHub Release contains the native API packages, Docker deployment materials, and integrity metadata for every supported target

#### Scenario: Release packaging fails
- **WHEN** any required API distribution target fails to build or validate
- **THEN** the production release SHALL NOT be published as complete

### Requirement: Administrator chooses a server distribution separately from integration activation
The system SHALL keep Orbis Mobile integration activation independent from API server download, installation, or startup, and SHALL provide a separate setup flow to select a supported native platform or Docker distribution.

#### Scenario: Integration is enabled
- **WHEN** an administrator enables Orbis Mobile
- **THEN** the system SHALL NOT download, install, or remotely start an API server

#### Scenario: Administrator starts server setup
- **WHEN** an administrator opens the separate API server setup flow
- **THEN** the system SHALL present the supported native and Docker deployment targets and the corresponding release version

### Requirement: Administrator can download the selected distribution in a browser
The system SHALL open the selected native package or Docker deployment bundle in the administrator's default browser after validating the signed release manifest. The browser SHALL manage the download and save location.

#### Scenario: Native package browser download
- **WHEN** the administrator selects a supported native target and chooses download
- **THEN** the system SHALL open that target's signed GitHub release URL in the default browser

#### Scenario: Docker bundle browser download
- **WHEN** the administrator selects Docker and chooses download
- **THEN** the system SHALL open the signed GitHub release URL for the Docker deployment materials in the default browser

#### Scenario: Browser cannot be opened
- **WHEN** the default browser cannot be opened for the selected release URL
- **THEN** the system SHALL report the failure and SHALL NOT report that a download has started

### Requirement: Release metadata is authenticated before download
The system SHALL verify the release manifest signature and validate the selected artifact URL and checksum metadata before opening the browser. The desktop app SHALL NOT claim that the downloaded file itself has been checksum-verified.

#### Scenario: Signed release metadata is valid
- **WHEN** the release manifest signature is valid and the selected artifact matches the signed release metadata
- **THEN** the system SHALL allow its expected GitHub release URL to be opened in the browser

#### Scenario: Signed release metadata is invalid
- **WHEN** the release manifest signature or selected artifact metadata is invalid
- **THEN** the system SHALL report verification failure and SHALL NOT open a download URL

### Requirement: Distribution supports persistent customer-operated API service setup
Each distribution SHALL provide platform-appropriate setup materials to configure the API database connection and run the API persistently under customer control, without embedding customer credentials in the release artifact.

#### Scenario: Customer configures a native service
- **WHEN** a customer follows the setup materials for a supported native platform
- **THEN** the customer can configure the database connection and install or start the API as a persistent platform service

#### Scenario: Customer configures Docker
- **WHEN** a customer follows the Docker setup materials
- **THEN** the customer can configure the database connection and run the API container persistently using the published Docker distribution

#### Scenario: Required database schema is unavailable
- **WHEN** the API starts before its configured database has the required Orbis schema and migrations
- **THEN** the API SHALL report the setup prerequisite and SHALL NOT indicate successful service startup

### Requirement: Administrator can connect a running server to Orbis Mobile
The system SHALL allow an administrator to enter and validate the HTTPS endpoint of a customer-operated API server independently from downloading its distribution.

#### Scenario: Endpoint is reachable and valid
- **WHEN** the administrator submits a valid HTTPS API endpoint
- **THEN** the system SHALL validate connectivity and allow the endpoint to be used for the institution's mobile deployment

#### Scenario: Endpoint validation fails
- **WHEN** the endpoint is unreachable, invalid, or does not use HTTPS
- **THEN** the system SHALL report the validation failure and SHALL NOT mark the endpoint as ready for mobile clients
