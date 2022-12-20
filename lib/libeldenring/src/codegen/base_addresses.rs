// **********************************
// *** AUTOGENERATED, DO NOT EDIT ***
// **********************************
#[derive(Debug)]
pub struct BaseAddresses {
    pub bullet_man: usize,
    pub chr_dbg_flags: usize,
    pub csfd4_virtual_memory_flag: usize,
    pub cs_flipper: usize,
    pub cs_menu_man: usize,
    pub cs_menu_man_imp: usize,
    pub cs_net_man: usize,
    pub cs_regulation_manager: usize,
    pub cs_session_manager: usize,
    pub damage_ctrl: usize,
    pub field_area: usize,
    pub game_data_man: usize,
    pub game_man: usize,
    pub group_mask: usize,
    pub hit_ins: usize,
    pub hit_ins_hitbox_offset: usize,
    pub global_pos: usize,
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
}

impl BaseAddresses {
    pub fn with_module_base_addr(self, base: usize) -> BaseAddresses {
        BaseAddresses {
            bullet_man: self.bullet_man + base,
            chr_dbg_flags: self.chr_dbg_flags + base,
            csfd4_virtual_memory_flag: self.csfd4_virtual_memory_flag + base,
            cs_flipper: self.cs_flipper + base,
            cs_menu_man: self.cs_menu_man + base,
            cs_menu_man_imp: self.cs_menu_man_imp + base,
            cs_net_man: self.cs_net_man + base,
            cs_regulation_manager: self.cs_regulation_manager + base,
            cs_session_manager: self.cs_session_manager + base,
            damage_ctrl: self.damage_ctrl + base,
            field_area: self.field_area + base,
            game_data_man: self.game_data_man + base,
            game_man: self.game_man + base,
            group_mask: self.group_mask + base,
            hit_ins: self.hit_ins + base,
            hit_ins_hitbox_offset: self.hit_ins_hitbox_offset + base,
            global_pos: self.global_pos + base,
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
}

impl From<(u32, u32, u32)> for Version {
    fn from(v: (u32, u32, u32)) -> Self {
        match v {
            (1, 2, 0) => Version::V1_02_0,
            (1, 2, 1) => Version::V1_02_1,
            (1, 2, 2) => Version::V1_02_2,
            (1, 2, 3) => Version::V1_02_3,
            (1, 3, 0) => Version::V1_03_0,
            (1, 3, 1) => Version::V1_03_1,
            (1, 3, 2) => Version::V1_03_2,
            (1, 4, 0) => Version::V1_04_0,
            (1, 4, 1) => Version::V1_04_1,
            (1, 5, 0) => Version::V1_05_0,
            (1, 6, 0) => Version::V1_06_0,
            (1, 7, 0) => Version::V1_07_0,
            (1, 8, 0) => Version::V1_08_0,
            (1, 8, 1) => Version::V1_08_1,
            (maj, min, patch) => {
                log::error!("Unrecognized version {maj}.{min:02}.{patch}");
                panic!()
            }
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
        }
    }
}

pub const BASE_ADDRESSES_1_02_0: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c4c9c8,
    csfd4_virtual_memory_flag: 0x3c526e8,
    cs_flipper: 0x4473138,
    cs_menu_man: 0x8ba62b94,
    cs_menu_man_imp: 0x3c55b30,
    cs_net_man: 0x3c45160,
    cs_regulation_manager: 0x3c70658,
    cs_session_manager: 0x3c63d30,
    chr_dbg_flags: 0x3c5047f,
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
    func_dbg_action_force: 0x527214a,
    func_item_inject: 0x54e570,
    func_item_spawn: 0x548c20,
    func_remove_intro_screens: 0xaaad4a,
};

pub const BASE_ADDRESSES_1_02_1: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c4c9e8,
    csfd4_virtual_memory_flag: 0x3c52708,
    cs_flipper: 0x4472d58,
    cs_menu_man: 0x8ba62c04,
    cs_menu_man_imp: 0x3c55b50,
    cs_net_man: 0x3c45180,
    cs_regulation_manager: 0x3c70278,
    cs_session_manager: 0x3c63d50,
    chr_dbg_flags: 0x3c5049f,
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
    func_dbg_action_force: 0x53f757d,
    func_item_inject: 0x54e5e0,
    func_item_spawn: 0x548c90,
    func_remove_intro_screens: 0xaaadca,
};

