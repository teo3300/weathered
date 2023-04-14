use std::fmt;
use strum_macros::Display;

const BASE_URL: &'static str = "https://api.open-meteo.com/v1/forecast";

#[derive(Display)]
#[allow(non_camel_case_types)]
/// Enumerate temperature units
pub enum Temperature {
    celsius,
    fahrenheit,
}

#[derive(Display)]
#[allow(non_camel_case_types)]
/// Enumerate windspeed units
pub enum Speed {
    kmh,
    ms,
    mph,
    kn,
}

#[derive(Display)]
#[allow(non_camel_case_types)]
/// Enumerate precipitation units
pub enum Precipitation {
    mm,
    inch,
}

#[derive(Display)]
#[allow(non_camel_case_types)]
/// Enumerate valid time formats
pub enum TimeFormat {
    iso8601,
    unixtime,
}

#[derive(Display)]
#[allow(non_camel_case_types)]
/// Enumerate cell selection
pub enum Cell {
    land,
    sea,
    nearest,
}

#[derive(Display)]
#[allow(non_camel_case_types)]
/// Enumerate settings and related value
pub enum Settings {
    elevation(f32),
    current_weather(bool),
    temperature_unit(Temperature),
    windspeed_unit(Speed),
    precipitation_unit(Precipitation),
    timeformat(TimeFormat),
    timezone(String),
    past_days(u8),
    forecast_days(u8),
    start_date(String),
    end_date(String),
    cell_selection(Cell),
}

impl Settings {
    fn get(&self) -> String {
        match self {
            Settings::elevation(t) => t.to_string(),
            Settings::current_weather(t) => t.to_string(),
            Settings::temperature_unit(t) => t.to_string(),
            Settings::windspeed_unit(t) => t.to_string(),
            Settings::precipitation_unit(t) => t.to_string(),
            Settings::cell_selection(t) => t.to_string(),
            Settings::timeformat(t) => t.to_string(),
            Settings::past_days(t) | Settings::forecast_days(t) => t.to_string(),
            Settings::timezone(t) | Settings::start_date(t) | Settings::end_date(t) => {
                t.to_string()
            }
        }
    }
}

#[derive(Display)]
#[allow(non_camel_case_types)]
/// Enumerate all Hourly flags
pub enum Hourly {
    temperature_2m,
    relative_humidity_2m,
    dewpoint_2m,
    apparent_temperature,
    pressure_msl,
    surface_pressure,
    cloudcover,
    cloudcover_low,
    cloudcover_mid,
    cloudcover_high,
    windspeed_10m,
    windspeed_80m,
    windspeed_120m,
    windspeed_180m,
    winddirection_10m,
    windspeedtion_80m,
    windspeedtion_120m,
    windspeedtion_180m,
    windgusts_10m,
    shortwave_radiation,
    direct_radiation,
    direct_normal_irradiance,
    diffuse_radiation,
    vapor_pressure_deficit,
    cape,
    evapotranspiration,
    et0_fao_evapotranspiration,
    precipitation,
    snowfall,
    precipitation_probability,
    rain,
    showers,
    weathercode,
    snow_depth,
    freezinglevel_height,
    visibility,
    soil_temperature_0cm,
    soil_temperature_6cm,
    soil_temperature_18cm,
    soil_temperature_54cm,
    soil_moisture_0_1cm,
    soil_moisture_1_3cm,
    soil_moisture_4_9cm,
    soil_moisture_9_27cm,
    soil_moisture_27_81cm,
    is_day,
}

#[derive(Display)]
#[allow(non_camel_case_types)]
/// Enumerate available pressure variables
pub enum PressureVar {
    temperature(u32),
    relativehumidity(u32),
    dewpoint(u32),
    cloudcover(u32),
    windspeed(u32),
    winddirection(u32),
    geopotential_height(u32),
}

// TODO: collapse value to valid ones:
// see https://open-meteo.com/en/docs > Pressure Level Variables
/// Compose the sting to obtain valid variables
impl PressureVar {
    fn get(&self) -> String {
        let value = match self {
            PressureVar::temperature(h)
            | PressureVar::relativehumidity(h)
            | PressureVar::dewpoint(h)
            | PressureVar::cloudcover(h)
            | PressureVar::windspeed(h)
            | PressureVar::winddirection(h)
            | PressureVar::geopotential_height(h) => h,
        };
        format!("{}_{}hPa", self.to_string(), value)
    }
}

