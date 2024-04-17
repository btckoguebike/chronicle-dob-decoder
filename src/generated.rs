#![allow(dead_code)]
pub mod character {
    pub const ADJECTIVE: &str = "{\"bytes\":1,\"schema\":{\"simple\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"TRC_010\",\"TRC_011\",\"TRC_012\",\"TRC_013\",\"TRC_014\",\"TRC_015\",\"TRC_016\"]}}}}";
    pub const NAME: &str = "{\"bytes\":1,\"schema\":{\"simple\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"TRC_001\",\"TRC_002\",\"TRC_003\",\"TRC_004\",\"TRC_005\",\"TRC_006\"]}}}}";
    pub const PROFESSION: &str = "{\"bytes\":1,\"schema\":{\"simple\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"TRC_020\",\"TRC_021\",\"TRC_022\",\"TRC_023\"]}}}}";
    pub const HP: &str = "{\"bytes\":1,\"schema\":{\"simple\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_range\":[50,100]}}}}";
    pub const POWER: &str = "{\"bytes\":2,\"schema\":{\"simple\":{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"number_pool\":[3,3,3,3,3,3,3,3,3,4,4,4,4,4,5,5,5]}}}}";
    pub const ATTACK: &str = "{\"bytes\":1,\"schema\":{\"simple\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_range\":[0,30]}}}}";
    pub const DEFENSE: &str = "{\"bytes\":1,\"schema\":{\"simple\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_range\":[0,30]}}}}";
    pub const GOLD: &str = "{\"bytes\":1,\"schema\":{\"simple\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_range\":[0,200]}}}}";
    pub const CARD: &str = "{\"bytes\":14,\"schema\":{\"variable\":{\"number\":{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"number_pool\":[2,2,2,2,3,3]}},\"patterns\":[{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"template_pool\":[\"TEC_001\",\"TEC_002\"]}},{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"template_pool\":[\"TEC_001\",\"TEC_002\"]}},{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"template_pool\":[\"TEC_001\",\"TEC_002\"]}}]}}}";
}
pub mod language {
    pub mod cn {
        pub const TRAIT_POOL: &str = "{\"__CHARACTER_SEGMENTATION_NAME__\":\"__角色名字__\",\"TRC_001\":\"赛尔文\",\"TRC_002\":\"埃尔德林\",\"TRC_003\":\"莫尔丁\",\"TRC_004\":\"泰勒斯\",\"TRC_005\":\"达尼洛斯\",\"TRC_006\":\"卡尔文\",\"__CHARACTER_SEGMENTATION_ADJECTIVE__\":\"__角色描述__\",\"TRC_010\":\"烈焰的\",\"TRC_011\":\"辉夜的\",\"TRC_012\":\"不朽的\",\"TRC_013\":\"传奇的\",\"TRC_014\":\"仙境的\",\"TRC_015\":\"神圣的\",\"TRC_016\":\"无尽的\",\"__CHARACTER_SEGMENTATION_PROFESSION__\":\"__角色职业__\",\"TRC_020\":\"战士\",\"TRC_021\":\"刺客\",\"TRC_022\":\"法师\",\"TRC_023\":\"牧师\"}";
        pub const TEMPLATE_POOL: &str = "{\"TEC_001\":[{\"range\":[0,3]},{\"template\":\"攻击[x]\"},{\"range\":[6,10]},{\"template\":\"造成x点伤害\"}],\"TEC_002\":[{\"range\":[0,3]},{\"template\":\"闪电击[x]\"},{\"range\":[6,12]},{\"template\":\"对所有敌人造成x点伤害\"}]}";
    }
}
