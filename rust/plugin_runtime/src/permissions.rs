use std::collections::HashSet;
use std::hash::Hash;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use anyhow::anyhow;
use deno_runtime::deno_fs::{FileSystemRc, RealFs};
use deno_runtime::deno_permissions::{AllowRunDescriptor, EnvDescriptor, EnvQueryDescriptor, NetDescriptor, Permissions, PermissionsContainer, QueryDescriptor, ReadDescriptor, RunQueryDescriptor, SysDescriptor, SysDescriptorParseError, UnaryPermission, WriteDescriptor};
use deno_runtime::permissions::RuntimePermissionDescriptorParser;
use once_cell::sync::Lazy;
use regex::Regex;
use typed_path::Utf8TypedPath;
use common::dirs::Dirs;
use crate::{JsPluginPermissions, JsPluginPermissionsExec};

pub static PERMISSIONS_VARIABLE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{(?<namespace>.+?):(?<name>.+?)}").expect("invalid regex"));

pub fn permissions_to_deno(
    fs: FileSystemRc,
    permissions: &JsPluginPermissions,
    home_dir: &Path,
    plugin_data_dir: &Path,
    plugin_cache_dir: &Path,
) -> anyhow::Result<PermissionsContainer> {
    Ok(PermissionsContainer::new(
        Arc::new(RuntimePermissionDescriptorParser::new(fs)),
        Permissions {
            read: path_permission(&permissions.filesystem.read, ReadDescriptor, home_dir, plugin_data_dir, plugin_cache_dir)?,
            write: path_permission(&permissions.filesystem.write, WriteDescriptor, home_dir, plugin_data_dir, plugin_cache_dir)?,
            net: net_permission(&permissions.network),
            env: env_permission(&permissions.environment),
            sys: sys_permission(&permissions.system)?,
            run: run_permission(&permissions.exec, home_dir, plugin_data_dir, plugin_cache_dir)?,
            ffi: Permissions::new_unary(None, None, false),
            import: UnaryPermission::default(),
            all: Permissions::new_all(false),
        }
    ))
}

fn path_permission<P: Eq + Hash, T: QueryDescriptor<AllowDesc = P, DenyDesc = P> + Hash>(
    paths: &[String],
    to_permission: fn(PathBuf) -> P,
    home_dir: &Path,
    plugin_data_dir: &Path,
    plugin_cache_dir: &Path,
) -> anyhow::Result<UnaryPermission<T>> {
    let allow_list = paths
        .into_iter()
        .map(|path| {
            augment_path(path, home_dir, plugin_data_dir, plugin_cache_dir)
                .map(|path| path.map(|path| to_permission(path)))
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .filter_map(std::convert::identity)
        .collect::<HashSet<_>>();

    let allow_list = if allow_list.is_empty() {
        None
    } else {
        Some(allow_list)
    };

    Ok(Permissions::new_unary(allow_list, None, false))
}

fn net_permission(domain_and_ports: &[String]) -> UnaryPermission<NetDescriptor> {
    let allow_list = if domain_and_ports.is_empty() {
        None
    } else {
        let allow_list = domain_and_ports.into_iter()
            .map(|domain_and_port| {
                NetDescriptor::parse(&domain_and_port)
                    .expect("should be validated when loading")
            })
            .collect();

        Some(allow_list)
    };

    Permissions::new_unary(allow_list, None, false)
}

fn env_permission(envs: &[String]) -> UnaryPermission<EnvQueryDescriptor> {
    let allow_list = if envs.is_empty() {
        None
    } else {
        let allow_list = envs.into_iter()
            .map(|env| EnvDescriptor::new(env))
            .collect();

        Some(allow_list)
    };

    Permissions::new_unary(allow_list, None, false)
}

fn sys_permission(system: &[String]) -> anyhow::Result<UnaryPermission<SysDescriptor>> {
    let allow_list = if system.is_empty() {
        None
    } else {
        let allow_list = system.into_iter()
            .map(|system| SysDescriptor::parse(system.to_owned()))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .collect();

        Some(allow_list)
    };

    Ok(Permissions::new_unary(allow_list, None, false))
}

fn run_permission(
    permissions: &JsPluginPermissionsExec,
    home_dir: &Path,
    plugin_data_dir: &Path,
    plugin_cache_dir: &Path,
) -> anyhow::Result<UnaryPermission<RunQueryDescriptor>> {
    let granted_executable = permissions.executable
        .iter()
        .map(|path| {
            augment_path(path, home_dir, plugin_data_dir, plugin_cache_dir)
                .map(|path| path.map(|path| AllowRunDescriptor(path)))
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .filter_map(std::convert::identity)
        .collect::<Vec<_>>();

    let granted_command = permissions.command
        .iter()
        .map(|cmd| AllowRunDescriptor(PathBuf::from(cmd)))
        .collect::<Vec<_>>();

    let mut granted = HashSet::new();
    granted.extend(granted_executable);
    granted.extend(granted_command);

    let allow_list = if granted.is_empty() {
        None
    } else {
        Some(granted)
    };

    Ok(Permissions::new_unary(allow_list, None, false))
}

fn augment_path(path: &String, home_dir: &Path, plugin_data_dir: &Path, plugin_cache_dir: &Path) -> anyhow::Result<Option<PathBuf>> {
    if let Some(matches) = PERMISSIONS_VARIABLE_PATTERN.captures(path) {
        let namespace = &matches["namespace"];
        let name = &matches["name"];

        let replacement = match (namespace, name) {
            ("macos", "user-home") => {
                if cfg!(target_os = "macos") {
                    Some(home_dir)
                } else {
                    None
                }
            },
            ("linux", "user-home") => {
                if cfg!(target_os = "linux") {
                    Some(home_dir)
                } else {
                    None
                }
            },
            ("windows", "user-home") => {
                if cfg!(windows) {
                    Some(home_dir)
                } else {
                    None
                }
            },
            ("common", "plugin-data") => Some(plugin_data_dir),
            ("common", "plugin-cache") => Some(plugin_cache_dir),
            (_, _) => {
                Err(anyhow!("Trying to load plugin with unknown variable in path in manifest permissions: {}", path))?
            }
        };

        match replacement {
            None => Ok(None),
            Some(replacement) => {
                let replacement = replacement.to_str()
                    .expect("non-utf8 file paths are not supported");

                Ok(Some(PathBuf::from(PERMISSIONS_VARIABLE_PATTERN.replace(path, replacement).to_string())))
            }
        }
    } else {
        match Utf8TypedPath::derive(&path) {
            Utf8TypedPath::Unix(_) => {
                if cfg!(unix) {
                    Ok(Some(PathBuf::from(path)))
                } else {
                    Ok(None)
                }
            }
            Utf8TypedPath::Windows(_) => {
                if cfg!(windows) {
                    Ok(Some(PathBuf::from(path)))
                } else {
                    Ok(None)
                }
            }
        }
    }
}