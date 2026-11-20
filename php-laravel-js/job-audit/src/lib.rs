use std::error::Error;
use std::fmt;
use std::io::{self, BufRead};
use std::num::NonZeroU64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobKind {
    Email,
    Report,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Job<'a> {
    pub kind: JobKind,
    pub id: NonZeroU64,
    pub payload: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseJobError {
    MissingFields,
    UnknownKind,
    InvalidId,
    EmptyPayload,
}

impl fmt::Display for ParseJobError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::MissingFields => "expected kind|id|payload",
            Self::UnknownKind => "job kind must be email or report",
            Self::InvalidId => "job id must be a positive 64-bit integer",
            Self::EmptyPayload => "job payload is empty",
        };
        f.write_str(message)
    }
}

impl Error for ParseJobError {}

pub fn parse_job(line: &str) -> Result<Job<'_>, ParseJobError> {
    let mut fields = line.splitn(3, '|').map(str::trim);
    let (Some(kind), Some(id), Some(payload)) =
        (fields.next(), fields.next(), fields.next())
    else {
        return Err(ParseJobError::MissingFields);
    };

    let kind = match kind {
        "email" => JobKind::Email,
        "report" => JobKind::Report,
        _ => return Err(ParseJobError::UnknownKind),
    };
    let id = id.parse().map_err(|_| ParseJobError::InvalidId)?;
    if payload.is_empty() {
        return Err(ParseJobError::EmptyPayload);
    }

    Ok(Job { kind, id, payload })
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Summary {
    pub total: u64,
    pub emails: u64,
    pub reports: u64,
}

#[derive(Debug)]
pub enum AuditError {
    Read { line: u64, source: io::Error },
    InvalidJob { line: u64, source: ParseJobError },
}

impl fmt::Display for AuditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { line, source } => {
                write!(f, "line {line}: could not read record: {source}")
            }
            Self::InvalidJob { line, source } => {
                write!(f, "line {line}: {source}")
            }
        }
    }
}

impl Error for AuditError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Read { source, .. } => Some(source),
            Self::InvalidJob { source, .. } => Some(source),
        }
    }
}

pub fn audit(mut input: impl BufRead) -> Result<Summary, AuditError> {
    let mut summary = Summary::default();
    let mut buffer = String::new();
    let mut line = 1;

    loop {
        buffer.clear();
        let bytes = input
            .read_line(&mut buffer)
            .map_err(|source| AuditError::Read { line, source })?;
        if bytes == 0 {
            return Ok(summary);
        }

        if !buffer.trim().is_empty() {
            let job = parse_job(&buffer)
                .map_err(|source| AuditError::InvalidJob { line, source })?;
            match job.kind {
                JobKind::Email => summary.emails += 1,
                JobKind::Report => summary.reports += 1,
            }
            summary.total += 1;
        }

        line += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_trims_the_fields() {
        let job = parse_job(" report | 42 | weekly | metrics ").unwrap();
        assert_eq!(job.kind, JobKind::Report);
        assert_eq!(job.id.get(), 42);
        assert_eq!(job.payload, "weekly | metrics");
    }

    #[test]
    fn distinguishes_validation_errors() {
        for (input, error) in [
            ("email|42", ParseJobError::MissingFields),
            ("cleanup|42|old logs", ParseJobError::UnknownKind),
            ("email|0|receipt", ParseJobError::InvalidId),
            (
                "email|18446744073709551616|receipt",
                ParseJobError::InvalidId,
            ),
            ("email|42|  ", ParseJobError::EmptyPayload),
        ] {
            assert_eq!(parse_job(input), Err(error));
        }
    }

    #[test]
    fn counts_jobs_across_blank_lines_and_accepts_unterminated_last_line() {
        let input = "email|42|receipt\r\n \r\nreport|99|metrics";
        let summary = audit(input.as_bytes()).unwrap();
        assert_eq!(
            summary,
            Summary {
                total: 2,
                emails: 1,
                reports: 1,
            }
        );
    }

    #[test]
    fn reports_physical_line_number() {
        let input = "email|42|receipt\n\nreport|0|metrics\n";
        let error = audit(input.as_bytes()).unwrap_err();
        assert!(matches!(
            error,
            AuditError::InvalidJob {
                line: 3,
                source: ParseJobError::InvalidId,
            }
        ));
    }

    #[test]
    fn reports_invalid_utf8_as_a_read_error() {
        let input = b"report|42|metrics\nemail|99|\xff\n";
        let error = audit(input.as_slice()).unwrap_err();
        assert!(matches!(
            error,
            AuditError::Read { line: 2, source } if source.kind() == io::ErrorKind::InvalidData
        ));
    }
}
