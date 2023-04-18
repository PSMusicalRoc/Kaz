use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash::app::sv_animcmd::*;
use smash::phx::Hash40;

#[fighter_frame( agent = FIGHTER_KIND_DEMON )]
fn ez_input_fn(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = fighter.module_accessor;
        let status = StatusModule::status_kind(fighter.module_accessor);
        if [
            *FIGHTER_STATUS_KIND_GUARD,
            *FIGHTER_STATUS_KIND_GUARD_ON,
            *FIGHTER_STATUS_KIND_GUARD_OFF,
            *FIGHTER_STATUS_KIND_STANDBY
        ].contains(&status)
        {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
            && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
            {
                let dir_x = PostureModule::lr(boma);
                let fw_dir: i32;
                if dir_x == 1.0
                {
                    fw_dir = 1;
                }
                else {
                    fw_dir = 255;
                }

                if ControlModule::get_flick_x(boma) < 5
                    && ControlModule::get_flick_x_dir(boma) == fw_dir
                {
                    StatusModule::change_status_request(boma,
                        *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2, true);
                }
                else if ControlModule::get_flick_x(boma) < 5
                    && ControlModule::get_flick_x_dir(boma) != fw_dir
                {
                    StatusModule::change_status_request(boma,
                        *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2S, true);
                }
                else if ControlModule::get_flick_y(boma) < 5
                    && ControlModule::get_flick_y_dir(boma) == 1
                {
                    StatusModule::change_status_request(boma,
                        *FIGHTER_DEMON_STATUS_KIND_ATTACK_STAND_1, true);
                }
            }
        }
    }
}

pub fn install()
{
    smashline::install_agent_frames!(
        ez_input_fn
    );
}