#[derive(Display)]
#[allow(non_camel_case_types)]
/// Enumerate Daily data flags
pub enum Daily {
    temperature_2m_max,
    temperature_2m_min,
    apparent_temperature_max,
    apparent_temperature_min,
    precipitation_sum,
    rain_sum,
    showers_sum,
    swnofall_sum,
    precipitation_hours,
    precipitation_probability_max,
    precipitation_probability_min,
    precipitation_probability_mean,
    weathercode,
    sunrise,
    sunset,
    windspeed_10m_max,
    windgusts_10m_max,
    winddirection_10m_dominant,
    shortwave_radiation_sum,
    et0_fao_evapotranspiration,
    uv_index_max,
    uv_index_clear_sky_max,
}

/// Geographic coordinates
pub struct Coordinates {
    latitude: f32,
    longitude: f32,
}

/// Basic data structure to keep all request's data
pub struct Forecast {
    coordinates: Coordinates,
    settings: Vec<Settings>,
    hourly: Vec<Hourly>,
    pressure_var: Vec<PressureVar>,
    daily: Vec<Daily>,
}

impl Forecast {
    /// Initialize Forecast object
    pub fn new() -> Self {
        Forecast {
            coordinates: Coordinates {
                latitude: Default::default(),
                longitude: Default::default(),
            },
            settings: Vec::new(),
            hourly: Vec::new(),
            pressure_var: Vec::new(),
            daily: Vec::new(),
        }
    }

    /// Specify coordinates (latitude, longitude)
    /// These two are the only mandatory fields
    pub fn coord(mut self, latitude: f32, longitude: f32) -> Self {
        self.coordinates = Coordinates {
            latitude,
            longitude,
        };
        self
    }

    /// Add optional settings
    pub fn settings(mut self, setting: Settings) -> Self {
        self.settings.push(setting);
        self
    }

    /// Get hourly value for a specific data
    pub fn hourly(mut self, hourly: Hourly) -> Self {
        self.hourly.push(hourly);
        self
    }

    /// Get daily value for a specific data
    pub fn daily(mut self, daily: Daily) -> Self {
        self.daily.push(daily);
        self
    }

    /// Get Pressure Level-related variables
    pub fn pressure_var(mut self, pressure_var: PressureVar) -> Self {
        self.pressure_var.push(pressure_var);
        self
    }

    /// Convert the forecast struct into a valid URL
    fn to_sring(&self) -> String {
        let mut url = String::from(BASE_URL);
        url.push_str(
            format!(
                "?latitude={}&longitude={}",
                self.coordinates.latitude, self.coordinates.longitude
            )
            .as_str(),
        );
        for el in &self.settings {
            url.push_str(format!("&{}={}", el.to_string(), el.get()).as_str());
        }
        if self.hourly.len() > 0 {
            url.push_str("&hourly=");
            for el in &self.hourly {
                url.push_str(format!(",{}", el.to_string()).as_str());
            }
        }
        if self.daily.len() > 0 {
            url.push_str("&daily=");
            for el in &self.daily {
                url.push_str(format!(",{}", el.to_string()).as_str());
            }
        }
        for el in &self.pressure_var {
            url.push_str(format!("&{}", el.get()).as_str());
        }
        url
    }
}

impl fmt::Display for Forecast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_sring())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn url_creation() {
        let forecast = Forecast::new()
            .coord(50.1, 50.1)
            .settings(Settings::elevation(1000.1))
            .hourly(Hourly::rain)
            .hourly(Hourly::cape)
            .pressure_var(PressureVar::dewpoint(50))
            .daily(Daily::sunrise)
            .pressure_var(PressureVar::windspeed(30))
            .daily(Daily::sunset);

        assert_eq!(
            format!("{forecast}"),
            "https://api.open-meteo.com/v1/forecast?latitude=50.1&longitude=50.1&elevation=1000.1&hourly=,rain,cape&daily=,sunrise,sunset&dewpoint_50hPa&windspeed_30hPa")
    }
}
