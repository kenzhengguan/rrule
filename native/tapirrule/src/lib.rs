use chrono::{DateTime, Duration, TimeZone};
use rrule::{RRuleSet, Tz};

#[rustler::nif]
fn r_range(
    rrule_str: String,
    after_date_str: String,
    before_date_str: String,
) -> Result<Vec<String>, String> {
    rrule_range(rrule_str, after_date_str, before_date_str)
}

fn rrule_range(
    rrule_str: String,
    after_date_str: String,
    before_date_str: String,
) -> Result<Vec<String>, String> {
    let rrule: RRuleSet = rrule_str.parse().map_err(|_| "rrule_str invalid format")?;

    let after: DateTime<Tz> = Tz::UTC
        .datetime_from_str(format!("{}T00:00:01+00:00", after_date_str).as_str(), "%+")
        .map_err(|e| format!("after_date_str {}", e.to_string()))?;
    let before = Tz::UTC
        .datetime_from_str(format!("{}T00:00:01+00:00", before_date_str).as_str(), "%+")
        .map_err(|e| format!("before_date_str {}", e.to_string()))?;

    let rrule = rrule
        .after(after - Duration::days(1))
        .before(before + Duration::days(1));
    let (events, _) = rrule.all(100);
    let event_vec = events
        .iter()
        .map(|dt| dt.date_naive().to_string())
        .collect();
    Ok(event_vec)
}

rustler::init!("Elixir.TapiRrule", [r_range]);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rrule_range_works() {
        let dates = rrule_range(
            "DTSTART:20120201T000000Z\nRRULE:FREQ=DAILY;COUNT=3".to_string(),
            "2012-02-01".to_string(),
            "2012-04-01".to_string(),
        )
        .unwrap();
        assert_eq!(
            vec![
                "2012-02-01".to_string(),
                "2012-02-02".to_string(),
                "2012-02-03".to_string()
            ],
            dates
        );
    }

    #[test]
    fn should_return_error_when_rrule_str_is_invalid() {
        let err_string = rrule_range(
            "DTSTART:20120201T000000Z\nRRULE:FREQ=DAILY;COUNT=3s".to_string(),
            "2012-02-01".to_string(),
            "2012-04-01".to_string(),
        )
        .unwrap_err();
        assert_eq!(err_string, "rrule_str invalid format".to_string());
    }

    #[test]
    fn should_return_error_when_after_date_str_is_invalid() {
        let err_string = rrule_range(
            "DTSTART:20120201T000000Z\nRRULE:FREQ=DAILY;COUNT=3".to_string(),
            "2012-02-01ss".to_string(),
            "2012-04-01".to_string(),
        )
        .unwrap_err();
        assert_eq!(
            err_string,
            "after_date_str input contains invalid characters".to_string()
        );
    }

    #[test]
    fn should_return_error_when_before_date_str_is_invalid() {
        let err_string = rrule_range(
            "DTSTART:20120201T000000Z\nRRULE:FREQ=DAILY;COUNT=3".to_string(),
            "2012-02-01".to_string(),
            "2012-04-01ss".to_string(),
        )
        .unwrap_err();
        assert_eq!(
            err_string,
            "before_date_str input contains invalid characters".to_string()
        );
    }
}
