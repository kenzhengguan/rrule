use chrono::{DateTime, Duration, TimeZone};
use rrule::{RRuleSet, Tz};

#[rustler::nif]
fn r_range(
    rrule_str: String,
    after_date_str: String,
    before_date_str: Option<String>,
) -> Result<Vec<String>, String> {
    rrule_range(rrule_str, after_date_str, before_date_str)
}

fn rrule_range(
    rrule_str: String,
    after_date_str: String,
    before_date_str: Option<String>,
) -> Result<Vec<String>, String> {
    let rrule: RRuleSet = rrule_str.parse().map_err(|_| "rrule_str invalid format")?;

    let after: DateTime<Tz> = Tz::UTC
        .datetime_from_str(format!("{}T00:00:01+00:00", after_date_str).as_str(), "%+")
        .map_err(|e| format!("after_date_str {}", e.to_string()))?;
    let rrule = match before_date_str {
        Some(date_str) => {
            let before = Tz::UTC
                .datetime_from_str(format!("{}T00:00:01+00:00", date_str).as_str(), "%+")
                .map_err(|e| format!("before_date_str {}", e.to_string()))?;

            rrule
                .after(after - Duration::days(1))
                .before(before + Duration::days(1))
        }
        None => rrule.after(after - Duration::days(1)),
    };
    // let before = Tz::UTC
    //     .datetime_from_str(format!("{}T00:00:01+00:00", before_date_str).as_str(), "%+")
    //     .map_err(|e| format!("before_date_str {}", e.to_string()))?;

    // let rrule = rrule
    //     .after(after - Duration::days(1))
    //     .before(before + Duration::days(1));
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
            Some("2012-04-01".to_string()),
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
            Some("2012-04-01".to_string()),
        )
        .unwrap_err();
        assert_eq!(err_string, "rrule_str invalid format".to_string());
    }

    #[test]
    fn should_return_error_when_after_date_str_is_invalid() {
        let err_string = rrule_range(
            "DTSTART:20120201T000000Z\nRRULE:FREQ=DAILY;COUNT=3".to_string(),
            "2012-02-01ss".to_string(),
            Some("2012-04-01".to_string()),
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
            Some("2012-04-01ss".to_string()),
        )
        .unwrap_err();
        assert_eq!(
            err_string,
            "before_date_str input contains invalid characters".to_string()
        );
    }

    #[test]
    fn should_return_100_days_when_before_date_str_is_none() {
        let dates = rrule_range(
            "DTSTART:20120201T000000Z\nRRULE:FREQ=DAILY;COUNT=100".to_string(),
            "2012-02-01".to_string(),
            None,
        )
        .unwrap();
        assert_eq!(
            vec![
                "2012-02-01".to_string(),
                "2012-02-02".to_string(),
                "2012-02-03".to_string(),
                "2012-02-04".to_string(),
                "2012-02-05".to_string(),
                "2012-02-06".to_string(),
                "2012-02-07".to_string(),
                "2012-02-08".to_string(),
                "2012-02-09".to_string(),
                "2012-02-10".to_string(),
                "2012-02-11".to_string(),
                "2012-02-12".to_string(),
                "2012-02-13".to_string(),
                "2012-02-14".to_string(),
                "2012-02-15".to_string(),
                "2012-02-16".to_string(),
                "2012-02-17".to_string(),
                "2012-02-18".to_string(),
                "2012-02-19".to_string(),
                "2012-02-20".to_string(),
                "2012-02-21".to_string(),
                "2012-02-22".to_string(),
                "2012-02-23".to_string(),
                "2012-02-24".to_string(),
                "2012-02-25".to_string(),
                "2012-02-26".to_string(),
                "2012-02-27".to_string(),
                "2012-02-28".to_string(),
                "2012-02-29".to_string(),
                "2012-03-01".to_string(),
                "2012-03-02".to_string(),
                "2012-03-03".to_string(),
                "2012-03-04".to_string(),
                "2012-03-05".to_string(),
                "2012-03-06".to_string(),
                "2012-03-07".to_string(),
                "2012-03-08".to_string(),
                "2012-03-09".to_string(),
                "2012-03-10".to_string(),
                "2012-03-11".to_string(),
                "2012-03-12".to_string(),
                "2012-03-13".to_string(),
                "2012-03-14".to_string(),
                "2012-03-15".to_string(),
                "2012-03-16".to_string(),
                "2012-03-17".to_string(),
                "2012-03-18".to_string(),
                "2012-03-19".to_string(),
                "2012-03-20".to_string(),
                "2012-03-21".to_string(),
                "2012-03-22".to_string(),
                "2012-03-23".to_string(),
                "2012-03-24".to_string(),
                "2012-03-25".to_string(),
                "2012-03-26".to_string(),
                "2012-03-27".to_string(),
                "2012-03-28".to_string(),
                "2012-03-29".to_string(),
                "2012-03-30".to_string(),
                "2012-03-31".to_string(),
                "2012-04-01".to_string(),
                "2012-04-02".to_string(),
                "2012-04-03".to_string(),
                "2012-04-04".to_string(),
                "2012-04-05".to_string(),
                "2012-04-06".to_string(),
                "2012-04-07".to_string(),
                "2012-04-08".to_string(),
                "2012-04-09".to_string(),
                "2012-04-10".to_string(),
                "2012-04-11".to_string(),
                "2012-04-12".to_string(),
                "2012-04-13".to_string(),
                "2012-04-14".to_string(),
                "2012-04-15".to_string(),
                "2012-04-16".to_string(),
                "2012-04-17".to_string(),
                "2012-04-18".to_string(),
                "2012-04-19".to_string(),
                "2012-04-20".to_string(),
                "2012-04-21".to_string(),
                "2012-04-22".to_string(),
                "2012-04-23".to_string(),
                "2012-04-24".to_string(),
                "2012-04-25".to_string(),
                "2012-04-26".to_string(),
                "2012-04-27".to_string(),
                "2012-04-28".to_string(),
                "2012-04-29".to_string(),
                "2012-04-30".to_string(),
                "2012-05-01".to_string(),
                "2012-05-02".to_string(),
                "2012-05-03".to_string(),
                "2012-05-04".to_string(),
                "2012-05-05".to_string(),
                "2012-05-06".to_string(),
                "2012-05-07".to_string(),
                "2012-05-08".to_string(),
                "2012-05-09".to_string(),
                "2012-05-10".to_string()
            ],
            dates
        );
    }
}
