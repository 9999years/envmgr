use super::*;

impl From<EnvConfig> for super::super::EnvConfig {
    fn from(cfg: EnvConfig) -> Self {
        Self {
            env: cfg.env.into(),
            tests: cfg.tests,
        }
    }
}

impl Into<HashMap<String, super::super::VarConfig>> for VarEntries {
    fn into(self) -> HashMap<String, super::super::VarConfig> {
        self.0
            .into_iter()
            .map(|(var, cfg)| (var, cfg.conv::<super::super::VarConfig>()))
            .collect()
    }
}
impl From<VarConfigWrapper> for super::super::VarConfig {
    fn from(config: VarConfigWrapper) -> Self {
        match config {
            VarConfigWrapper::Entries(paths) => Self {
                sep: default_var_sep(),
                paths: paths
                    .into_iter()
                    .map(|entry| entry.conv::<super::super::DirEntry>())
                    .collect(),
            },
            VarConfigWrapper::Full(cfg) => cfg.into(),
            VarConfigWrapper::SingleString(path) => Self {
                sep: default_var_sep(),
                paths: vec![super::super::DirEntry {
                    path: path.into(),
                    when: Default::default(),
                }],
            },
        }
    }
}

impl From<VarConfig> for super::super::VarConfig {
    fn from(cfg: VarConfig) -> Self {
        Self {
            sep: cfg.sep,
            paths: cfg
                .paths
                .into_iter()
                .map(|entry| entry.conv::<super::super::DirEntry>())
                .collect(),
        }
    }
}

impl From<DirEntry> for super::super::DirEntry {
    fn from(entry: DirEntry) -> Self {
        match entry {
            DirEntry::Plain(path) => Self {
                path: path.into(),
                when: Default::default(),
            },
            DirEntry::Conditional(el) => Self {
                path: el.value.into(),
                when: el.when.into(),
            },
        }
    }
}
