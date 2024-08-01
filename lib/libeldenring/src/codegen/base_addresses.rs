// **********************************
// *** AUTOGENERATED, DO NOT EDIT ***
// **********************************
#[derive(Debug)]
pub struct BaseAddresses {
    pub bullet_man: usize,
    pub chr_dbg_flags: usize,
    pub csfd4_virtual_memory_flag: usize,
    pub cs_flipper: usize,
    pub cs_lua_event_manager: usize,
    pub cs_menu_man: usize,
    pub cs_menu_man_imp: usize,
    pub cs_net_man: usize,
    pub cs_regulation_manager: usize,
    pub cs_session_manager: usize,
    pub damage_ctrl: usize,
    pub field_area: usize,
    pub game_data_man: usize,
    pub game_man: usize,
    pub global_pos: usize,
    pub group_mask: usize,
    pub hit_ins: usize,
    pub hit_ins_hitbox_offset: usize,
    pub map_item_man: usize,
    pub menu_man_ins: usize,
    pub msg_repository: usize,
    pub solo_param_repository: usize,
    pub world_chr_man: usize,
    pub world_chr_man_dbg: usize,
    pub world_chr_man_imp: usize,
    pub func_item_spawn: usize,
    pub func_item_inject: usize,
    pub func_remove_intro_screens: usize,
    pub func_dbg_action_force: usize,
    pub lua_warp: usize,
    pub current_target: usize,
    pub base_fps: usize,
    pub base_anim: usize,
}

