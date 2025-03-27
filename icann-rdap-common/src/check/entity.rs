use std::{any::TypeId, str::FromStr};

use crate::{
    contact::Contact,
    response::entity::{Entity, EntityRole},
};

use super::{
    string::{StringCheck, StringListCheck},
    Check, CheckParams, Checks, GetChecks, GetSubChecks, RdapStructure,
};

impl GetChecks for Entity {
    fn get_checks(&self, params: CheckParams) -> super::Checks {
        let sub_checks = if params.do_subchecks {
            let mut sub_checks: Vec<Checks> = self
                .common
                .get_sub_checks(params.from_parent(TypeId::of::<Self>()));
            sub_checks.append(
                &mut self
                    .object_common
                    .get_sub_checks(params.from_parent(TypeId::of::<Self>())),
            );
            if let Some(public_ids) = &self.public_ids {
                sub_checks.append(&mut public_ids.get_sub_checks(params));
            }
            sub_checks
        } else {
            vec![]
        };

        let mut items = vec![];

        if let Some(roles) = &self.roles {
            if roles.is_string() {
                items.push(Check::RoleIsString.check_item());
            }
            let roles = roles.vec();
            if roles.is_empty_or_any_empty_or_whitespace() {
                items.push(Check::RoleIsEmpty.check_item());
            } else {
                for role in roles {
                    let r = EntityRole::from_str(role);
                    if r.is_err() {
                        items.push(Check::UnknownRole.check_item());
                    }
                }
            }
        }

        if let Some(vcard) = &self.vcard_array {
            if let Some(contact) = Contact::from_vcard(vcard) {
                if let Some(full_name) = contact.full_name {
                    if full_name.is_whitespace_or_empty() {
                        items.push(Check::VcardFnIsEmpty.check_item())
                    }
                } else {
                    items.push(Check::VcardHasNoFn.check_item())
                }
            } else {
                items.push(Check::VcardArrayIsEmpty.check_item())
            }
        }

        Checks {
            rdap_struct: RdapStructure::Entity,
            items,
            sub_checks,
        }
    }
}