pub const BASE_ADDRESSES_1_02_2: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c4ca08,
    csfd4_virtual_memory_flag: 0x3c52728,
    cs_flipper: 0x4472d78,
    cs_menu_man: 0x8ba62c04,
    cs_menu_man_imp: 0x3c55b70,
    cs_net_man: 0x3c451a0,
    cs_regulation_manager: 0x3c70298,
    cs_session_manager: 0x3c63d70,
    chr_dbg_flags: 0x3c504bf,
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
    func_dbg_action_force: 0x4f45cb0,
    func_item_inject: 0x54e5e0,
    func_item_spawn: 0x548c90,
    func_remove_intro_screens: 0xaaae3a,
};

pub const BASE_ADDRESSES_1_02_3: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c4fa28,
    csfd4_virtual_memory_flag: 0x3c55748,
    cs_flipper: 0x4475d98,
    cs_menu_man: 0x8ba62d24,
    cs_menu_man_imp: 0x3c58b90,
    cs_net_man: 0x3c481c0,
    cs_regulation_manager: 0x3c732b8,
    cs_session_manager: 0x3c66d80,
    chr_dbg_flags: 0x3c534dd,
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
    func_dbg_action_force: 0x4edd1ef,
    func_item_inject: 0x54e700,
    func_item_spawn: 0x548db0,
    func_remove_intro_screens: 0xaaaf1a,
};

pub const BASE_ADDRESSES_1_03_0: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c61588,
    csfd4_virtual_memory_flag: 0x3c672a8,
    cs_flipper: 0x4487918,
    cs_menu_man: 0x8ba63d24,
    cs_menu_man_imp: 0x3c6a700,
    cs_net_man: 0x3c59d20,
    cs_regulation_manager: 0x3c84e38,
    cs_session_manager: 0x3c78900,
    chr_dbg_flags: 0x3c6504f,
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
    func_dbg_action_force: 0x1e42a5c,
    func_item_inject: 0x54f640,
    func_item_spawn: 0x549c70,
    func_remove_intro_screens: 0xab021d,
};

pub const BASE_ADDRESSES_1_03_1: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c61588,
    csfd4_virtual_memory_flag: 0x3c672a8,
    cs_flipper: 0x4487908,
    cs_menu_man: 0x8ba63d24,
    cs_menu_man_imp: 0x3c6a700,
    cs_net_man: 0x3c59d20,
    cs_regulation_manager: 0x3c84e38,
    cs_session_manager: 0x3c78900,
    chr_dbg_flags: 0x3c6504f,
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
    func_dbg_action_force: 0x523c67d,
    func_item_inject: 0x54f640,
    func_item_spawn: 0x549c70,
    func_remove_intro_screens: 0xab022d,
};

pub const BASE_ADDRESSES_1_03_2: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c61588,
    csfd4_virtual_memory_flag: 0x3c672a8,
    cs_flipper: 0x4487908,
    cs_menu_man: 0x8ba63d14,
    cs_menu_man_imp: 0x3c6a700,
    cs_net_man: 0x3c59d20,
    cs_regulation_manager: 0x3c84e38,
    cs_session_manager: 0x3c78900,
    chr_dbg_flags: 0x3c6504f,
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
    func_dbg_action_force: 0x54e7d76,
    func_item_inject: 0x54f640,
    func_item_spawn: 0x549c70,
    func_remove_intro_screens: 0xab020d,
};

pub const BASE_ADDRESSES_1_04_0: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c04828,
    csfd4_virtual_memory_flag: 0x3c0a538,
    cs_flipper: 0x442ab08,
    cs_menu_man: 0x8ba66a14,
    cs_menu_man_imp: 0x3c0d9d0,
    cs_net_man: 0x3bfcf40,
    cs_regulation_manager: 0x3c27fd8,
    cs_session_manager: 0x3c1ba90,
    chr_dbg_flags: 0x3c082df,
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
    func_dbg_action_force: 0x4ff4205,
    func_item_inject: 0x552330,
    func_item_spawn: 0x54c950,
    func_remove_intro_screens: 0xa8fb6d,
};