impl BaseAddresses {
    pub fn with_module_base_addr(self, base: usize) -> BaseAddresses {
        BaseAddresses {
            bullet_man: self.bullet_man + base,
            chr_dbg_flags: self.chr_dbg_flags + base,
            csfd4_virtual_memory_flag: self.csfd4_virtual_memory_flag + base,
            cs_flipper: self.cs_flipper + base,
            cs_lua_event_manager: self.cs_lua_event_manager + base,
            cs_menu_man: self.cs_menu_man + base,
            cs_menu_man_imp: self.cs_menu_man_imp + base,
            cs_net_man: self.cs_net_man + base,
            cs_regulation_manager: self.cs_regulation_manager + base,
            cs_session_manager: self.cs_session_manager + base,
            damage_ctrl: self.damage_ctrl + base,
            field_area: self.field_area + base,
            game_data_man: self.game_data_man + base,
            game_man: self.game_man + base,
            global_pos: self.global_pos + base,
            group_mask: self.group_mask + base,
            hit_ins: self.hit_ins + base,
            hit_ins_hitbox_offset: self.hit_ins_hitbox_offset + base,
            map_item_man: self.map_item_man + base,
            menu_man_ins: self.menu_man_ins + base,
            msg_repository: self.msg_repository + base,
            solo_param_repository: self.solo_param_repository + base,
            world_chr_man: self.world_chr_man + base,
            world_chr_man_dbg: self.world_chr_man_dbg + base,
            world_chr_man_imp: self.world_chr_man_imp + base,
            func_item_spawn: self.func_item_spawn + base,
            func_item_inject: self.func_item_inject + base,
            func_remove_intro_screens: self.func_remove_intro_screens + base,
            func_dbg_action_force: self.func_dbg_action_force + base,
            lua_warp: self.lua_warp + base,
            current_target: self.current_target + base,
            base_fps: self.base_fps + base,
            base_anim: self.base_anim + base,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Version {
    V1_02_0,
    V1_02_1,
    V1_02_2,
    V1_02_3,
    V1_03_0,
    V1_03_1,
    V1_03_2,
    V1_04_0,
    V1_04_1,
    V1_05_0,
    V1_06_0,
    V1_07_0,
    V1_08_0,
    V1_08_1,
    V1_09_0,
    V1_09_1,
    V2_00_0,
    V2_00_1,
    V2_02_0,
    V2_02_3,
    V2_03_0,
}

impl TryFrom<(u32, u32, u32)> for Version {
    type Error = ();

    fn try_from(v: (u32, u32, u32)) -> Result<Self, ()> {
        match v {
            (1, 2, 0) => Ok(Version::V1_02_0),
            (1, 2, 1) => Ok(Version::V1_02_1),
            (1, 2, 2) => Ok(Version::V1_02_2),
            (1, 2, 3) => Ok(Version::V1_02_3),
            (1, 3, 0) => Ok(Version::V1_03_0),
            (1, 3, 1) => Ok(Version::V1_03_1),
            (1, 3, 2) => Ok(Version::V1_03_2),
            (1, 4, 0) => Ok(Version::V1_04_0),
            (1, 4, 1) => Ok(Version::V1_04_1),
            (1, 5, 0) => Ok(Version::V1_05_0),
            (1, 6, 0) => Ok(Version::V1_06_0),
            (1, 7, 0) => Ok(Version::V1_07_0),
            (1, 8, 0) => Ok(Version::V1_08_0),
            (1, 8, 1) => Ok(Version::V1_08_1),
            (1, 9, 0) => Ok(Version::V1_09_0),
            (1, 9, 1) => Ok(Version::V1_09_1),
            (2, 0, 0) => Ok(Version::V2_00_0),
            (2, 0, 1) => Ok(Version::V2_00_1),
            (2, 2, 0) => Ok(Version::V2_02_0),
            (2, 2, 3) => Ok(Version::V2_02_3),
            (2, 3, 0) => Ok(Version::V2_03_0),
            (maj, min, patch) => {
                log::error!("Unrecognized version {maj}.{min:02}.{patch}");
                Err(())
            },
        }
    }
}

impl From<Version> for (u32, u32, u32) {
    fn from(v: Version) -> Self {
        match v {
            Version::V1_02_0 => (1, 2, 0),
            Version::V1_02_1 => (1, 2, 1),
            Version::V1_02_2 => (1, 2, 2),
            Version::V1_02_3 => (1, 2, 3),
            Version::V1_03_0 => (1, 3, 0),
            Version::V1_03_1 => (1, 3, 1),
            Version::V1_03_2 => (1, 3, 2),
            Version::V1_04_0 => (1, 4, 0),
            Version::V1_04_1 => (1, 4, 1),
            Version::V1_05_0 => (1, 5, 0),
            Version::V1_06_0 => (1, 6, 0),
            Version::V1_07_0 => (1, 7, 0),
            Version::V1_08_0 => (1, 8, 0),
            Version::V1_08_1 => (1, 8, 1),
            Version::V1_09_0 => (1, 9, 0),
            Version::V1_09_1 => (1, 9, 1),
            Version::V2_00_0 => (2, 0, 0),
            Version::V2_00_1 => (2, 0, 1),
            Version::V2_02_0 => (2, 2, 0),
            Version::V2_02_3 => (2, 2, 3),
            Version::V2_03_0 => (2, 3, 0),
        }
    }
}

impl From<Version> for BaseAddresses {
    fn from(v: Version) -> Self {
        match v {
            Version::V1_02_0 => BASE_ADDRESSES_1_02_0,
            Version::V1_02_1 => BASE_ADDRESSES_1_02_1,
            Version::V1_02_2 => BASE_ADDRESSES_1_02_2,
            Version::V1_02_3 => BASE_ADDRESSES_1_02_3,
            Version::V1_03_0 => BASE_ADDRESSES_1_03_0,
            Version::V1_03_1 => BASE_ADDRESSES_1_03_1,
            Version::V1_03_2 => BASE_ADDRESSES_1_03_2,
            Version::V1_04_0 => BASE_ADDRESSES_1_04_0,
            Version::V1_04_1 => BASE_ADDRESSES_1_04_1,
            Version::V1_05_0 => BASE_ADDRESSES_1_05_0,
            Version::V1_06_0 => BASE_ADDRESSES_1_06_0,
            Version::V1_07_0 => BASE_ADDRESSES_1_07_0,
            Version::V1_08_0 => BASE_ADDRESSES_1_08_0,
            Version::V1_08_1 => BASE_ADDRESSES_1_08_1,
            Version::V1_09_0 => BASE_ADDRESSES_1_09_0,
            Version::V1_09_1 => BASE_ADDRESSES_1_09_1,
            Version::V2_00_0 => BASE_ADDRESSES_2_00_0,
            Version::V2_00_1 => BASE_ADDRESSES_2_00_1,
            Version::V2_02_0 => BASE_ADDRESSES_2_02_0,
            Version::V2_02_3 => BASE_ADDRESSES_2_02_3,
            Version::V2_03_0 => BASE_ADDRESSES_2_03_0,
        }
    }
}

pub const BASE_ADDRESSES_1_02_0: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c4c9c8,
    chr_dbg_flags: 0x3c5047f,
    csfd4_virtual_memory_flag: 0x3c526e8,
    cs_flipper: 0x4473138,
    cs_lua_event_manager: 0x3c520f8,
    cs_menu_man: 0x8ba62b94,
    cs_menu_man_imp: 0x3c55b30,
    cs_net_man: 0x3c45160,
    cs_regulation_manager: 0x3c70658,
    cs_session_manager: 0x3c63d30,
    damage_ctrl: 0x3c50658,
    field_area: 0x3c53470,
    game_data_man: 0x3c481b8,
    game_man: 0x3c53b88,
    global_pos: 0x3c50268,
    group_mask: 0x3a367df,
    hit_ins: 0x3c54320,
    hit_ins_hitbox_offset: 0x3c5432c,
    map_item_man: 0x3c51cf0,
    menu_man_ins: 0x3c55b30,
    msg_repository: 0x3c66d48,
    solo_param_repository: 0x3c6b578,
    world_chr_man: 0x3c50268,
    world_chr_man_dbg: 0x3c50478,
    world_chr_man_imp: 0x3c50268,
    func_item_spawn: 0x548c20,
    func_item_inject: 0x54e570,
    func_remove_intro_screens: 0xaaad4a,
    func_dbg_action_force: 0x527214a,
    lua_warp: 0x5855ae,
    current_target: 0x6f0a22,
    base_fps: 0x3c5f868,
    base_anim: 0x3a1ad30,
};

pub const BASE_ADDRESSES_1_02_1: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c4c9e8,
    chr_dbg_flags: 0x3c5049f,
    csfd4_virtual_memory_flag: 0x3c52708,
    cs_flipper: 0x4472d58,
    cs_lua_event_manager: 0x3c52118,
    cs_menu_man: 0x8ba62c04,
    cs_menu_man_imp: 0x3c55b50,
    cs_net_man: 0x3c45180,
    cs_regulation_manager: 0x3c70278,
    cs_session_manager: 0x3c63d50,
    damage_ctrl: 0x3c50678,
    field_area: 0x3c53490,
    game_data_man: 0x3c481d8,
    game_man: 0x3c53ba8,
    global_pos: 0x3c50288,
    group_mask: 0x3a367ff,
    hit_ins: 0x3c54340,
    hit_ins_hitbox_offset: 0x3c5434c,
    map_item_man: 0x3c51d10,
    menu_man_ins: 0x3c55b50,
    msg_repository: 0x3c66d68,
    solo_param_repository: 0x3c6b598,
    world_chr_man: 0x3c50288,
    world_chr_man_dbg: 0x3c50498,
    world_chr_man_imp: 0x3c50288,
    func_item_spawn: 0x548c90,
    func_item_inject: 0x54e5e0,
    func_remove_intro_screens: 0xaaadca,
    func_dbg_action_force: 0x53f757d,
    lua_warp: 0x58561e,
    current_target: 0x6f0a92,
    base_fps: 0x3c5f888,
    base_anim: 0x3a1ad50,
};

pub const BASE_ADDRESSES_1_02_2: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c4ca08,
    chr_dbg_flags: 0x3c504bf,
    csfd4_virtual_memory_flag: 0x3c52728,
    cs_flipper: 0x4472d78,
    cs_lua_event_manager: 0x3c52138,
    cs_menu_man: 0x8ba62c04,
    cs_menu_man_imp: 0x3c55b70,
    cs_net_man: 0x3c451a0,
    cs_regulation_manager: 0x3c70298,
    cs_session_manager: 0x3c63d70,
    damage_ctrl: 0x3c50698,
    field_area: 0x3c534b0,
    game_data_man: 0x3c481f8,
    game_man: 0x3c53bc8,
    global_pos: 0x3c502a8,
    group_mask: 0x3a367ff,
    hit_ins: 0x3c54360,
    hit_ins_hitbox_offset: 0x3c5436c,
    map_item_man: 0x3c51d30,
    menu_man_ins: 0x3c55b70,
    msg_repository: 0x3c66d88,
    solo_param_repository: 0x3c6b5b8,
    world_chr_man: 0x3c502a8,
    world_chr_man_dbg: 0x3c504b8,
    world_chr_man_imp: 0x3c502a8,
    func_item_spawn: 0x548c90,
    func_item_inject: 0x54e5e0,
    func_remove_intro_screens: 0xaaae3a,
    func_dbg_action_force: 0x4f45cb0,
    lua_warp: 0x58561e,
    current_target: 0x6f0a92,
    base_fps: 0x3c5f8a8,
    base_anim: 0x3a1ad50,
};

