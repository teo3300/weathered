use std::fmt;
use strum_macros::Display;

const BASE_URL: &str = "https://api.open-meteo.com/v1/forecast";

#[derive(Display, Copy, Clone)]
#[allow(non_camel_case_types)]
/// Enumerate temperature units
pub enum Temperature {
    celsius,
    fahrenheit,
}

#[derive(Display, Copy, Clone)]
#[allow(non_camel_case_types)]
/// Enumerate windspeed units
pub enum Speed {
    kmh,
    ms,
    mph,
    kn,
}

#[derive(Display, Copy, Clone)]
#[allow(non_camel_case_types)]
/// Enumerate precipitation units
pub enum Precipitation {
    mm,
    inch,
}

#[derive(Display, Copy, Clone)]
#[allow(non_camel_case_types)]
/// Enumerate valid time formats
pub enum TimeFormat {
    iso8601,
    unixtime,
}

#[derive(Display, Copy, Clone)]
#[allow(non_camel_case_types)]
/// Enumerate cell selection
pub enum Cell {
    land,
    sea,
    nearest,
}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
/// Enumerate timezones
/// Provide timezone in format
///     ("Continent", "Country")
/// or specify automatic timezone
///     "auto"
pub enum Timezone<'a> {
    explicit(&'a str, &'a str),
    auto,
}

impl<'a> Timezone<'a> {
    pub fn get(&self) -> String {
        match self {
            Timezone::explicit(continent, country) => continent.to_string() + "%2F" + country,
            Timezone::auto => "auto".to_owned(),
        }
    }
}

#[derive(Display, Copy, Clone)]
#[allow(non_camel_case_types)]
/// Enumerate settings and related value
pub enum Settings<'a> {
    elevation(f32),
    current_weather(bool),
    temperature_unit(Temperature),
    windspeed_unit(Speed),
    precipitation_unit(Precipitation),
    timeformat(TimeFormat),
    timezone(Timezone<'a>),
    past_days(u8),
    forecast_days(u8),
    start_date(&'a str),
    end_date(&'a str),
    cell_selection(Cell),
}

impl<'a> Settings<'a> {
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
            Settings::timezone(t) => t.get(),
            Settings::start_date(t) | Settings::end_date(t) => t.to_string(),
        }
    }
}

#[derive(Display, Copy, Clone)]
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

#[derive(Display, Copy, Clone)]
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
        format!("{}_{}hPa", self, value)
    }
}

#[derive(Display, Copy, Clone)]
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

// Implement typestates to avoid requesting an URL without coordinates
// region:    --- ForecastStates
#[derive(Default)]
pub struct NoCoordinates;
#[derive(Default)]
pub struct Coordinates {
    latitude: f32,
    longitude: f32,
}
// endregion: --- ForecastStates

// Generic Forecast type, with or without coordinates
/// Basic data structure to keep all request's data
#[derive(Default)]
pub struct Forecast<'a, C> {
    coordinates: C,
    settings: Vec<Settings<'a>>,
    hourly: Vec<Hourly>,
    pressure_var: Vec<PressureVar>,
    daily: Vec<Daily>,
}

// Create a generic forecast without coordinates,
// Provide a method to set coordinates only if they are not already present
impl<'a> Forecast<'a, NoCoordinates> {
    /// Initialize Forecast object
    pub fn new() -> Self {
        Forecast::default()
    }

    /// Specify coordinates (latitude, longitude)
    /// These two are the only mandatory fields
    pub fn coord(self, latitude: f32, longitude: f32) -> Forecast<'a, Coordinates> {
        Forecast {
            coordinates: Coordinates {
                latitude,
                longitude,
            },
            settings: self.settings,
            hourly: self.hourly,
            pressure_var: self.pressure_var,
            daily: self.daily,
        }
    }
}

// Allow creating final URL only if it's valid
impl<'a> Forecast<'a, Coordinates> {
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
            url.push_str(format!("&{}={}", el, el.get()).as_str());
        }
        if !self.hourly.is_empty() {
            url.push_str("&hourly=");
            for el in &self.hourly {
                url.push_str(format!(",{}", el).as_str());
            }
        }
        if !self.daily.is_empty() {
            url.push_str("&daily=");
            for el in &self.daily {
                url.push_str(format!(",{}", el).as_str());
            }
        }
        for el in &self.pressure_var {
            url.push_str(format!("&{}", el.get()).as_str());
        }
        url
    }
}

// Used for format! macro
impl<'a> fmt::Display for Forecast<'a, Coordinates> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_sring())
    }
}

// Generic settings can be added both before and after coordinates
impl<'a, C> Forecast<'a, C> {
    /// Add optional settings
    pub fn settings(mut self, settings: &[Settings<'a>]) -> Self {
        for el in settings.iter() {
            self.settings.push(*el);
        }
        self
    }

    /// Get hourly value for a specific data
    pub fn hourly(mut self, hourly: &[Hourly]) -> Self {
        for el in hourly.iter() {
            self.hourly.push(*el);
        }
        self
    }

    /// Get daily value for a specific data
    pub fn daily(mut self, daily: &[Daily]) -> Self {
        for el in daily.iter() {
            self.daily.push(*el);
        }
        self
    }

    /// Get Pressure Level-related variables
    pub fn pressure_var(mut self, pressure_var: &[PressureVar]) -> Self {
        for el in pressure_var.iter() {
            self.pressure_var.push(*el);
        }
        self
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn url_creation() {
        let forecast = Forecast::new()
            .coord(50.1, 50.1)
            .settings(&[
                Settings::elevation(1000.1),
                Settings::timezone(Timezone::explicit("Europe", "London")),
            ])
            .hourly(&[Hourly::rain, Hourly::cape])
            .daily(&[Daily::sunrise, Daily::sunset])
            .pressure_var(&[PressureVar::dewpoint(50), PressureVar::windspeed(30)]);

        assert_eq!(
            forecast.to_string(),
            "https://api.open-meteo.com/v1/forecast?latitude=50.1&longitude=50.1&elevation=1000.1&timezone=Europe%2FLondon&hourly=,rain,cape&daily=,sunrise,sunset&dewpoint_50hPa&windspeed_30hPa")
    }
}