pub const BASE_ADDRESSES_1_04_1: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c04828,
    csfd4_virtual_memory_flag: 0x3c0a538,
    cs_flipper: 0x442ab08,
    cs_menu_man: 0x8ba66924,
    cs_menu_man_imp: 0x3c0d9d0,
    cs_net_man: 0x3bfcf40,
    cs_regulation_manager: 0x3c27fd8,
    cs_session_manager: 0x3c1ba90,
    chr_dbg_flags: 0x3c082df,
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
    func_dbg_action_force: 0x55ac951,
    func_item_inject: 0x552240,
    func_item_spawn: 0x54c860,
    func_remove_intro_screens: 0xa8fa7d,
};

pub const BASE_ADDRESSES_1_05_0: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c1c6e8,
    csfd4_virtual_memory_flag: 0x3c222e8,
    cs_flipper: 0x4442c18,
    cs_menu_man: 0x8ba67044,
    cs_menu_man_imp: 0x3c25780,
    cs_net_man: 0x3c14e00,
    cs_regulation_manager: 0x3c3fdd8,
    cs_session_manager: 0x3c33850,
    chr_dbg_flags: 0x3c20089,
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
    func_dbg_action_force: 0x3683da,
    func_item_inject: 0x552840,
    func_item_spawn: 0x54ce60,
    func_remove_intro_screens: 0xa9417d,
};

pub const BASE_ADDRESSES_1_06_0: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c2d918,
    csfd4_virtual_memory_flag: 0x3c33508,
    cs_flipper: 0x4453e98,
    cs_menu_man: 0x8ba68604,
    cs_menu_man_imp: 0x3c369a0,
    cs_net_man: 0x3c26020,
    cs_regulation_manager: 0x3c51038,
    cs_session_manager: 0x3c44ac0,
    chr_dbg_flags: 0x3c312af,
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
    func_dbg_action_force: 0x4f2244f,
    func_item_inject: 0x5539e0,
    func_item_spawn: 0x54dfd0,
    func_remove_intro_screens: 0xa9807d,
};

pub const BASE_ADDRESSES_1_07_0: BaseAddresses = BaseAddresses {
    bullet_man: 0x3c482c8,
    csfd4_virtual_memory_flag: 0x3c4dec8,
    cs_flipper: 0x446e858,
    cs_menu_man: 0x8ba69484,
    cs_menu_man_imp: 0x3c51360,
    cs_net_man: 0x3c409e0,
    cs_regulation_manager: 0x3c6ba08,
    cs_session_manager: 0x3c5f490,
    chr_dbg_flags: 0x3c4bc6f,
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
    func_dbg_action_force: 0xb7f33f,
    func_item_inject: 0x554850,
    func_item_spawn: 0x54ee40,
    func_remove_intro_screens: 0xa9972d,
};

pub const BASE_ADDRESSES_1_08_0: BaseAddresses = BaseAddresses {
    bullet_man: 0x3cd6158,
    csfd4_virtual_memory_flag: 0x3cdbdf8,
    cs_flipper: 0x44fd2c8,
    cs_menu_man: 0x8ba74bc4,
    cs_menu_man_imp: 0x3cdf140,
    cs_net_man: 0x3cce860,
    cs_regulation_manager: 0x3cfa478,
    cs_session_manager: 0x3ceddb0,
    chr_dbg_flags: 0x3cd9b96,
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
    func_dbg_action_force: 0x575772e,
    func_item_inject: 0x55c1a0,
    func_item_spawn: 0x556790,
    func_remove_intro_screens: 0xadb0fd,
};

pub const BASE_ADDRESSES_1_08_1: BaseAddresses = BaseAddresses {
    bullet_man: 0x3cd6158,
    csfd4_virtual_memory_flag: 0x3cdbdf8,
    cs_flipper: 0x44fd2c8,
    cs_menu_man: 0x8ba74bc4,
    cs_menu_man_imp: 0x3cdf140,
    cs_net_man: 0x3cce860,
    cs_regulation_manager: 0x3cfa478,
    cs_session_manager: 0x3ceddb0,
    chr_dbg_flags: 0x3cd9b96,
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
    func_dbg_action_force: 0x56ba77a,
    func_item_inject: 0x55c1a0,
    func_item_spawn: 0x556790,
    func_remove_intro_screens: 0xadb0fd,
};

