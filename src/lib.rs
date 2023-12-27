#![feature(lazy_cell, ptr_sub_ptr)]
use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, person::*, unit::*};
use engage::{sequence::*, gamevariable::*, gameuserdata::*};
use engage::gamedata::*;
use skyline::patching::Patch;
use engage::force::Force;

#[unity::from_offset("App", "PersonData", "IsHero")]
pub fn IsHero(this: &PersonData, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x025c9240)]
pub fn Mess_Get(label: &Il2CppString, method_info: OptionalMethod) -> &Il2CppString;

#[unity::from_offset("App", "UnitEdit", "SetName")]
pub fn SetName(this: &UnitEdit, name: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x02616200)]
pub fn Force_Get(forceType:  i32, method_info: OptionalMethod) -> &'static Force;

//Ignore Mauvier for Reverse Recruitment
#[skyline::hook(offset=0x01cd6020)]
pub fn ignoreMauvierLevel(this: u64, unit: &Unit, method_info: OptionalMethod){
    unsafe {
        let pid = unit_get_pid(unit, None);
        if pid.get_string().unwrap() == "PID_モーヴ" { return;  }
        else { call_original!(this, unit, method_info); }
    }
}
#[skyline::from_offset(0x01f73e50)]
pub fn set_gender(this: &UnitEdit, gender: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "UnitEdit", "CopyFrom")]
pub fn UnitEdit_CopyFrom(this: &UnitEdit, src: &UnitEdit, method_info: OptionalMethod);

//Set Veyle's name 
#[skyline::hook(offset=0x01bee0a0)]
pub fn loading_screen(this: u64, method_info: OptionalMethod){
    call_original!(this, method_info);
    unsafe {
        for f in 0..7 {
            let force = Force_Get(f, None);
            let mut force_iter = Force::iter(force);
            while let Some(unit) = force_iter.next() {
                if IsHero(unit.person, None){
                    let newName = Mess_Get(unit.person.name, None);
                    set_gender(unit.edit, 2, None);    
                    SetName(unit.edit, newName, None);
                }
            }
        }
    }
}
//Check if Alear is dead
pub fn isLueurDead() -> bool {
    unsafe {
        let deadForce = Force_Get(5, None);
        let mut force_iter = Force::iter(deadForce);
        while let Some(unit2) = force_iter.next() {
            if unit2.person.pid.get_string().unwrap() == "PID_リュール" {
                return true;
            }
        }
    }
    false
}
#[skyline::hook(offset=0x01a08b60)]
pub fn create_unit_hook(unit: &Unit, method_info: OptionalMethod) {
    call_original!(unit, method_info);
    unsafe {
        if IsHero(unit.person, None){
            let newName = Mess_Get(unit.person.name, None);
            set_gender(unit.edit, 2, None);    
            SetName(unit.edit, newName, None);
        }
        if isLueurDead() && unit.person.name.get_string().unwrap() == "MPID_Lueur" {
            let deadForce = Force_Get(5, None);
            let mut force_iter = Force::iter(deadForce);
            while let Some(unit2) = force_iter.next() {
                if unit2.person.pid.get_string().unwrap() == "PID_リュール" {
                    UnitEdit_CopyFrom(unit.edit, unit2.edit, None);
                }
            }
        }
    }
}
#[skyline::main(name = "rr")]
pub fn main() { skyline::install_hooks!(ignoreMauvierLevel, create_unit_hook, loading_screen); }



