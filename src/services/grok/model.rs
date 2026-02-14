use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct ValidationException(pub String);

impl std::fmt::Display for ValidationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for ValidationException {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Tier {
    Basic,
    Super,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Cost {
    Low,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub model_id: String,
    pub grok_model: String,
    pub model_mode: String,

    #[serde(default = "default_tier")]
    pub tier: Tier,

    #[serde(default = "default_cost")]
    pub cost: Cost,

    pub display_name: String,

    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub is_video: bool,

    #[serde(default)]
    pub is_image: bool,
}

fn default_tier() -> Tier {
    Tier::Basic
}
fn default_cost() -> Cost {
    Cost::Low
}

impl ModelInfo {
    pub fn new(model_id: &str, grok_model: &str, model_mode: &str, display_name: &str) -> Self {
        Self {
            model_id: model_id.to_string(),
            grok_model: grok_model.to_string(),
            model_mode: model_mode.to_string(),
            tier: Tier::Basic,
            cost: Cost::Low,
            display_name: display_name.to_string(),
            description: String::new(),
            is_video: false,
            is_image: false,
        }
    }

    pub fn with_tier(mut self, tier: Tier) -> Self {
        self.tier = tier;
        self
    }

    pub fn with_cost(mut self, cost: Cost) -> Self {
        self.cost = cost;
        self
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn as_image(mut self) -> Self {
        self.is_image = true;
        self
    }

    pub fn as_video(mut self) -> Self {
        self.is_video = true;
        self
    }
}

/// 复刻 Python: ModelService.MODELS
static MODELS: Lazy<Vec<ModelInfo>> = Lazy::new(|| {
    vec![
        ModelInfo::new("grok-3", "grok-3", "MODEL_MODE_GROK_3", "GROK-3")
            .with_cost(Cost::Low),
        ModelInfo::new(
            "grok-3-mini",
            "grok-3",
            "MODEL_MODE_GROK_3_MINI_THINKING",
            "GROK-3-MINI",
        )
        .with_cost(Cost::Low),
        ModelInfo::new(
            "grok-3-thinking",
            "grok-3",
            "MODEL_MODE_GROK_3_THINKING",
            "GROK-3-THINKING",
        )
        .with_cost(Cost::Low),
        ModelInfo::new("grok-4", "grok-4", "MODEL_MODE_GROK_4", "GROK-4")
            .with_cost(Cost::Low),
        ModelInfo::new(
            "grok-4-mini",
            "grok-4-mini",
            "MODEL_MODE_GROK_4_MINI_THINKING",
            "GROK-4-MINI",
        )
        .with_cost(Cost::Low),
        ModelInfo::new(
            "grok-4-thinking",
            "grok-4",
            "MODEL_MODE_GROK_4_THINKING",
            "GROK-4-THINKING",
        )
        .with_cost(Cost::Low),
        ModelInfo::new(
            "grok-4-heavy",
            "grok-4",
            "MODEL_MODE_HEAVY",
            "GROK-4-HEAVY",
        )
        .with_cost(Cost::High)
        .with_tier(Tier::Super),
        ModelInfo::new(
            "grok-4.1-mini",
            "grok-4-1-thinking-1129",
            "MODEL_MODE_GROK_4_1_MINI_THINKING",
            "GROK-4.1-MINI",
        )
        .with_cost(Cost::Low),
        ModelInfo::new(
            "grok-4.1-fast",
            "grok-4-1-thinking-1129",
            "MODEL_MODE_FAST",
            "GROK-4.1-FAST",
        )
        .with_cost(Cost::Low),
        ModelInfo::new(
            "grok-4.1-expert",
            "grok-4-1-thinking-1129",
            "MODEL_MODE_EXPERT",
            "GROK-4.1-EXPERT",
        )
        .with_cost(Cost::High),
        ModelInfo::new(
            "grok-4.1-thinking",
            "grok-4-1-thinking-1129",
            "MODEL_MODE_GROK_4_1_THINKING",
            "GROK-4.1-THINKING",
        )
        .with_cost(Cost::High),
        ModelInfo::new(
            "grok-imagine-1.0",
            "grok-3",
            "MODEL_MODE_FAST",
            "Grok Image",
        )
        .with_cost(Cost::High)
        .with_description("Image generation model")
        .as_image(),
        ModelInfo::new(
            "grok-imagine-1.0-edit",
            "imagine-image-edit",
            "MODEL_MODE_FAST",
            "Grok Image Edit",
        )
        .with_cost(Cost::High)
        .with_description("Image edit model")
        .as_image(),
        ModelInfo::new(
            "grok-imagine-1.0-video",
            "grok-3",
            "MODEL_MODE_FAST",
            "Grok Video",
        )
        .with_cost(Cost::High)
        .with_description("Video generation model")
        .as_video(),
    ]
});

/// 复刻 Python: ModelService._map = {m.model_id: m for m in MODELS}
static MODEL_MAP: Lazy<HashMap<&'static str, &'static ModelInfo>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for m in MODELS.iter() {
        // 安全：MODELS 是静态 Lazy，m 的引用生命周期可视为 'static
        let key: &'static str = Box::leak(m.model_id.clone().into_boxed_str());
        let val: &'static ModelInfo = unsafe { &*(m as *const ModelInfo) };
        map.insert(key, val);
    }
    map
});

pub struct ModelService;

impl ModelService {
    /// 获取模型信息（对齐 Python: Optional[ModelInfo]）
    /// Rust 这里返回引用，避免 clone；如果你需要拥有所有权，用 get_cloned()
    pub fn get(model_id: &str) -> Option<&'static ModelInfo> {
        MODEL_MAP.get(model_id).copied()
    }

    /// 若你希望返回拥有所有权的结构（更接近 Python 返回对象语义）
    pub fn get_cloned(model_id: &str) -> Option<ModelInfo> {
        Self::get(model_id).cloned()
    }

    /// 获取所有模型（对齐 Python list()）
    pub fn list() -> Vec<ModelInfo> {
        MODELS.clone()
    }

    /// 模型是否有效（对齐 Python valid()）
    pub fn valid(model_id: &str) -> bool {
        MODEL_MAP.contains_key(model_id)
    }

    /// 转换为 Grok 参数（对齐 Python to_grok()）
    pub fn to_grok(model_id: &str) -> Result<(String, String), ValidationException> {
        let m = Self::get(model_id)
            .ok_or_else(|| ValidationException(format!("Invalid model ID: {}", model_id)))?;
        Ok((m.grok_model.clone(), m.model_mode.clone()))
    }

    /// 根据模型选择 Token 池（对齐 Python pool_for_model()）
    pub fn pool_for_model(model_id: &str) -> &'static str {
        match Self::get(model_id) {
            Some(m) if m.tier == Tier::Super => "ssoSuper",
            _ => "ssoBasic",
        }
    }

    /// 按优先级返回可用 Token 池列表（对齐 Python pool_candidates_for_model()）
    pub fn pool_candidates_for_model(model_id: &str) -> Vec<&'static str> {
        match Self::get(model_id) {
            Some(m) if m.tier == Tier::Super => vec!["ssoSuper"],
            _ => vec!["ssoBasic", "ssoSuper"],
        }
    }
}
