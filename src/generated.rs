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
    pub const CARD: &str = "{\"bytes\":14,\"schema\":{\"variable\":{\"number\":{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"number_pool\":[1,2,2,2,2,3,3]}},\"patterns\":[{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"template_pool\":[\"TEC_01\",\"TEC_02\"]}},{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"template_pool\":[\"TEC_01\",\"TEC_02\"]}},{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"template_pool\":[\"TEC_01\",\"TEC_02\"]}}]}}}";
}
pub mod location {
    pub const ADJECTIVE: &str = "{\"bytes\":1,\"schema\":{\"simple\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"TRL_001\",\"TRL_002\",\"TRL_003\",\"TRL_004\",\"TRL_005\",\"TRL_006\"]}}}}";
    pub const NAME: &str = "{\"bytes\":1,\"schema\":{\"simple\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"TRL_010\",\"TRL_011\",\"TRL_012\",\"TRL_013\",\"TRL_014\"]}}}}";
    pub const BELONGING: &str = "{\"bytes\":1,\"schema\":{\"simple\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"TRL_020\",\"TRL_021\",\"TRL_022\",\"TRL_023\",\"TRL_024\"]}}}}";
    pub const COORDINATE: &str = "{\"bytes\":2,\"schema\":{\"fixed\":[{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_range\":[0,30]}},{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_range\":[0,45]}}]}}";
    pub const AREA: &str = "{\"bytes\":4,\"schema\":{\"fixed\":[{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"number_range\":[1,4]}},{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"number_range\":[1,4]}}]}}";
    pub const COLOR: &str = "{\"bytes\":4,\"schema\":{\"fixed\":[{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_range\":[128,255]}},{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_range\":[0,255]}},{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_range\":[0,255]}},{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_range\":[0,255]}}]}}";
    pub const COMMODITY: &str = "{\"bytes\":14,\"schema\":{\"variable\":{\"number\":{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"number_pool\":[1,2,2,2,2,3,3]}},\"patterns\":[{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"template_pool\":[\"TEL_01\",\"TEL_02\"]}},{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"template_pool\":[\"TEL_01\",\"TEL_02\"]}},{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"template_pool\":[\"TEL_01\",\"TEL_02\"]}}]}}}";
}
pub mod date {
    pub const ERA: &str = "{\"bytes\":1,\"schema\":{\"simple\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"TRD_001\",\"TRD_002\",\"TRD_003\",\"TRD_004\",\"TRD_005\",\"TRD_006\",\"TRD_007\",\"TRD_008\",\"TRD_009\"]}}}}";
    pub const YEAR: &str = "{\"bytes\":1,\"schema\":{\"simple\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_range\":[1,9]}}}}";
    pub const TIME: &str = "{\"bytes\":1,\"schema\":{\"simple\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"TRD_010\",\"TRD_011\",\"TRD_012\",\"TRD_013\",\"TRD_014\",\"TRD_015\"]}}}}";
    pub const WEATHER: &str = "{\"bytes\":1,\"schema\":{\"simple\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"TRD_030\",\"TRD_031\",\"TRD_032\",\"TRD_033\",\"TRD_034\",\"TRD_035\",\"TRD_036\",\"TRD_037\",\"TRD_038\"]}}}}";
    pub const HOLIDAY: &str = "{\"bytes\":1,\"schema\":{\"simple\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"TRD_040\",\"TRD_041\",\"TRD_042\",\"TRD_043\",\"TRD_044\",\"TRD_045\",\"TRD_046\"]}}}}";
    pub const SEASON: &str = "{\"bytes\":1,\"schema\":{\"simple\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"TRD_020\",\"TRD_021\",\"TRD_022\",\"TRD_023\"]}}}}";
    pub const BACKGROUND: &str = "{\"bytes\":4,\"schema\":{\"fixed\":[{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_range\":[128,255]}},{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_range\":[0,255]}},{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_range\":[0,255]}},{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_range\":[0,255]}}]}}";
    pub const EFFECT: &str = "{\"bytes\":14,\"schema\":{\"variable\":{\"number\":{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"number_pool\":[1,2,2,2,2,3,3]}},\"patterns\":[{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"template_pool\":[\"TED_01\",\"TED_02\"]}},{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"template_pool\":[\"TED_01\",\"TED_02\"]}},{\"occupied\":2,\"selector\":\"double\",\"pool\":{\"template_pool\":[\"TED_01\",\"TED_02\"]}}]}}}";
}
pub mod story {
    pub const CHARACTER: &str = "{\"bytes\":6,\"schema\":{\"multiple_variables\":[{\"number\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_pool\":[0,0,1,1,1,1]}},\"patterns\":[{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"TRC_010\",\"TRC_011\",\"TRC_012\",\"TRC_013\",\"TRC_014\",\"TRC_015\",\"TRC_016\"]}}]},{\"number\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_pool\":[0,0,1,1,1,1]}},\"patterns\":[{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"TRC_001\",\"TRC_002\",\"TRC_003\",\"TRC_004\",\"TRC_005\",\"TRC_006\"]}}]},{\"number\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_pool\":[0,0,1,1,1,1]}},\"patterns\":[{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"TRC_020\",\"TRC_021\",\"TRC_022\",\"TRC_023\"]}}]}]}}";
    pub const LOCATION: &str = "{\"bytes\":6,\"schema\":{\"multiple_variables\":[{\"number\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_pool\":[0]}},\"patterns\":[]},{\"number\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_pool\":[0]}},\"patterns\":[]},{\"number\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_pool\":[0]}},\"patterns\":[]}]}}";
    pub const DATE: &str = "{\"bytes\":6,\"schema\":{\"multiple_variables\":[{\"number\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_pool\":[0]}},\"patterns\":[]},{\"number\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_pool\":[0]}},\"patterns\":[]},{\"number\":{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"number_pool\":[0]}},\"patterns\":[]}]}}";
    pub const STORY: &str = "{\"bytes\":4,\"schema\":{\"fixed\":[{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"PAB01\"]}},{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"PAT01\",\"PAT02\",\"PAT03\",\"PAT04\",\"PAT05\",\"PAT06\",\"PAT07\",\"PAT08\",\"PAT09\",\"PAT10\",\"PAT11\",\"PAT12\",\"PAT13\",\"PAT14\",\"PAT15\",\"PAT16\",\"PAT17\"]}},{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"PAC01\",\"PAC02\",\"PAC03\",\"PAC04\",\"PAC05\",\"PAC06\",\"PAC07\",\"PAC08\",\"PAC09\",\"PAC10\",\"PAC11\",\"PAC12\",\"PAC13\",\"PAC14\",\"PAC15\",\"PAC16\"]}},{\"occupied\":1,\"selector\":\"single\",\"pool\":{\"trait_pool\":[\"PAE01\",\"PAE02\",\"PAE03\",\"PAE04\",\"PAE05\",\"PAE06\",\"PAE07\",\"PAE08\",\"PAE09\",\"PAE10\",\"PAE11\",\"PAE12\"]}}]}}";
}
pub mod language {
    pub mod cn {
        pub const TRAIT_POOL: &str = "";
        pub const TEMPLATE_POOL: &str = "";
        pub const PARAGRAPH_POOL: &str = "";
    }
}
