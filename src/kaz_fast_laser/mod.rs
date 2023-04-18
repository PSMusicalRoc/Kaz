use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash::app::sv_animcmd::*;
use smash::phx::Hash40;

#[acmd_script( agent = "demon", script = "game_specialnstart", category = ACMD_GAME, low_priority )]
unsafe fn kaz_nspecial_startup(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.05);
    frame(fighter.lua_state_agent, 11.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 17.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    //macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 19.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 23.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
}

#[acmd_script( agent = "demon", script = "game_specialairnstart", category = ACMD_GAME, low_priority )]
unsafe fn kaz_air_nspecial_startup(fighter: &mut L2CAgentBase) {
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 10.0);
    frame(fighter.lua_state_agent, 4.0);
    macros::FT_MOTION_RATE(fighter, 0.05);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 6.0);
    frame(fighter.lua_state_agent, 6.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 5.0);
    frame(fighter.lua_state_agent, 9.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 4.0);
    frame(fighter.lua_state_agent, 12.0);
    FighterSpecializer_Demon::set_devil(fighter.module_accessor, true, 3.0);
}

pub fn install()
{
    smashline::install_acmd_scripts!(
        kaz_nspecial_startup,
        kaz_air_nspecial_startup
    );
}