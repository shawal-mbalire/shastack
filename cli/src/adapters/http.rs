use crate::domain::errors::ShaError;
use crate::domain::ports::HttpPort;

pub struct RealHttp;

impl HttpPort for RealHttp {
    fn get(&self, url: &str) -> Result<Vec<u8>, ShaError> {
        let response = reqwest::blocking::get(url)
            .map_err(|e| ShaError::NetworkError(e.to_string()))?;
        if !response.status().is_success() {
            return Err(ShaError::NetworkError(format!("HTTP {} for {}", response.status(), url)));
        }
        response.bytes()
            .map(|b| b.to_vec())
            .map_err(|e| ShaError::NetworkError(e.to_string()))
    }

    fn check_latest_version(&self, owner: &str, repo: &str, current: &str) -> Result<Option<String>, ShaError> {
        let status = self_update::backends::github::Update::configure()
            .repo_owner(owner)
            .repo_name(repo)
            .bin_name("sha")
            .show_download_progress(false)
            .current_version(current)
            .no_confirm(true)
            .build()
            .map_err(|e| ShaError::NetworkError(e.to_string()))?;

        let latest = status.get_latest_release()
            .map_err(|e| ShaError::NetworkError(e.to_string()))?;

        if self_update::version::bump_is_greater(current, &latest.version)
            .map_err(|e| ShaError::NetworkError(e.to_string()))?
        {
            Ok(Some(latest.version))
        } else {
            Ok(None)
        }
    }
}