pub const BASE_ADDRESSES_1_02_3: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c4fa28,
    chr_dbg_flags: 0x3c534dd,
    csfd4_virtual_memory_flag: 0x3c55748,
    cs_flipper: 0x4475d98,
    cs_lua_event_manager: 0x3c55158,
    cs_menu_man: 0x8ba62d24,
    cs_menu_man_imp: 0x3c58b90,
    cs_net_man: 0x3c481c0,
    cs_regulation_manager: 0x3c732b8,
    cs_session_manager: 0x3c66d80,
    damage_ctrl: 0x3c536b8,
    field_area: 0x3c564d0,
    game_data_man: 0x3c4b218,
    game_man: 0x3c56be8,
    global_pos: 0x3c532c8,
    group_mask: 0x3a39807,
    hit_ins: 0x3c57380,
    hit_ins_hitbox_offset: 0x3c5738c,
    map_item_man: 0x3c54d50,
    menu_man_ins: 0x3c58b90,
    msg_repository: 0x3c69d98,
    solo_param_repository: 0x3c6e5c8,
    world_chr_man: 0x3c532c8,
    world_chr_man_dbg: 0x3c53520,
    world_chr_man_imp: 0x3c532c8,
    func_item_spawn: 0x548db0,
    func_item_inject: 0x54e700,
    func_remove_intro_screens: 0xaaaf1a,
    func_dbg_action_force: 0x4edd1ef,
    lua_warp: 0x58573e,
    current_target: 0x6f0bb2,
    base_fps: 0x3c628c8,
    base_anim: 0x3a1dd50,
};

pub const BASE_ADDRESSES_1_03_0: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c61588,
    chr_dbg_flags: 0x3c6504f,
    csfd4_virtual_memory_flag: 0x3c672a8,
    cs_flipper: 0x4487918,
    cs_lua_event_manager: 0x3c66cb8,
    cs_menu_man: 0x8ba63d24,
    cs_menu_man_imp: 0x3c6a700,
    cs_net_man: 0x3c59d20,
    cs_regulation_manager: 0x3c84e38,
    cs_session_manager: 0x3c78900,
    damage_ctrl: 0x3c65228,
    field_area: 0x3c68040,
    game_data_man: 0x3c5cd78,
    game_man: 0x3c68758,
    global_pos: 0x3c64e38,
    group_mask: 0x3a4a807,
    hit_ins: 0x3c68ef0,
    hit_ins_hitbox_offset: 0x3c68efc,
    map_item_man: 0x3c668c0,
    menu_man_ins: 0x3c6a700,
    msg_repository: 0x3c7b928,
    solo_param_repository: 0x3c80158,
    world_chr_man: 0x3c64e38,
    world_chr_man_dbg: 0x3c65048,
    world_chr_man_imp: 0x3c64e38,
    func_item_spawn: 0x549c70,
    func_item_inject: 0x54f640,
    func_remove_intro_screens: 0xab021d,
    func_dbg_action_force: 0x1e42a5c,
    lua_warp: 0x58671e,
    current_target: 0x6f1ee2,
    base_fps: 0x3c74448,
    base_anim: 0x3a2ed50,
};

