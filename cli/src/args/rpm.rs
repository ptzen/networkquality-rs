use clap::Args;

#[derive(Debug, Args)]
pub struct RpmArgs {
    /// The endpoint to get the responsiveness config from. Should be JSON in
    /// the form:
    ///
    /// {
    ///     "version": number,
    ///     "test_endpoint": string?,
    ///     "urls": {
    ///         "small_https_download_url": string,
    ///         "large_https_download_url": string,
    ///         "https_upload_url": string
    ///     }
    /// }
    #[clap(short = 'c', long = "config")]
    pub config: Option<String>,
    /// The large file endpoint which should be multiple GBs.
    #[clap(
        short = 'l',
        long = "large",
        default_value = "https://aim.cloudflare.com/responsiveness/api/v1/large"
    )]
    pub large_download_url: String,
    /// The small file endpoint which should be very small, only a few bytes.
    #[clap(
        short = 's',
        long = "small",
        default_value = "https://aim.cloudflare.com/cdn-cgi/build/nginx-ssl"
    )]
    pub small_download_url: String,
    /// The upload url which accepts an arbitrary amount of data.
    #[clap(
        short = 'u',
        long = "upload",
        default_value = "https://aim.cloudflare.com/responsiveness/api/v1/upload"
    )]
    pub upload_url: String,
    /// The number of intervals to use when calculating the moving average.
    #[clap(long = "mad", default_value = "4")]
    pub moving_average_distance: usize,
    /// How far a measurement is allowed to be from the previous moving average
    /// before the measurement is considered unstable.
    #[clap(long = "std", default_value = "0.05")]
    pub std_tolerance: f64,
    /// Determines which percentile to use for averaging when calculating the
    /// trimmed mean of throughputs or RPM scores. A value of `0.95` means to
    /// only use values in the 95th percentile to calculate an average.
    #[clap(long = "trim", default_value = "0.95")]
    pub trimmed_mean_percent: f64,
    /// The maximum number of loaded connections that the test can use to
    /// saturate the network.
    #[clap(long = "max-load", default_value = "16")]
    pub max_loaded_connections: usize,
    /// The duration between interval updates in milliseconds (ms).
    #[clap(long = "interval-duration", default_value = "1000")]
    pub interval_duration_ms: u64,
    /// The overall test duration in milliseconds (ms).
    #[clap(long = "test-duration", default_value = "1000")]
    pub test_duration_ms: u64,
}

impl Default for RpmArgs {
    fn default() -> Self {
        Self {
            config: None,
            large_download_url: "https://aim.cloudflare.com/responsiveness/api/v1/large"
                .to_string(),
            small_download_url: "https://aim.cloudflare.com/cdn-cgi/build/nginx-ssl".to_string(),
            upload_url: "https://aim.cloudflare.com/responsiveness/api/v1/upload".to_string(),
            moving_average_distance: 4,
            std_tolerance: 0.05,
            trimmed_mean_percent: 0.95,
            max_loaded_connections: 16,
            interval_duration_ms: 1_000, // 1s
            test_duration_ms: 20_000,    // 20s
        }
    }
}
