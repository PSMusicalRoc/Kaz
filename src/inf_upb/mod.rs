use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash::app::sv_animcmd::*;
use smash::phx::Hash40;

#[fighter_frame( agent = FIGHTER_KIND_DEMON )]
fn inf_upb(fighter: &mut L2CFighterCommon) {
    unsafe {
        let state = StatusModule::status_kind(fighter.module_accessor);
        if [
            *FIGHTER_STATUS_KIND_SPECIAL_HI,
            *FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_FALL,
            *FIGHTER_DEMON_STATUS_KIND_SPECIAL_HI_RISE
        ].contains(&state)
        {
            if ControlModule::get_stick_y(fighter.module_accessor) > 0.6 &&
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            {
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_HI, true);
            }
        }
    }
}

pub fn install()
{
    install_agent_frames!(
        inf_upb
    );
}