pub const BASE_ADDRESSES_1_03_1: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c61588,
    chr_dbg_flags: 0x3c6504f,
    csfd4_virtual_memory_flag: 0x3c672a8,
    cs_flipper: 0x4487908,
    cs_lua_event_manager: 0x3c66cb8,
    cs_menu_man: 0x8ba63d24,
    cs_menu_man_imp: 0x3c6a700,
    cs_net_man: 0x3c59d20,
    cs_regulation_manager: 0x3c84e38,
    cs_session_manager: 0x3c78900,
    damage_ctrl: 0x3c65228,
    field_area: 0x3c68040,
    game_data_man: 0x3c5cd78,
    game_man: 0x3c68758,
    global_pos: 0x3c64e38,
    group_mask: 0x3a4a807,
    hit_ins: 0x3c68ef0,
    hit_ins_hitbox_offset: 0x3c68efc,
    map_item_man: 0x3c668c0,
    menu_man_ins: 0x3c6a700,
    msg_repository: 0x3c7b928,
    solo_param_repository: 0x3c80158,
    world_chr_man: 0x3c64e38,
    world_chr_man_dbg: 0x3c65048,
    world_chr_man_imp: 0x3c64e38,
    func_item_spawn: 0x549c70,
    func_item_inject: 0x54f640,
    func_remove_intro_screens: 0xab022d,
    func_dbg_action_force: 0x523c67d,
    lua_warp: 0x58671e,
    current_target: 0x6f1ee2,
    base_fps: 0x3c74448,
    base_anim: 0x3a2ed50,
};

pub const BASE_ADDRESSES_1_03_2: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c61588,
    chr_dbg_flags: 0x3c6504f,
    csfd4_virtual_memory_flag: 0x3c672a8,
    cs_flipper: 0x4487908,
    cs_lua_event_manager: 0x3c66cb8,
    cs_menu_man: 0x8ba63d14,
    cs_menu_man_imp: 0x3c6a700,
    cs_net_man: 0x3c59d20,
    cs_regulation_manager: 0x3c84e38,
    cs_session_manager: 0x3c78900,
    damage_ctrl: 0x3c65228,
    field_area: 0x3c68040,
    game_data_man: 0x3c5cd78,
    game_man: 0x3c68758,
    global_pos: 0x3c64e38,
    group_mask: 0x3a4a807,
    hit_ins: 0x3c68ef0,
    hit_ins_hitbox_offset: 0x3c68efc,
    map_item_man: 0x3c668c0,
    menu_man_ins: 0x3c6a700,
    msg_repository: 0x3c7b928,
    solo_param_repository: 0x3c80158,
    world_chr_man: 0x3c64e38,
    world_chr_man_dbg: 0x3c65048,
    world_chr_man_imp: 0x3c64e38,
    func_item_spawn: 0x549c70,
    func_item_inject: 0x54f640,
    func_remove_intro_screens: 0xab020d,
    func_dbg_action_force: 0x54e7d76,
    lua_warp: 0x58670e,
    current_target: 0x6f1ec2,
    base_fps: 0x3c74448,
    base_anim: 0x3a2ed50,
};

pub const BASE_ADDRESSES_1_04_0: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c04828,
    chr_dbg_flags: 0x3c082df,
    csfd4_virtual_memory_flag: 0x3c0a538,
    cs_flipper: 0x442ab08,
    cs_lua_event_manager: 0x3c09f48,
    cs_menu_man: 0x8ba66a14,
    cs_menu_man_imp: 0x3c0d9d0,
    cs_net_man: 0x3bfcf40,
    cs_regulation_manager: 0x3c27fd8,
    cs_session_manager: 0x3c1ba90,
    damage_ctrl: 0x3c084b8,
    field_area: 0x3c0b2c0,
    game_data_man: 0x3c00028,
    game_man: 0x3c0ba08,
    global_pos: 0x3c080e8,
    group_mask: 0x39f681f,
    hit_ins: 0x3c0c1c8,
    hit_ins_hitbox_offset: 0x3c0c1b4,
    map_item_man: 0x3c09b50,
    menu_man_ins: 0x3c0d9d0,
    msg_repository: 0x3c1eab8,
    solo_param_repository: 0x3c232e8,
    world_chr_man: 0x3c080e8,
    world_chr_man_dbg: 0x3c082d8,
    world_chr_man_imp: 0x3c080e8,
    func_item_spawn: 0x54c950,
    func_item_inject: 0x552330,
    func_remove_intro_screens: 0xa8fb6d,
    func_dbg_action_force: 0x4ff4205,
    lua_warp: 0x58940e,
    current_target: 0x6f5d92,
    base_fps: 0x3c175c8,
    base_anim: 0x39dad60,
};

