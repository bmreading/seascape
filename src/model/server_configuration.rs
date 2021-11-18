//! Items specific to a server configuraiton
use serde::{Deserialize, Serialize};

/// Represents a server's configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerConfiguration {
    /// The number of days we should retain log files.
    #[serde(
        rename = "LogFileRetentionDays",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_file_retention_days: Option<i32>,
    /// A value indicating whether this instance is first run.
    #[serde(
        rename = "IsStartupWizardCompleted",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_startup_wizard_completed: Option<bool>,
    /// The cache path.
    #[serde(rename = "CachePath", skip_serializing_if = "Option::is_none")]
    pub cache_path: Option<String>,
    /// The last known version that was ran using the configuration.
    #[serde(rename = "PreviousVersion", skip_serializing_if = "Option::is_none")]
    pub previous_version: Option<Box<super::Version>>,
    /// The stringified PreviousVersion to be stored/loaded.
    #[serde(rename = "PreviousVersionStr", skip_serializing_if = "Option::is_none")]
    pub previous_version_str: Option<String>,
    /// A value indicating whether to enable automatic port forwarding.
    #[serde(rename = "EnableUPnP", skip_serializing_if = "Option::is_none")]
    pub enable_upn_p: Option<bool>,
    /// A value indicating whether to enable prometheus metrics exporting.
    #[serde(rename = "EnableMetrics", skip_serializing_if = "Option::is_none")]
    pub enable_metrics: Option<bool>,
    /// The public mapped port.
    #[serde(rename = "PublicPort", skip_serializing_if = "Option::is_none")]
    pub public_port: Option<i32>,
    /// A value indicating whether the http port should be mapped as part of UPnP automatic port forwarding.
    #[serde(
        rename = "UPnPCreateHttpPortMap",
        skip_serializing_if = "Option::is_none"
    )]
    pub upn_p_create_http_port_map: Option<bool>,
    /// Client udp port range.
    #[serde(rename = "UDPPortRange", skip_serializing_if = "Option::is_none")]
    pub udp_port_range: Option<String>,
    /// A value indicating whether IPV6 capability is enabled.
    #[serde(rename = "EnableIPV6", skip_serializing_if = "Option::is_none")]
    pub enable_ipv6: Option<bool>,
    /// A value indicating whether IPV4 capability is enabled.
    #[serde(rename = "EnableIPV4", skip_serializing_if = "Option::is_none")]
    pub enable_ipv4: Option<bool>,
    /// A value indicating whether detailed ssdp logs are sent to the console/log.  \"Emby.Dlna\": \"Debug\" must be set in logging.default.json for this property to work.
    #[serde(rename = "EnableSSDPTracing", skip_serializing_if = "Option::is_none")]
    pub enable_ssdp_tracing: Option<bool>,
    /// A value indicating whether an IP address is to be used to filter the detailed ssdp logs that are being sent to the console/log.  If the setting \"Emby.Dlna\": \"Debug\" msut be set in logging.default.json for this property to work.
    #[serde(rename = "SSDPTracingFilter", skip_serializing_if = "Option::is_none")]
    pub ssdp_tracing_filter: Option<String>,
    /// The number of times SSDP UDP messages are sent.
    #[serde(rename = "UDPSendCount", skip_serializing_if = "Option::is_none")]
    pub udp_send_count: Option<i32>,
    /// The delay between each groups of SSDP messages (in ms).
    #[serde(rename = "UDPSendDelay", skip_serializing_if = "Option::is_none")]
    pub udp_send_delay: Option<i32>,
    /// A value indicating whether address names that match MediaBrowser.Model.Configuration.ServerConfiguration.VirtualInterfaceNames should be Ignore for the purposes of binding.
    #[serde(
        rename = "IgnoreVirtualInterfaces",
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_virtual_interfaces: Option<bool>,
    /// A value indicating the interfaces that should be ignored. The list can be comma separated. <seealso cref=\"P:MediaBrowser.Model.Configuration.ServerConfiguration.IgnoreVirtualInterfaces\" />.
    #[serde(
        rename = "VirtualInterfaceNames",
        skip_serializing_if = "Option::is_none"
    )]
    pub virtual_interface_names: Option<String>,
    /// The time (in seconds) between the pings of SSDP gateway monitor.
    #[serde(
        rename = "GatewayMonitorPeriod",
        skip_serializing_if = "Option::is_none"
    )]
    pub gateway_monitor_period: Option<i32>,
    /// A value indicating whether multi-socket binding is available.
    #[serde(
        rename = "EnableMultiSocketBinding",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_multi_socket_binding: Option<bool>,
    /// A value indicating whether all IPv6 interfaces should be treated as on the internal network.  Depending on the address range implemented ULA ranges might not be used.
    #[serde(
        rename = "TrustAllIP6Interfaces",
        skip_serializing_if = "Option::is_none"
    )]
    pub trust_all_ip6_interfaces: Option<bool>,
    /// The ports that HDHomerun uses.
    #[serde(rename = "HDHomerunPortRange", skip_serializing_if = "Option::is_none")]
    pub hd_homerun_port_range: Option<String>,
    /// PublishedServerUri to advertise for specific subnets.
    #[serde(
        rename = "PublishedServerUriBySubnet",
        skip_serializing_if = "Option::is_none"
    )]
    pub published_server_uri_by_subnet: Option<Vec<String>>,
    /// A value indicating whether Autodiscovery tracing is enabled.
    #[serde(
        rename = "AutoDiscoveryTracing",
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_discovery_tracing: Option<bool>,
    /// A value indicating whether Autodiscovery is enabled.
    #[serde(rename = "AutoDiscovery", skip_serializing_if = "Option::is_none")]
    pub auto_discovery: Option<bool>,
    /// The public HTTPS port.
    #[serde(rename = "PublicHttpsPort", skip_serializing_if = "Option::is_none")]
    pub public_https_port: Option<i32>,
    /// The HTTP server port number.
    #[serde(
        rename = "HttpServerPortNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub http_server_port_number: Option<i32>,
    /// The HTTPS server port number.
    #[serde(rename = "HttpsPortNumber", skip_serializing_if = "Option::is_none")]
    pub https_port_number: Option<i32>,
    /// A value indicating whether to use HTTPS.
    #[serde(rename = "EnableHttps", skip_serializing_if = "Option::is_none")]
    pub enable_https: Option<bool>,
    #[serde(
        rename = "EnableNormalizedItemByNameIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_normalized_item_by_name_ids: Option<bool>,
    /// The filesystem path of an X.509 certificate to use for SSL.
    #[serde(rename = "CertificatePath", skip_serializing_if = "Option::is_none")]
    pub certificate_path: Option<String>,
    /// The password required to access the X.509 certificate data.
    #[serde(
        rename = "CertificatePassword",
        skip_serializing_if = "Option::is_none"
    )]
    pub certificate_password: Option<String>,
    /// A value indicating whether this instance is port authorized.
    #[serde(rename = "IsPortAuthorized", skip_serializing_if = "Option::is_none")]
    pub is_port_authorized: Option<bool>,
    /// A value indicating whether quick connect is available for use on this server.
    #[serde(
        rename = "QuickConnectAvailable",
        skip_serializing_if = "Option::is_none"
    )]
    pub quick_connect_available: Option<bool>,
    /// A value indicating whether access outside of the LAN is permitted.
    #[serde(rename = "EnableRemoteAccess", skip_serializing_if = "Option::is_none")]
    pub enable_remote_access: Option<bool>,
    /// A value indicating whether [enable case sensitive item ids].
    #[serde(
        rename = "EnableCaseSensitiveItemIds",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_case_sensitive_item_ids: Option<bool>,
    #[serde(
        rename = "DisableLiveTvChannelUserDataName",
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_live_tv_channel_user_data_name: Option<bool>,
    /// The metadata path.
    #[serde(rename = "MetadataPath", skip_serializing_if = "Option::is_none")]
    pub metadata_path: Option<String>,
    #[serde(
        rename = "MetadataNetworkPath",
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_network_path: Option<String>,
    /// The preferred metadata language.
    #[serde(
        rename = "PreferredMetadataLanguage",
        skip_serializing_if = "Option::is_none"
    )]
    pub preferred_metadata_language: Option<String>,
    /// The metadata country code.
    #[serde(
        rename = "MetadataCountryCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_country_code: Option<String>,
    /// Characters to be replaced with a ' ' in strings to create a sort name.
    #[serde(
        rename = "SortReplaceCharacters",
        skip_serializing_if = "Option::is_none"
    )]
    pub sort_replace_characters: Option<Vec<String>>,
    /// Characters to be removed from strings to create a sort name.
    #[serde(
        rename = "SortRemoveCharacters",
        skip_serializing_if = "Option::is_none"
    )]
    pub sort_remove_characters: Option<Vec<String>>,
    /// Words to be removed from strings to create a sort name.
    #[serde(rename = "SortRemoveWords", skip_serializing_if = "Option::is_none")]
    pub sort_remove_words: Option<Vec<String>>,
    /// The minimum percentage of an item that must be played in order for playstate to be updated.
    #[serde(rename = "MinResumePct", skip_serializing_if = "Option::is_none")]
    pub min_resume_pct: Option<i32>,
    /// The maximum percentage of an item that can be played while still saving playstate. If this percentage is crossed playstate will be reset to the beginning and the item will be marked watched.
    #[serde(rename = "MaxResumePct", skip_serializing_if = "Option::is_none")]
    pub max_resume_pct: Option<i32>,
    /// The minimum duration that an item must have in order to be eligible for playstate updates..
    #[serde(
        rename = "MinResumeDurationSeconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub min_resume_duration_seconds: Option<i32>,
    /// The minimum minutes of a book that must be played in order for playstate to be updated.
    #[serde(rename = "MinAudiobookResume", skip_serializing_if = "Option::is_none")]
    pub min_audiobook_resume: Option<i32>,
    /// The remaining minutes of a book that can be played while still saving playstate. If this percentage is crossed playstate will be reset to the beginning and the item will be marked watched.
    #[serde(rename = "MaxAudiobookResume", skip_serializing_if = "Option::is_none")]
    pub max_audiobook_resume: Option<i32>,
    /// The delay in seconds that we will wait after a file system change to try and discover what has been added/removed  Some delay is necessary with some items because their creation is not atomic.  It involves the creation of several  different directories and files.
    #[serde(
        rename = "LibraryMonitorDelay",
        skip_serializing_if = "Option::is_none"
    )]
    pub library_monitor_delay: Option<i32>,
    /// A value indicating whether [enable dashboard response caching].
    #[serde(
        rename = "EnableDashboardResponseCaching",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_dashboard_response_caching: Option<bool>,
    /// The image saving convention.
    #[serde(
        rename = "ImageSavingConvention",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_saving_convention: Option<Box<super::ImageSavingConvention>>,
    #[serde(rename = "MetadataOptions", skip_serializing_if = "Option::is_none")]
    pub metadata_options: Option<Vec<super::MetadataOptions>>,
    #[serde(
        rename = "SkipDeserializationForBasicTypes",
        skip_serializing_if = "Option::is_none"
    )]
    pub skip_deserialization_for_basic_types: Option<bool>,
    #[serde(rename = "ServerName", skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    #[serde(rename = "BaseUrl", skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[serde(rename = "UICulture", skip_serializing_if = "Option::is_none")]
    pub ui_culture: Option<String>,
    #[serde(rename = "SaveMetadataHidden", skip_serializing_if = "Option::is_none")]
    pub save_metadata_hidden: Option<bool>,
    #[serde(rename = "ContentTypes", skip_serializing_if = "Option::is_none")]
    pub content_types: Option<Vec<super::NameValuePair>>,
    #[serde(
        rename = "RemoteClientBitrateLimit",
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_client_bitrate_limit: Option<i32>,
    #[serde(rename = "EnableFolderView", skip_serializing_if = "Option::is_none")]
    pub enable_folder_view: Option<bool>,
    #[serde(
        rename = "EnableGroupingIntoCollections",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_grouping_into_collections: Option<bool>,
    #[serde(
        rename = "DisplaySpecialsWithinSeasons",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_specials_within_seasons: Option<bool>,
    /// The subnets that are deemed to make up the LAN.
    #[serde(
        rename = "LocalNetworkSubnets",
        skip_serializing_if = "Option::is_none"
    )]
    pub local_network_subnets: Option<Vec<String>>,
    /// The interface addresses which Jellyfin will bind to. If empty, all interfaces will be used.
    #[serde(
        rename = "LocalNetworkAddresses",
        skip_serializing_if = "Option::is_none"
    )]
    pub local_network_addresses: Option<Vec<String>>,
    #[serde(rename = "CodecsUsed", skip_serializing_if = "Option::is_none")]
    pub codecs_used: Option<Vec<String>>,
    #[serde(rename = "PluginRepositories", skip_serializing_if = "Option::is_none")]
    pub plugin_repositories: Option<Vec<super::RepositoryInfo>>,
    #[serde(
        rename = "EnableExternalContentInSuggestions",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_external_content_in_suggestions: Option<bool>,
    /// A value indicating whether the server should force connections over HTTPS.
    #[serde(rename = "RequireHttps", skip_serializing_if = "Option::is_none")]
    pub require_https: Option<bool>,
    #[serde(
        rename = "EnableNewOmdbSupport",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_new_omdb_support: Option<bool>,
    /// The filter for remote IP connectivity. Used in conjuntion with <seealso cref=\"P:MediaBrowser.Model.Configuration.ServerConfiguration.IsRemoteIPFilterBlacklist\" />.
    #[serde(rename = "RemoteIPFilter", skip_serializing_if = "Option::is_none")]
    pub remote_ip_filter: Option<Vec<String>>,
    /// A value indicating whether <seealso cref=\"P:MediaBrowser.Model.Configuration.ServerConfiguration.RemoteIPFilter\" /> contains a blacklist or a whitelist. Default is a whitelist.
    #[serde(
        rename = "IsRemoteIPFilterBlacklist",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_remote_ip_filter_blacklist: Option<bool>,
    #[serde(
        rename = "ImageExtractionTimeoutMs",
        skip_serializing_if = "Option::is_none"
    )]
    pub image_extraction_timeout_ms: Option<i32>,
    #[serde(rename = "PathSubstitutions", skip_serializing_if = "Option::is_none")]
    pub path_substitutions: Option<Vec<super::PathSubstitution>>,
    #[serde(rename = "UninstalledPlugins", skip_serializing_if = "Option::is_none")]
    pub uninstalled_plugins: Option<Vec<String>>,
    /// A value indicating whether slow server responses should be logged as a warning.
    #[serde(
        rename = "EnableSlowResponseWarning",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_slow_response_warning: Option<bool>,
    /// The threshold for the slow response time warning in ms.
    #[serde(
        rename = "SlowResponseThresholdMs",
        skip_serializing_if = "Option::is_none"
    )]
    pub slow_response_threshold_ms: Option<i64>,
    /// The cors hosts.
    #[serde(rename = "CorsHosts", skip_serializing_if = "Option::is_none")]
    pub cors_hosts: Option<Vec<String>>,
    /// The known proxies.
    #[serde(rename = "KnownProxies", skip_serializing_if = "Option::is_none")]
    pub known_proxies: Option<Vec<String>>,
    /// The number of days we should retain activity logs.
    #[serde(
        rename = "ActivityLogRetentionDays",
        skip_serializing_if = "Option::is_none"
    )]
    pub activity_log_retention_days: Option<i32>,
    /// The how the library scan fans out.
    #[serde(
        rename = "LibraryScanFanoutConcurrency",
        skip_serializing_if = "Option::is_none"
    )]
    pub library_scan_fanout_concurrency: Option<i32>,
    /// The how many metadata refreshes can run concurrently.
    #[serde(
        rename = "LibraryMetadataRefreshConcurrency",
        skip_serializing_if = "Option::is_none"
    )]
    pub library_metadata_refresh_concurrency: Option<i32>,
    /// A value indicating whether older plugins should automatically be deleted from the plugin folder.
    #[serde(rename = "RemoveOldPlugins", skip_serializing_if = "Option::is_none")]
    pub remove_old_plugins: Option<bool>,
}
