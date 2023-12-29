#![feature(lazy_cell, ptr_sub_ptr)]
use unity::prelude::*;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};
use engage::gamedata::{*, person::*, unit::*};
use engage::{sequence::*, gamevariable::*, gameuserdata::*};
use engage::gamedata::*;
use skyline::patching::Patch;
use engage::force::Force;
static mut LUEUR_IGNORED: bool = false;

#[skyline::from_offset(0x1f261a0)]
pub fn get_attrs(this: &PersonData, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x1f261b0)]
pub fn set_attrs(this: &PersonData, value: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x1f25db0)]
pub fn set_gender_person(this: &PersonData, value: i32, method_info: OptionalMethod);

#[unity::hook("App", "PersonData", "IsHero")]
pub fn IsHero(this: &PersonData, method_info: OptionalMethod) -> bool {
    unsafe {
        let lueur = isLueur_Recruited();
        if lueur.is_some() && !LUEUR_IGNORED {
            let unit = lueur.unwrap();
            if unit.status.value & 8388608 == 8388608 && unit.m_GodLink.is_some() {
                if unit.m_GodLink.unwrap().m_Data.mid.get_string().unwrap() == "MGID_Lueur" {
                    return this.pid.get_string().unwrap() == "PID_リュール";
                }
            }
        }
    }
    call_original!(this, method_info)
}

#[skyline::from_offset(0x025c9240)]
pub fn Mess_Get(label: &Il2CppString, method_info: OptionalMethod) -> &Il2CppString;

#[skyline::from_offset(0x025d4410)]
pub fn Mess_Get2(label: &Il2CppString, arg0: &Il2CppString, method_info: OptionalMethod) -> &'static Il2CppString;

#[unity::from_offset("App", "UnitEdit", "SetName")]
pub fn SetName(this: &UnitEdit, name: &Il2CppString, method_info: OptionalMethod);

#[unity::from_offset("App", "Unit", "GetName")]
pub fn Unit_GetName(this: &Unit, method_info: OptionalMethod) -> &Il2CppString;

#[skyline::from_offset(0x02616200)]
pub fn Force_Get(forceType:  i32, method_info: OptionalMethod) -> &'static Force;

#[unity::from_offset("App", "InfoUtil", "TrySetText")]
pub fn TrySetText(tmp: &u64, str: &Il2CppString, method_info: OptionalMethod);

#[unity::class("App", "GodUnit_Flags")]
pub struct GodData_Flags {
    pub value: u32,
}

#[unity::class("App", "UnitStatusSetter")]
pub struct StatusSetter {
    junk : [u8; 184],
    pub m_bond: &'static u64,
}

#[unity::from_offset("App", "GodData", "get_Flag")]
pub fn GodData_get_flag(this: &GodData, method_info: OptionalMethod) -> &'static mut GodData_Flags;

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

//Set Veyle's and Alear name 
#[skyline::hook(offset=0x01bee0a0)]
pub fn loading_screen(this: u64, method_info: OptionalMethod){
    call_original!(this, method_info);
    unsafe {
        for f in 0..7 {
            if f == 5 { continue; }
            let force = Force_Get(f, None);
            let mut force_iter = Force::iter(force);
            while let Some(unit) = force_iter.next() {
                if unit.person.pid.get_string().unwrap() == "PID_ヴェイル" {
                    let newName = Mess_Get(unit.person.name, None);
                    set_gender(unit.edit, 2, None);    
                    SetName(unit.edit, newName, None);
                }
                if unit.person.pid.get_string().unwrap() == "PID_リュール" {
                    let lueur = isLueurDead();
                    if lueur.is_some() {  
                        UnitEdit_CopyFrom(unit.edit, lueur.unwrap().edit, None); 
                    }
                }
            }
        }
        let lueur = isLueurDead();
        if lueur.is_some() {
            let gender = lueur.unwrap().edit.gender;
            let triabolical = PersonData::get_list_mut().expect("triabolical is 'None'");
            let t_list = &triabolical.list.items;
            for x in 1..800 {
                if t_list[x].name.get_string().unwrap() == "MPID_Lueur" {
                    set_gender_person(t_list[x], gender, None);
                }
            }
        }
    }
}
//Check if Alear is dead
pub fn isLueurDead() -> Option<&'static Unit> {
    unsafe {
        let deadForce = Force_Get(5, None);
        let mut force_iter = Force::iter(deadForce);
        while let Some(unit2) = force_iter.next() {
            if unit2.person.pid.get_string().unwrap() == "PID_リュール" {
                return Some(unit2);
            }
        }
    }
    return None;
}
pub fn isLueur_Recruited() -> Option<&'static Unit> {
    unsafe {
        for f in 0..7 {
            if f == 5 { continue; }
            let deadForce = Force_Get(f, None);
            let mut force_iter = Force::iter(deadForce);
            while let Some(unit2) = force_iter.next() {
                if unit2.person.pid.get_string().unwrap() == "PID_リュール" {
                    return Some(unit2);
                }
            }
        }

    }
    return None;
}
#[skyline::hook(offset=0x01a08b60)]
pub fn create_unit_hook(unit: &Unit, method_info: OptionalMethod) {
    call_original!(unit, method_info);
    unsafe {
        let lueur = isLueurDead();
        if lueur.is_some() && unit.person.name.get_string().unwrap() == "MPID_Lueur" {
            UnitEdit_CopyFrom(unit.edit, lueur.unwrap().edit, None);
        }
    }
}