pub const BASE_ADDRESSES_1_04_1: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c04828,
    chr_dbg_flags: 0x3c082df,
    csfd4_virtual_memory_flag: 0x3c0a538,
    cs_flipper: 0x442ab08,
    cs_lua_event_manager: 0x3c09f48,
    cs_menu_man: 0x8ba66924,
    cs_menu_man_imp: 0x3c0d9d0,
    cs_net_man: 0x3bfcf40,
    cs_regulation_manager: 0x3c27fd8,
    cs_session_manager: 0x3c1ba90,
    damage_ctrl: 0x3c084b8,
    field_area: 0x3c0b2c0,
    game_data_man: 0x3c00028,
    game_man: 0x3c0ba08,
    global_pos: 0x3c080e8,
    group_mask: 0x39f681f,
    hit_ins: 0x3c0c1c8,
    hit_ins_hitbox_offset: 0x3c0c1b4,
    map_item_man: 0x3c09b50,
    menu_man_ins: 0x3c0d9d0,
    msg_repository: 0x3c1eab8,
    solo_param_repository: 0x3c232e8,
    world_chr_man: 0x3c080e8,
    world_chr_man_dbg: 0x3c082d8,
    world_chr_man_imp: 0x3c080e8,
    func_item_spawn: 0x54c860,
    func_item_inject: 0x552240,
    func_remove_intro_screens: 0xa8fa7d,
    func_dbg_action_force: 0x55ac951,
    lua_warp: 0x58931e,
    current_target: 0x6f5ca2,
    base_fps: 0x3c175c8,
    base_anim: 0x39dad60,
};

pub const BASE_ADDRESSES_1_05_0: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c1c6e8,
    chr_dbg_flags: 0x3c20089,
    csfd4_virtual_memory_flag: 0x3c222e8,
    cs_flipper: 0x4442c18,
    cs_lua_event_manager: 0x3c21cf8,
    cs_menu_man: 0x8ba67044,
    cs_menu_man_imp: 0x3c25780,
    cs_net_man: 0x3c14e00,
    cs_regulation_manager: 0x3c3fdd8,
    cs_session_manager: 0x3c33850,
    damage_ctrl: 0x3c20268,
    field_area: 0x3c23070,
    game_data_man: 0x3c17ee8,
    game_man: 0x3c237b8,
    global_pos: 0x3c1fe98,
    group_mask: 0x3a0d838,
    hit_ins: 0x3c23f78,
    hit_ins_hitbox_offset: 0x3c23f64,
    map_item_man: 0x3c21900,
    menu_man_ins: 0x3c25780,
    msg_repository: 0x3c36878,
    solo_param_repository: 0x3c3b0d8,
    world_chr_man: 0x3c1fe98,
    world_chr_man_dbg: 0x3c200d0,
    world_chr_man_imp: 0x3c1fe98,
    func_item_spawn: 0x54ce60,
    func_item_inject: 0x552840,
    func_remove_intro_screens: 0xa9417d,
    func_dbg_action_force: 0x3683da,
    lua_warp: 0x58992e,
    current_target: 0x6f6b52,
    base_fps: 0x3c2f430,
    base_anim: 0x39f1d70,
};

pub const BASE_ADDRESSES_1_06_0: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c2d918,
    chr_dbg_flags: 0x3c312af,
    csfd4_virtual_memory_flag: 0x3c33508,
    cs_flipper: 0x4453e98,
    cs_lua_event_manager: 0x3c32f18,
    cs_menu_man: 0x8ba68604,
    cs_menu_man_imp: 0x3c369a0,
    cs_net_man: 0x3c26020,
    cs_regulation_manager: 0x3c51038,
    cs_session_manager: 0x3c44ac0,
    damage_ctrl: 0x3c31488,
    field_area: 0x3c34298,
    game_data_man: 0x3c29108,
    game_man: 0x3c349d8,
    global_pos: 0x3c310b8,
    group_mask: 0x3a1e82f,
    hit_ins: 0x3c35180,
    hit_ins_hitbox_offset: 0x3c3518c,
    map_item_man: 0x3c32b20,
    menu_man_ins: 0x3c369a0,
    msg_repository: 0x3c47ae8,
    solo_param_repository: 0x3c4c348,
    world_chr_man: 0x3c310b8,
    world_chr_man_dbg: 0x3c312a8,
    world_chr_man_imp: 0x3c310b8,
    func_item_spawn: 0x54dfd0,
    func_item_inject: 0x5539e0,
    func_remove_intro_screens: 0xa9807d,
    func_dbg_action_force: 0x4f2244f,
    lua_warp: 0x58abfe,
    current_target: 0x6f89a2,
    base_fps: 0x3c405a8,
    base_anim: 0x3a02d70,
};

