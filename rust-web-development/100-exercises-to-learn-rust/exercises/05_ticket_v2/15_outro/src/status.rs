// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `Status` enum.
//  The parsing should be case-insensitive.

#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, thiserror::Error)]
pub enum StatusError {
    #[error("Converting to Status Failed")]
    StatusConversionFailed,
}

impl TryFrom<&str> for Status {
    type Error = StatusError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.trim().to_lowercase() == "todo" {
            return Ok(Status::ToDo);
        }
        if value.trim().to_lowercase() == "inprogress" {
            return Ok(Status::InProgress);
        }
        if value.trim().to_lowercase() == "done" {
            return Ok(Status::Done);
        }
        Err(StatusError::StatusConversionFailed)
    }
}

impl TryFrom<String> for Status {
    type Error = StatusError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Status::try_from(value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
