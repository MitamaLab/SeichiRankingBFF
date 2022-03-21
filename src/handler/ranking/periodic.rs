use actix_web::{HttpRequest, HttpResponse, Responder};
use qstring::QString;

enum RankingType {
    Break,
    Build,
    PlayTime,
    Vote,
}

enum RankingTypeCoercionError {
    InvalidSpecifier
}

impl TryFrom<&str> for RankingType {
    type Error = RankingTypeCoercionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "break" => Ok(Self::Break),
            "build" => Ok(Self::Build),
            "playtime" => Ok(Self::PlayTime),
            "vote" => Ok(Self::Vote),
            _ => Err(RankingTypeCoercionError::InvalidSpecifier),
        }
    }
}

enum RankingPeriod {
    Total,
    Yearly,
    Monthly,
    Weekly,
    Daily,
}

enum RankingPeriodCoercionError {
    InvalidSpecifier
}

impl TryFrom<&str> for RankingPeriod {
    type Error = RankingPeriodCoercionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "total" => Ok(Self::Total),
            "yearly" => Ok(Self::Yearly),
            "monthly" => Ok(Self::Monthly),
            "weekly" => Ok(Self::Weekly),
            "daily" => Ok(Self::Daily),
            _ => Err(RankingPeriodCoercionError::InvalidSpecifier)
        }
    }
}
pub async fn periodic(req: HttpRequest) -> impl Responder {
    let qs: QString =  req.query_string().into();

    let _kind: RankingType = match qs.get("type").unwrap_or("break").try_into() {
        Ok(t) => t,
        Err(_e) => return HttpResponse::BadRequest().body("")
    };

    let _duration: RankingPeriod = match qs.get("duration").unwrap_or("total").try_into() {
        Ok(t) => t,
        Err(_e) => return HttpResponse::BadRequest().body("")
    };

    HttpResponse::Ok().json(vec![
        1
    ])
}