pub const BASE_ADDRESSES_1_07_0: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c482c8,
    chr_dbg_flags: 0x3c4bc6f,
    csfd4_virtual_memory_flag: 0x3c4dec8,
    cs_flipper: 0x446e858,
    cs_lua_event_manager: 0x3c4d8d8,
    cs_menu_man: 0x8ba69484,
    cs_menu_man_imp: 0x3c51360,
    cs_net_man: 0x3c409e0,
    cs_regulation_manager: 0x3c6ba08,
    cs_session_manager: 0x3c5f490,
    damage_ctrl: 0x3c4be48,
    field_area: 0x3c4ec50,
    game_data_man: 0x3c43ac8,
    game_man: 0x3c4f398,
    global_pos: 0x3c4ba78,
    group_mask: 0x3a37a1f,
    hit_ins: 0x3c4fb40,
    hit_ins_hitbox_offset: 0x3c4fb4c,
    map_item_man: 0x3c4d4e0,
    menu_man_ins: 0x3c51360,
    msg_repository: 0x3c624b8,
    solo_param_repository: 0x3c66d18,
    world_chr_man: 0x3c4ba78,
    world_chr_man_dbg: 0x3c4bc68,
    world_chr_man_imp: 0x3c4ba78,
    func_item_spawn: 0x54ee40,
    func_item_inject: 0x554850,
    func_remove_intro_screens: 0xa9972d,
    func_dbg_action_force: 0xb7f33f,
    lua_warp: 0x58ba6e,
    current_target: 0x6fa0f2,
    base_fps: 0x3c5b030,
    base_anim: 0x3a18d80,
};

pub const BASE_ADDRESSES_1_08_0: BaseAddresses = BaseAddresses {
    bullet_man: 0x3cd6158,
    chr_dbg_flags: 0x3cd9b96,
    csfd4_virtual_memory_flag: 0x3cdbdf8,
    cs_flipper: 0x44fd2c8,
    cs_lua_event_manager: 0x3cdb7f8,
    cs_menu_man: 0x8ba74bc4,
    cs_menu_man_imp: 0x3cdf140,
    cs_net_man: 0x3cce860,
    cs_regulation_manager: 0x3cfa478,
    cs_session_manager: 0x3ceddb0,
    damage_ctrl: 0x3cd9d68,
    field_area: 0x3cdcb80,
    game_data_man: 0x3cd1948,
    game_man: 0x3cdd2c8,
    global_pos: 0x3cd9998,
    group_mask: 0x3abfae7,
    hit_ins: 0x3cdda70,
    hit_ins_hitbox_offset: 0x3cdda7c,
    map_item_man: 0x3cdb400,
    menu_man_ins: 0x3cdf140,
    msg_repository: 0x3cf0dd8,
    solo_param_repository: 0x3cf5788,
    world_chr_man: 0x3cd9998,
    world_chr_man_dbg: 0x3cd9bd0,
    world_chr_man_imp: 0x3cd9998,
    func_item_spawn: 0x556790,
    func_item_inject: 0x55c1a0,
    func_remove_intro_screens: 0xadb0fd,
    func_dbg_action_force: 0x575772e,
    lua_warp: 0x5951de,
    current_target: 0x7078d2,
    base_fps: 0x3ce9898,
    base_anim: 0x3aa0da0,
};

pub const BASE_ADDRESSES_1_08_1: BaseAddresses = BaseAddresses {
    bullet_man: 0x3cd6158,
    chr_dbg_flags: 0x3cd9b96,
    csfd4_virtual_memory_flag: 0x3cdbdf8,
    cs_flipper: 0x44fd2c8,
    cs_lua_event_manager: 0x3cdb7f8,
    cs_menu_man: 0x8ba74bc4,
    cs_menu_man_imp: 0x3cdf140,
    cs_net_man: 0x3cce860,
    cs_regulation_manager: 0x3cfa478,
    cs_session_manager: 0x3ceddb0,
    damage_ctrl: 0x3cd9d68,
    field_area: 0x3cdcb80,
    game_data_man: 0x3cd1948,
    game_man: 0x3cdd2c8,
    global_pos: 0x3cd9998,
    group_mask: 0x3abfae7,
    hit_ins: 0x3cdda70,
    hit_ins_hitbox_offset: 0x3cdda7c,
    map_item_man: 0x3cdb400,
    menu_man_ins: 0x3cdf140,
    msg_repository: 0x3cf0dd8,
    solo_param_repository: 0x3cf5788,
    world_chr_man: 0x3cd9998,
    world_chr_man_dbg: 0x3cd9bd0,
    world_chr_man_imp: 0x3cd9998,
    func_item_spawn: 0x556790,
    func_item_inject: 0x55c1a0,
    func_remove_intro_screens: 0xadb0fd,
    func_dbg_action_force: 0x56ba77a,
    lua_warp: 0x5951de,
    current_target: 0x7078d2,
    base_fps: 0x3ce9898,
    base_anim: 0x3aa0da0,
};

pub const BASE_ADDRESSES_1_09_0: BaseAddresses = BaseAddresses {
    bullet_man: 0x3cd9598,
    chr_dbg_flags: 0x3cdcfe7,
    csfd4_virtual_memory_flag: 0x3cdf238,
    cs_flipper: 0x4500708,
    cs_lua_event_manager: 0x3cdec38,
    cs_menu_man: 0x8ba74f44,
    cs_menu_man_imp: 0x3ce2580,
    cs_net_man: 0x3cd1ca0,
    cs_regulation_manager: 0x3cfd8b8,
    cs_session_manager: 0x3cf11f0,
    damage_ctrl: 0x3cdd1a8,
    field_area: 0x3cdffc0,
    game_data_man: 0x3cd4d88,
    game_man: 0x3ce0708,
    global_pos: 0x3cdcdd8,
    group_mask: 0x3ac2ae7,
    hit_ins: 0x3ce0eb0,
    hit_ins_hitbox_offset: 0x3ce0ebc,
    map_item_man: 0x3cde840,
    menu_man_ins: 0x3ce2580,
    msg_repository: 0x3cf4218,
    solo_param_repository: 0x3cf8bc8,
    world_chr_man: 0x3cdcdd8,
    world_chr_man_dbg: 0x3cdd010,
    world_chr_man_imp: 0x3cdcdd8,
    func_item_spawn: 0x556ab0,
    func_item_inject: 0x55c4c0,
    func_remove_intro_screens: 0xaddc8d,
    func_dbg_action_force: 0x5679cfa,
    lua_warp: 0x59555e,
    current_target: 0x708972,
    base_fps: 0x3ceccd8,
    base_anim: 0x3aa3da0,
};

