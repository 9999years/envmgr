use wyz::conv::Conv;

use super::*;

impl From<EnvConfig> for super::super::EnvConfig {
    fn from(cfg: EnvConfig) -> Self {
        Self {
            env: cfg.env.into(),
            tests: cfg.tests,
        }
    }
}

impl Into<super::super::VarMap> for EnvMap {
    fn into(self) -> super::super::VarMap {
        match self {
            EnvMap::Block(ConditionEl { value, when }) => {
                let flattened_block: super::super::VarMap = value.into();
                for &mut v in flattened_block.0.values_mut() {}
                flattened_block
            }
            EnvMap::Map(map) => map.into(),
        }
    }
}

impl From<VarMap> for super::super::VarMap {
    fn from(map: VarMap) -> Self {
        Self(
            map.0
                .into_iter()
                .map(|(k, v)| (k, v.conv::<ConditionEl<super::super::VarConfig>>()))
                .collect(),
        )
    }
}

impl Into<super::super::VarMap> for Block {
    fn into(self) -> super::super::VarMap {
        (*self.block).into()
    }
}

impl Into<ConditionEl<super::super::VarConfig>> for VarConfig {
    fn into(self) -> ConditionEl<super::super::VarConfig> {
        match self {
            VarConfig::SingleString(s) => {
                vec![s.conv::<super::super::ShellPath>().conv::<ConditionEl<_>>()]
                    .conv::<super::super::VarConfig>()
                    .conv()
            }
            VarConfig::Entries(entries) => {}
            VarConfig::Full(_) => {}
        }
    }
}

impl From<DirEntry> for ConditionEl<super::super::ShellPath> {
    fn from(entry: DirEntry) -> Self {
        match entry {
            DirEntry::Plain(s) => s.conv::<super::super::ShellPath>().conv(),
            DirEntry::Conditional(cond) => ConditionEl {
                value: cond
                    .value
                    .conv::<String>()
                    .conv::<super::super::ShellPath>(),
                when: cond.when,
            },
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
