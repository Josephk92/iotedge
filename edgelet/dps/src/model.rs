// Copyright (c) Microsoft. All rights reserved.

/*
 * ProvisioningDeviceClient
 *
 * API for device runtime operations with the Azure IoT Hub Device Provisioning Service
 *
 * OpenAPI spec version: 2018-11-01
 *
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::default::Default;

use serde_derive::{Deserialize, Serialize};
/// [`DeviceRegistration`] : Device registration.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceRegistration {
    #[serde(rename = "registrationId", skip_serializing_if = "Option::is_none")]
    registration_id: Option<String>,
    #[serde(rename = "tpm", skip_serializing_if = "Option::is_none")]
    tpm: Option<TpmAttestation>,
}

impl DeviceRegistration {
    /// Device registration.
    pub fn new() -> Self {
        DeviceRegistration {
            registration_id: None,
            tpm: None,
        }
    }

    pub fn set_registration_id(&mut self, registration_id: String) {
        self.registration_id = Some(registration_id);
    }

    pub fn with_registration_id(mut self, registration_id: String) -> Self {
        self.registration_id = Some(registration_id);
        self
    }

    pub fn registration_id(&self) -> Option<&str> {
        self.registration_id.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_registration_id(&mut self) {
        self.registration_id = None;
    }

    pub fn set_tpm(&mut self, tpm: TpmAttestation) {
        self.tpm = Some(tpm);
    }

    pub fn with_tpm(mut self, tpm: TpmAttestation) -> Self {
        self.tpm = Some(tpm);
        self
    }

    pub fn tpm(&self) -> Option<&TpmAttestation> {
        self.tpm.as_ref()
    }

    pub fn reset_tpm(&mut self) {
        self.tpm = None;
    }
}

impl Default for DeviceRegistration {
    fn default() -> Self {
        DeviceRegistration::new()
    }
}

/// [`TpmAttestation`] : Attestation via TPM.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TpmAttestation {
    #[serde(rename = "endorsementKey")]
    endorsement_key: String,
    #[serde(rename = "storageRootKey", skip_serializing_if = "Option::is_none")]
    storage_root_key: Option<String>,
}

impl TpmAttestation {
    /// Attestation via TPM.
    pub fn new(endorsement_key: String) -> Self {
        TpmAttestation {
            endorsement_key,
            storage_root_key: None,
        }
    }

    pub fn set_endorsement_key(&mut self, endorsement_key: String) {
        self.endorsement_key = endorsement_key;
    }

    pub fn with_endorsement_key(mut self, endorsement_key: String) -> Self {
        self.endorsement_key = endorsement_key;
        self
    }

    pub fn endorsement_key(&self) -> &String {
        &self.endorsement_key
    }

    pub fn set_storage_root_key(&mut self, storage_root_key: String) {
        self.storage_root_key = Some(storage_root_key);
    }

    pub fn with_storage_root_key(mut self, storage_root_key: String) -> Self {
        self.storage_root_key = Some(storage_root_key);
        self
    }

    pub fn storage_root_key(&self) -> Option<&str> {
        self.storage_root_key.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_storage_root_key(&mut self) {
        self.storage_root_key = None;
    }
}

/// [`TpmRegistrationResult`] : TPM registration result.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TpmRegistrationResult {
    #[serde(rename = "authenticationKey", skip_serializing_if = "Option::is_none")]
    authentication_key: Option<String>,
}

impl TpmRegistrationResult {
    /// TPM registration result.
    pub fn new() -> Self {
        TpmRegistrationResult {
            authentication_key: None,
        }
    }

    pub fn set_authentication_key(&mut self, authentication_key: String) {
        self.authentication_key = Some(authentication_key);
    }

    pub fn with_authentication_key(mut self, authentication_key: String) -> Self {
        self.authentication_key = Some(authentication_key);
        self
    }

    pub fn authentication_key(&self) -> Option<&str> {
        self.authentication_key.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_authentication_key(&mut self) {
        self.authentication_key = None;
    }
}

impl Default for TpmRegistrationResult {
    fn default() -> Self {
        TpmRegistrationResult::new()
    }
}

/// [`SymmetricKeyRegistrationResult`] : Registration result returned when using `SymmetricKey` attestation

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SymmetricKeyRegistrationResult {
    #[serde(rename = "enrollmentGroupId")]
    enrollment_group_id: Option<String>,
}

impl SymmetricKeyRegistrationResult {
    /// Registration result returned when using SymmetricKey attestation
    pub fn new() -> SymmetricKeyRegistrationResult {
        SymmetricKeyRegistrationResult {
            enrollment_group_id: None,
        }
    }

    pub fn set_enrollment_group_id(&mut self, enrollment_group_id: String) {
        self.enrollment_group_id = Some(enrollment_group_id);
    }

    pub fn with_enrollment_group_id(mut self, enrollment_group_id: String) -> Self {
        self.enrollment_group_id = Some(enrollment_group_id);
        self
    }

    pub fn enrollment_group_id(&self) -> Option<&str> {
        self.enrollment_group_id.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_enrollment_group_id(&mut self) {
        self.enrollment_group_id = None;
    }
}

impl Default for SymmetricKeyRegistrationResult {
    fn default() -> Self {
        SymmetricKeyRegistrationResult::new()
    }
}

/// [`X509RegistrationResult`] : Registration result returned when using `X509` attestation

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct X509RegistrationResult {
    #[serde(rename = "certificateInfo", skip_serializing_if = "Option::is_none")]
    certificate_info: Option<X509CertificateInfo>,
    #[serde(rename = "enrollmentGroupId", skip_serializing_if = "Option::is_none")]
    enrollment_group_id: Option<String>,
    #[serde(rename = "signingCertificateInfo", skip_serializing_if = "Option::is_none")]
    signing_certificate_info: Option<X509CertificateInfo>,
}

impl X509RegistrationResult {
    /// X509 registration result.
    pub fn new() -> X509RegistrationResult {
        X509RegistrationResult {
            certificate_info: None,
            enrollment_group_id: None,
            signing_certificate_info: None,
        }
    }

    pub fn set_certificate_info(&mut self, certificate_info: X509CertificateInfo) {
        self.certificate_info = Some(certificate_info);
    }

    pub fn with_certificate_info(
        mut self,
        certificate_info: X509CertificateInfo,
    ) -> X509RegistrationResult {
        self.certificate_info = Some(certificate_info);
        self
    }

    pub fn certificate_info(&self) -> Option<&X509CertificateInfo> {
        self.certificate_info.as_ref()
    }

    pub fn reset_certificate_info(&mut self) {
        self.certificate_info = None;
    }

    pub fn set_enrollment_group_id(&mut self, enrollment_group_id: String) {
        self.enrollment_group_id = Some(enrollment_group_id);
    }

    pub fn with_enrollment_group_id(
        mut self,
        enrollment_group_id: String,
    ) -> X509RegistrationResult {
        self.enrollment_group_id = Some(enrollment_group_id);
        self
    }

    pub fn enrollment_group_id(&self) -> Option<&String> {
        self.enrollment_group_id.as_ref()
    }

    pub fn reset_enrollment_group_id(&mut self) {
        self.enrollment_group_id = None;
    }

    pub fn set_signing_certificate_info(&mut self, signing_certificate_info: X509CertificateInfo) {
        self.signing_certificate_info = Some(signing_certificate_info);
    }

    pub fn with_signing_certificate_info(
        mut self,
        signing_certificate_info: X509CertificateInfo,
    ) -> X509RegistrationResult {
        self.signing_certificate_info = Some(signing_certificate_info);
        self
    }

    pub fn signing_certificate_info(&self) -> Option<&X509CertificateInfo> {
        self.signing_certificate_info.as_ref()
    }

    pub fn reset_signing_certificate_info(&mut self) {
        self.signing_certificate_info = None;
    }
}

impl Default for X509RegistrationResult {
    fn default() -> Self {
        X509RegistrationResult::new()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct X509CertificateInfo {
    #[serde(rename = "subjectName")]
    subject_name: String,
    #[serde(rename = "sha1Thumbprint")]
    sha1_thumbprint: String,
    #[serde(rename = "sha256Thumbprint")]
    sha256_thumbprint: String,
    #[serde(rename = "issuerName")]
    issuer_name: String,
    #[serde(rename = "notBeforeUtc")]
    not_before_utc: String,
    #[serde(rename = "notAfterUtc")]
    not_after_utc: String,
    #[serde(rename = "serialNumber")]
    serial_number: String,
    #[serde(rename = "version")]
    version: i32,
}

impl X509CertificateInfo {
    /// X509 certificate info.
    pub fn new(
        subject_name: String,
        sha1_thumbprint: String,
        sha256_thumbprint: String,
        issuer_name: String,
        not_before_utc: String,
        not_after_utc: String,
        serial_number: String,
        version: i32,
    ) -> X509CertificateInfo {
        X509CertificateInfo {
            subject_name: subject_name,
            sha1_thumbprint: sha1_thumbprint,
            sha256_thumbprint: sha256_thumbprint,
            issuer_name: issuer_name,
            not_before_utc: not_before_utc,
            not_after_utc: not_after_utc,
            serial_number: serial_number,
            version: version,
        }
    }

    pub fn set_subject_name(&mut self, subject_name: String) {
        self.subject_name = subject_name;
    }

    pub fn with_subject_name(mut self, subject_name: String) -> X509CertificateInfo {
        self.subject_name = subject_name;
        self
    }

    pub fn subject_name(&self) -> &String {
        &self.subject_name
    }

    pub fn set_sha1_thumbprint(&mut self, sha1_thumbprint: String) {
        self.sha1_thumbprint = sha1_thumbprint;
    }

    pub fn with_sha1_thumbprint(mut self, sha1_thumbprint: String) -> X509CertificateInfo {
        self.sha1_thumbprint = sha1_thumbprint;
        self
    }

    pub fn sha1_thumbprint(&self) -> &String {
        &self.sha1_thumbprint
    }

    pub fn set_sha256_thumbprint(&mut self, sha256_thumbprint: String) {
        self.sha256_thumbprint = sha256_thumbprint;
    }

    pub fn with_sha256_thumbprint(mut self, sha256_thumbprint: String) -> X509CertificateInfo {
        self.sha256_thumbprint = sha256_thumbprint;
        self
    }

    pub fn sha256_thumbprint(&self) -> &String {
        &self.sha256_thumbprint
    }

    pub fn set_issuer_name(&mut self, issuer_name: String) {
        self.issuer_name = issuer_name;
    }

    pub fn with_issuer_name(mut self, issuer_name: String) -> X509CertificateInfo {
        self.issuer_name = issuer_name;
        self
    }

    pub fn issuer_name(&self) -> &String {
        &self.issuer_name
    }

    pub fn set_not_before_utc(&mut self, not_before_utc: String) {
        self.not_before_utc = not_before_utc;
    }

    pub fn with_not_before_utc(mut self, not_before_utc: String) -> X509CertificateInfo {
        self.not_before_utc = not_before_utc;
        self
    }

    pub fn not_before_utc(&self) -> &String {
        &self.not_before_utc
    }

    pub fn set_not_after_utc(&mut self, not_after_utc: String) {
        self.not_after_utc = not_after_utc;
    }

    pub fn with_not_after_utc(mut self, not_after_utc: String) -> X509CertificateInfo {
        self.not_after_utc = not_after_utc;
        self
    }

    pub fn not_after_utc(&self) -> &String {
        &self.not_after_utc
    }

    pub fn set_serial_number(&mut self, serial_number: String) {
        self.serial_number = serial_number;
    }

    pub fn with_serial_number(mut self, serial_number: String) -> X509CertificateInfo {
        self.serial_number = serial_number;
        self
    }

    pub fn serial_number(&self) -> &String {
        &self.serial_number
    }

    pub fn set_version(&mut self, version: i32) {
        self.version = version;
    }

    pub fn with_version(mut self, version: i32) -> X509CertificateInfo {
        self.version = version;
        self
    }

    pub fn version(&self) -> &i32 {
        &self.version
    }
}

/// [`RegistrationOperationStatus`] : Registration operation status.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegistrationOperationStatus {
    /// Operation ID.
    #[serde(rename = "operationId")]
    operation_id: String,
    /// Device enrollment status.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    /// Device registration status.
    #[serde(rename = "registrationState", skip_serializing_if = "Option::is_none")]
    registration_state: Option<DeviceRegistrationResult>,
}

impl RegistrationOperationStatus {
    /// Registration operation status.
    pub fn new(operation_id: String) -> Self {
        RegistrationOperationStatus {
            operation_id,
            status: None,
            registration_state: None,
        }
    }

    pub fn set_operation_id(&mut self, operation_id: String) {
        self.operation_id = operation_id;
    }

    pub fn with_operation_id(mut self, operation_id: String) -> Self {
        self.operation_id = operation_id;
        self
    }

    pub fn operation_id(&self) -> &String {
        &self.operation_id
    }

    pub fn set_status(&mut self, status: String) {
        self.status = Some(status);
    }

    pub fn with_status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }

    pub fn status(&self) -> Option<&str> {
        self.status.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_status(&mut self) {
        self.status = None;
    }

    pub fn set_registration_state(&mut self, registration_state: DeviceRegistrationResult) {
        self.registration_state = Some(registration_state);
    }

    pub fn with_registration_state(mut self, registration_state: DeviceRegistrationResult) -> Self {
        self.registration_state = Some(registration_state);
        self
    }

    pub fn registration_state(&self) -> Option<&DeviceRegistrationResult> {
        self.registration_state.as_ref()
    }

    pub fn reset_registration_state(&mut self) {
        self.registration_state = None;
    }
}

/// [`DeviceRegistrationResult`] : Device registration result.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceRegistrationResult {
    /// Registration result returned when using TPM attestation
    #[serde(rename = "tpm", skip_serializing_if = "Option::is_none")]
    tpm: Option<TpmRegistrationResult>,
    /// Registration result returned when using X509 attestation
    #[serde(skip_deserializing)]
    x509: Option<X509RegistrationResult>,
    /// Registration result returned when using SymmetricKey attestation
    #[serde(rename = "symmetricKey", skip_serializing_if = "Option::is_none")]
    symmetric_key: Option<SymmetricKeyRegistrationResult>,
    /// The registration ID is alphanumeric, lowercase, and may contain hyphens.
    #[serde(rename = "registrationId", skip_serializing_if = "Option::is_none")]
    registration_id: Option<String>,
    /// Registration create date time (in UTC).
    #[serde(rename = "createdDateTimeUtc", skip_serializing_if = "Option::is_none")]
    created_date_time_utc: Option<String>,
    /// Assigned Azure IoT Hub.
    #[serde(rename = "assignedHub", skip_serializing_if = "Option::is_none")]
    assigned_hub: Option<String>,
    /// Device ID.
    #[serde(rename = "deviceId", skip_serializing_if = "Option::is_none")]
    device_id: Option<String>,
    /// Enrollment status.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    /// Substatus for 'Assigned' devices. Possible values include -
    /// 'initialAssignment':  Device has been assigned to an IoT hub for the first time,
    /// 'deviceDataMigrated': Device has been assigned to a different IoT hub and its
    ///                       device data was migrated from the previously assigned IoT hub.
    ///                       Device data was removed from the previously assigned IoT hub,
    /// 'deviceDataReset':    Device has been assigned to a different IoT hub and its device
    ///                       data was populated from the initial state stored in the enrollment.
    ///                       Device data was removed from the previously assigned IoT hub.
    #[serde(rename = "substatus", skip_serializing_if = "Option::is_none")]
    substatus: Option<String>,
    /// Error code.
    #[serde(rename = "errorCode", skip_serializing_if = "Option::is_none")]
    error_code: Option<i32>,
    /// Error message.
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    error_message: Option<String>,
    /// Last updated date time (in UTC).
    #[serde(
        rename = "lastUpdatedDateTimeUtc",
        skip_serializing_if = "Option::is_none"
    )]
    last_updated_date_time_utc: Option<String>,
    /// The entity tag associated with the resource.
    #[serde(rename = "etag", skip_serializing_if = "Option::is_none")]
    etag: Option<String>,
}

impl DeviceRegistrationResult {
    /// Device registration result.
    pub fn new() -> Self {
        DeviceRegistrationResult {
            tpm: None,
            x509: None,
            symmetric_key: None,
            registration_id: None,
            created_date_time_utc: None,
            assigned_hub: None,
            device_id: None,
            status: None,
            substatus: None,
            error_code: None,
            error_message: None,
            last_updated_date_time_utc: None,
            etag: None,
        }
    }

    pub fn set_tpm(&mut self, tpm: TpmRegistrationResult) {
        self.tpm = Some(tpm);
    }

    pub fn with_tpm(mut self, tpm: TpmRegistrationResult) -> Self {
        self.tpm = Some(tpm);
        self
    }

    pub fn tpm(&self) -> Option<&TpmRegistrationResult> {
        self.tpm.as_ref()
    }

    pub fn reset_tpm(&mut self) {
        self.tpm = None;
    }

    pub fn set_symmetric_key(&mut self, symmetric_key: SymmetricKeyRegistrationResult) {
        self.symmetric_key = Some(symmetric_key);
    }

    pub fn with_symmetric_key(mut self, symmetric_key: SymmetricKeyRegistrationResult) -> Self {
        self.symmetric_key = Some(symmetric_key);
        self
    }

    pub fn symmetric_key(&self) -> Option<&SymmetricKeyRegistrationResult> {
        self.symmetric_key.as_ref()
    }

    pub fn reset_symmetric_key(&mut self) {
        self.symmetric_key = None;
    }

    pub fn set_registration_id(&mut self, registration_id: String) {
        self.registration_id = Some(registration_id);
    }

    pub fn with_registration_id(mut self, registration_id: String) -> Self {
        self.registration_id = Some(registration_id);
        self
    }

    pub fn registration_id(&self) -> Option<&str> {
        self.registration_id.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_registration_id(&mut self) {
        self.registration_id = None;
    }

    pub fn set_created_date_time_utc(&mut self, created_date_time_utc: String) {
        self.created_date_time_utc = Some(created_date_time_utc);
    }

    pub fn with_created_date_time_utc(mut self, created_date_time_utc: String) -> Self {
        self.created_date_time_utc = Some(created_date_time_utc);
        self
    }

    pub fn created_date_time_utc(&self) -> Option<&str> {
        self.created_date_time_utc.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_created_date_time_utc(&mut self) {
        self.created_date_time_utc = None;
    }

    pub fn set_assigned_hub(&mut self, assigned_hub: String) {
        self.assigned_hub = Some(assigned_hub);
    }

    pub fn with_assigned_hub(mut self, assigned_hub: String) -> Self {
        self.assigned_hub = Some(assigned_hub);
        self
    }

    pub fn assigned_hub(&self) -> Option<&str> {
        self.assigned_hub.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_assigned_hub(&mut self) {
        self.assigned_hub = None;
    }

    pub fn set_device_id(&mut self, device_id: String) {
        self.device_id = Some(device_id);
    }

    pub fn with_device_id(mut self, device_id: String) -> Self {
        self.device_id = Some(device_id);
        self
    }

    pub fn device_id(&self) -> Option<&str> {
        self.device_id.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_device_id(&mut self) {
        self.device_id = None;
    }

    pub fn set_status(&mut self, status: String) {
        self.status = Some(status);
    }

    pub fn with_status(mut self, status: String) -> Self {
        self.status = Some(status);
        self
    }

    pub fn status(&self) -> Option<&str> {
        self.status.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_status(&mut self) {
        self.status = None;
    }

    pub fn set_substatus(&mut self, substatus: String) {
        self.substatus = Some(substatus);
    }

    pub fn with_substatus(mut self, substatus: String) -> Self {
        self.substatus = Some(substatus);
        self
    }

    pub fn substatus(&self) -> Option<&str> {
        self.substatus.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_substatus(&mut self) {
        self.substatus = None;
    }

    pub fn set_error_code(&mut self, error_code: i32) {
        self.error_code = Some(error_code);
    }

    pub fn with_error_code(mut self, error_code: i32) -> Self {
        self.error_code = Some(error_code);
        self
    }

    pub fn error_code(&self) -> Option<i32> {
        self.error_code
    }

    pub fn reset_error_code(&mut self) {
        self.error_code = None;
    }

    pub fn set_error_message(&mut self, error_message: String) {
        self.error_message = Some(error_message);
    }

    pub fn with_error_message(mut self, error_message: String) -> Self {
        self.error_message = Some(error_message);
        self
    }

    pub fn error_message(&self) -> Option<&str> {
        self.error_message.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_error_message(&mut self) {
        self.error_message = None;
    }

    pub fn set_last_updated_date_time_utc(&mut self, last_updated_date_time_utc: String) {
        self.last_updated_date_time_utc = Some(last_updated_date_time_utc);
    }

    pub fn with_last_updated_date_time_utc(mut self, last_updated_date_time_utc: String) -> Self {
        self.last_updated_date_time_utc = Some(last_updated_date_time_utc);
        self
    }

    pub fn last_updated_date_time_utc(&self) -> Option<&str> {
        self.last_updated_date_time_utc.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_last_updated_date_time_utc(&mut self) {
        self.last_updated_date_time_utc = None;
    }

    pub fn set_etag(&mut self, etag: String) {
        self.etag = Some(etag);
    }

    pub fn with_etag(mut self, etag: String) -> Self {
        self.etag = Some(etag);
        self
    }

    pub fn etag(&self) -> Option<&str> {
        self.etag.as_ref().map(AsRef::as_ref)
    }

    pub fn reset_etag(&mut self) {
        self.etag = None;
    }
}

impl Default for DeviceRegistrationResult {
    fn default() -> Self {
        DeviceRegistrationResult::new()
    }
}