pub const BASE_ADDRESSES_1_09_1: BaseAddresses = BaseAddresses {
    bullet_man: 0x3cd9598,
    chr_dbg_flags: 0x3cdcfe7,
    csfd4_virtual_memory_flag: 0x3cdf238,
    cs_flipper: 0x4500708,
    cs_lua_event_manager: 0x3cdec38,
    cs_menu_man: 0x8ba74fa4,
    cs_menu_man_imp: 0x3ce2580,
    cs_net_man: 0x3cd1ca0,
    cs_regulation_manager: 0x3cfd8b8,
    cs_session_manager: 0x3cf11f0,
    damage_ctrl: 0x3cdd1a8,
    field_area: 0x3cdffc0,
    game_data_man: 0x3cd4d88,
    game_man: 0x3ce0708,
    global_pos: 0x3cdcdd8,
    group_mask: 0x3ac2ae7,
    hit_ins: 0x3ce0eb0,
    hit_ins_hitbox_offset: 0x3ce0ebc,
    map_item_man: 0x3cde840,
    menu_man_ins: 0x3ce2580,
    msg_repository: 0x3cf4218,
    solo_param_repository: 0x3cf8bc8,
    world_chr_man: 0x3cdcdd8,
    world_chr_man_dbg: 0x3cdd010,
    world_chr_man_imp: 0x3cdcdd8,
    func_item_spawn: 0x556b10,
    func_item_inject: 0x55c520,
    func_remove_intro_screens: 0xaddced,
    func_dbg_action_force: 0x17b591f,
    lua_warp: 0x5955be,
    current_target: 0x7089d2,
    base_fps: 0x3ceccd8,
    base_anim: 0x3aa3da0,
};

pub const BASE_ADDRESSES_2_00_0: BaseAddresses = BaseAddresses {
    bullet_man: 0x3cd9598,
    chr_dbg_flags: 0x3cdcfe7,
    csfd4_virtual_memory_flag: 0x3cdf238,
    cs_flipper: 0x4500708,
    cs_lua_event_manager: 0x3cdec38,
    cs_menu_man: 0x8ba751e4,
    cs_menu_man_imp: 0x3ce2580,
    cs_net_man: 0x3cd1ca0,
    cs_regulation_manager: 0x3cfd8b8,
    cs_session_manager: 0x3cf11f0,
    damage_ctrl: 0x3cdd1a8,
    field_area: 0x3cdffc0,
    game_data_man: 0x3cd4d88,
    game_man: 0x3ce0708,
    global_pos: 0x3cdcdd8,
    group_mask: 0x3ac2ae7,
    hit_ins: 0x3ce0eb0,
    hit_ins_hitbox_offset: 0x3ce0ebc,
    map_item_man: 0x3cde840,
    menu_man_ins: 0x3ce2580,
    msg_repository: 0x3cf4218,
    solo_param_repository: 0x3cf8bc8,
    world_chr_man: 0x3cdcdd8,
    world_chr_man_dbg: 0x3cdd010,
    world_chr_man_imp: 0x3cdcdd8,
    func_item_spawn: 0x556d50,
    func_item_inject: 0x55c760,
    func_remove_intro_screens: 0xaddf7d,
    func_dbg_action_force: 0x1d9c6f0,
    lua_warp: 0x5957fe,
    current_target: 0x708c62,
    base_fps: 0x3ceccd8,
    base_anim: 0x3aa3da0,
};

pub const BASE_ADDRESSES_2_00_1: BaseAddresses = BaseAddresses {
    bullet_man: 0x3cd9598,
    chr_dbg_flags: 0x3cdcfe7,
    csfd4_virtual_memory_flag: 0x3cdf238,
    cs_flipper: 0x4500708,
    cs_lua_event_manager: 0x3cdec38,
    cs_menu_man: 0x8ba751e4,
    cs_menu_man_imp: 0x3ce2580,
    cs_net_man: 0x3cd1ca0,
    cs_regulation_manager: 0x3cfd8b8,
    cs_session_manager: 0x3cf11f0,
    damage_ctrl: 0x3cdd1a8,
    field_area: 0x3cdffc0,
    game_data_man: 0x3cd4d88,
    game_man: 0x3ce0708,
    global_pos: 0x3cdcdd8,
    group_mask: 0x3ac2ae7,
    hit_ins: 0x3ce0eb0,
    hit_ins_hitbox_offset: 0x3ce0ebc,
    map_item_man: 0x3cde840,
    menu_man_ins: 0x3ce2580,
    msg_repository: 0x3cf4218,
    solo_param_repository: 0x3cf8bc8,
    world_chr_man: 0x3cdcdd8,
    world_chr_man_dbg: 0x3cdd010,
    world_chr_man_imp: 0x3cdcdd8,
    func_item_spawn: 0x556d50,
    func_item_inject: 0x55c760,
    func_remove_intro_screens: 0xaddf7d,
    func_dbg_action_force: 0x55611f7,
    lua_warp: 0x5957fe,
    current_target: 0x708c62,
    base_fps: 0x3ceccd8,
    base_anim: 0x3aa3da0,
};

