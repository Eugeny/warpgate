use poem_openapi::OpenApi;

pub mod known_hosts_detail;
pub mod known_hosts_list;
pub mod logs;
pub mod recordings_detail;
pub mod sessions_detail;
pub mod sessions_list;
pub mod ssh_keys;
pub mod targets_list;
pub mod tickets_detail;
pub mod tickets_list;
pub mod users_list;
mod pagination;

pub fn get() -> impl OpenApi {
    (
        sessions_list::Api,
        sessions_detail::Api,
        recordings_detail::Api,
        users_list::Api,
        targets_list::Api,
        tickets_list::Api,
        tickets_detail::Api,
        known_hosts_list::Api,
        known_hosts_detail::Api,
        ssh_keys::Api,
        logs::Api,
    )
}
