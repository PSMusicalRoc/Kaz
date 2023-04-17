use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;

#[fighter_frame( agent = FIGHTER_KIND_DEMON )]
fn kaz_jc_sideb(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status = StatusModule::status_kind(fighter.module_accessor);
        if [*FIGHTER_STATUS_KIND_SPECIAL_S,
            *FIGHTER_DEMON_STATUS_KIND_SPECIAL_S_AIR_END,
            *FIGHTER_DEMON_STATUS_KIND_SPECIAL_S_END,
            *FIGHTER_DEMON_STATUS_KIND_SPECIAL_S_HIT,
            *FIGHTER_DEMON_STATUS_KIND_SPECIAL_S_LANDING]
            .contains(&status)
        {
            // we're in side b
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
            {
                // we're holding jump
                StatusModule::change_status_request_from_script(fighter.module_accessor,
                    *FIGHTER_STATUS_KIND_JUMP, true);
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        kaz_jc_sideb
    );
}