pub const BASE_ADDRESSES_2_02_0: BaseAddresses = BaseAddresses {
    bullet_man: 0x3d62748,
    chr_dbg_flags: 0x3d6619f,
    csfd4_virtual_memory_flag: 0x3d68448,
    cs_flipper: 0x4589ad8,
    cs_lua_event_manager: 0x3d67e48,
    cs_menu_man: 0x8ba7b444,
    cs_menu_man_imp: 0x3d6b7b0,
    cs_net_man: 0x3d5ae60,
    cs_regulation_manager: 0x3d86c58,
    cs_session_manager: 0x3d7a4d0,
    damage_ctrl: 0x3d66378,
    field_area: 0x3d691d8,
    game_data_man: 0x3d5df38,
    game_man: 0x3d69918,
    global_pos: 0x3d65f88,
    group_mask: 0x3b33cff,
    hit_ins: 0x3d6a0e0,
    hit_ins_hitbox_offset: 0x3d6a0ec,
    map_item_man: 0x3d67a50,
    menu_man_ins: 0x3d6b7b0,
    msg_repository: 0x3d7d4f8,
    solo_param_repository: 0x3d81ee8,
    world_chr_man: 0x3d65f88,
    world_chr_man_dbg: 0x3d66198,
    world_chr_man_imp: 0x3d65f88,
    func_item_spawn: 0x55aad0,
    func_item_inject: 0x5604e0,
    func_remove_intro_screens: 0xb0bd7d,
    func_dbg_action_force: 0x5aa1b83,
    lua_warp: 0x599b1e,
    current_target: 0x716fae,
    base_fps: 0x3d76060,
    base_anim: 0x3b12e30,
};

pub const BASE_ADDRESSES_2_02_3: BaseAddresses = BaseAddresses {
    bullet_man: 0x3d62768,
    chr_dbg_flags: 0x3d661bf,
    csfd4_virtual_memory_flag: 0x3d68468,
    cs_flipper: 0x4589af8,
    cs_lua_event_manager: 0x3d67e68,
    cs_menu_man: 0x8ba7b444,
    cs_menu_man_imp: 0x3d6b7d0,
    cs_net_man: 0x3d5ae80,
    cs_regulation_manager: 0x3d86c78,
    cs_session_manager: 0x3d7a4f0,
    damage_ctrl: 0x3d66398,
    field_area: 0x3d691f8,
    game_data_man: 0x3d5df58,
    game_man: 0x3d69938,
    global_pos: 0x3d65fa8,
    group_mask: 0x3b33cff,
    hit_ins: 0x3d6a100,
    hit_ins_hitbox_offset: 0x3d6a10c,
    map_item_man: 0x3d67a70,
    menu_man_ins: 0x3d6b7d0,
    msg_repository: 0x3d7d518,
    solo_param_repository: 0x3d81f08,
    world_chr_man: 0x3d65fa8,
    world_chr_man_dbg: 0x3d661b8,
    world_chr_man_imp: 0x3d65fa8,
    func_item_spawn: 0x55aad0,
    func_item_inject: 0x5604e0,
    func_remove_intro_screens: 0xb0bd7d,
    func_dbg_action_force: 0x5a454d7,
    lua_warp: 0x599b1e,
    current_target: 0x716fae,
    base_fps: 0x3d76080,
    base_anim: 0x3b12e30,
};

pub const BASE_ADDRESSES_2_03_0: BaseAddresses = BaseAddresses {
    bullet_man: 0x3d62768,
    chr_dbg_flags: 0x3d661bf,
    csfd4_virtual_memory_flag: 0x3d68468,
    cs_flipper: 0x4589af8,
    cs_lua_event_manager: 0x3d67e68,
    cs_menu_man: 0x8ba7b5c4,
    cs_menu_man_imp: 0x3d6b7d0,
    cs_net_man: 0x3d5ae80,
    cs_regulation_manager: 0x3d86c78,
    cs_session_manager: 0x3d7a4f0,
    damage_ctrl: 0x3d66398,
    field_area: 0x3d691f8,
    game_data_man: 0x3d5df58,
    game_man: 0x3d69938,
    global_pos: 0x3d65fa8,
    group_mask: 0x3b33cff,
    hit_ins: 0x3d6a100,
    hit_ins_hitbox_offset: 0x3d6a10c,
    map_item_man: 0x3d67a70,
    menu_man_ins: 0x3d6b7d0,
    msg_repository: 0x3d7d518,
    solo_param_repository: 0x3d81f08,
    world_chr_man: 0x3d65fa8,
    world_chr_man_dbg: 0x3d661b8,
    world_chr_man_imp: 0x3d65fa8,
    func_item_spawn: 0x55ac50,
    func_item_inject: 0x560660,
    func_remove_intro_screens: 0xb0c0ed,
    func_dbg_action_force: 0x57d2871,
    lua_warp: 0x599c9e,
    current_target: 0x71719e,
    base_fps: 0x3d76080,
    base_anim: 0x3b12e30,
};