#[skyline::from_offset(0x02c4ea10)]
pub fn Set_Active(this: &u64, value: bool, method_info: OptionalMethod);
#[skyline::from_offset(0x0290f7d0)]
pub fn TrySetActive(this: &u64, isValue: bool, method_info: OptionalMethod);

#[skyline::hook(offset=0x01f4bfd0)]
pub fn IsLoseUnitDead(this: u64, unit: u64, method_info: OptionalMethod) -> bool {
    unsafe {
        LUEUR_IGNORED = true;
        let result = call_original!(this, unit, method_info);
        LUEUR_IGNORED = false;
        return result;
    }
}
#[skyline::hook(offset=0x01a240c0)]
pub fn can_convoy(this: &Unit, method_info: OptionalMethod) -> bool  {
    unsafe {
        if this.person.pid.get_string().unwrap() == "PID_ヴェイル" { return true; }
        else if this.person.pid.get_string().unwrap() == "PID_リュール" {
            LUEUR_IGNORED = true;
            let result = call_original!(this, method_info);
            LUEUR_IGNORED = false;
            return result;
        }
    }
    call_original!(this, method_info)
}
#[skyline::hook(offset=0x02d51d80)]
pub fn get_face(this: &Unit, method_info: OptionalMethod) -> &Il2CppString {
    if this.person.pid.get_string().unwrap() == "PID_ヴェイル" {
        unsafe { return get_ascii_name(this.person, None);  }
    }
    else if this.person.pid.get_string().unwrap() == "PID_リュール" {
        if isLueurDead().is_some() {
            if isLueurDead().unwrap().edit.gender == 2 { return "LueurW".into(); }
            else { return "Lueur".into(); }
        }
    }
    call_original!(this, method_info)
}

#[skyline::hook(offset=0x02d52340)]
pub fn get_god_face(this: &GodData, method_info: OptionalMethod) -> &Il2CppString {
    if this.mid.get_string().unwrap() == "MGID_Lueur" {
        unsafe {
            if isLueurDead().is_some() {
                if isLueurDead().unwrap().edit.gender == 2 { return "LueurW_God".into(); }
                else { return "Lueur_God".into(); }
            }
            if isLueur_Recruited().is_some() {
                if isLueur_Recruited().unwrap().edit.gender == 2 { return "LueurW_God".into(); }
                else { return "Lueur_God".into(); }
            }
        }
    }
    call_original!(this, method_info)
}
#[skyline::hook(offset=0x0233f090)]
pub fn get_God_name(this: &GodData, method_info : OptionalMethod) -> &Il2CppString {
    if this.mid.get_string().unwrap() == "MGID_Lueur" {
        if isLueurDead().is_some() { return isLueurDead().unwrap().edit.name.unwrap(); }
        if isLueur_Recruited().is_some() { return isLueur_Recruited().unwrap().edit.name.unwrap(); }
    }
    call_original!(this, method_info)
}

#[skyline::hook(offset=0x021e1250)]
pub fn Get_Bond_Face(this: &Unit, method_info: OptionalMethod) -> &Il2CppString {
    unsafe {
        if this.person.pid.get_string().unwrap() == "PID_ヴェイル" {  return  "Telop/LevelUp/FaceThumb/Veyre".into(); }
        else if this.person.pid.get_string().unwrap() == "PID_リュール" {
            if isLueurDead().is_some() {
                if isLueurDead().unwrap().edit.gender == 2 { return "Telop/LevelUp/FaceThumb/LueurW".into();}
                else { return "Telop/LevelUp/FaceThumb/Lueur".into(); }
            }
            if isLueurDead().unwrap().edit.gender == 2 { return "Telop/LevelUp/FaceThumb/LueurW".into();}
            else { return "Telop/LevelUp/FaceThumb/Lueur".into(); }
        }
    }
    call_original!(this, method_info)
}

#[skyline::hook(offset=0x025d6930)]
pub fn mess_is_hero_female(method_info: OptionalMethod) -> bool {
    if isLueurDead().is_some() {  return isLueurDead().unwrap().edit.gender == 2; }
    if isLueur_Recruited().is_some() {  return isLueurDead().unwrap().edit.gender == 2; }
    call_original!(method_info)
}
#[skyline::main(name = "rr")]
pub fn main() { skyline::install_hooks!(Get_Bond_Face, get_God_name, get_god_face, get_face, IsHero, IsLoseUnitDead, create_unit_hook, loading_screen, can_convoy); }