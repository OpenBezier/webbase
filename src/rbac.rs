use std::sync::Arc;
use std::sync::OnceLock;
use tokio::sync::RwLock;

static RBAC: OnceLock<RbacCache> = OnceLock::<RbacCache>::new();

pub struct RbacCache {
    pub rbac: Arc<RwLock<RpConfig>>,
}

pub fn init_rbac(rpconfig: RpConfig) -> &'static RbacCache {
    RBAC.get_or_init(|| RbacCache {
        rbac: Arc::new(RwLock::new(rpconfig)),
    })
}

pub fn get_rbac() -> &'static RbacCache {
    RBAC.get().unwrap()
}

pub fn update_rbac_config() {
    tokio::spawn(async {
        loop {
            let perm = get_rbac();
            let mut rbac = perm.rbac.write().await;
            *rbac = RpConfig::default();
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
    });
}

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::HashMap;

pub type UserPermission =
    HashMap<String, BTreeMap<String, (bool, Option<Vec<(String, String)>>, Option<String>)>>;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Permission {
    pub role: Vec<String>,
    pub comment: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Role {
    Member(Vec<String>),
    Group(HashMap<String, Vec<String>>),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct RpConfig {
    pub name: String,
    pub role: HashMap<String, Role>,
    pub permission: HashMap<String, HashMap<String, Permission>>,
}

/// return: <page, <action, (has_permission_or_not, require_group_info_list, comment_info)>>
impl RpConfig {
    pub fn get_user_permission(&self, account: String, get_all_actions: bool) -> UserPermission {
        let mut result = HashMap::<
            String,
            BTreeMap<String, (bool, Option<Vec<(String, String)>>, Option<String>)>,
        >::default();

        for (each_page, page_map) in self.permission.iter() {
            let mut page_result =
                BTreeMap::<String, (bool, Option<Vec<(String, String)>>, Option<String>)>::default(
                );
            'action: for (each_action, action_permission) in page_map.iter() {
                // if Member role list already have this user, all is done, don't need check Group role map
                for each_role in action_permission.role.iter() {
                    if each_role.eq("default") {
                        page_result.insert(
                            each_action.clone(),
                            (true, None, action_permission.comment.clone()),
                        );
                        continue 'action;
                    } else {
                        if let Some(role_users) = self.role.get(each_role) {
                            match role_users {
                                Role::Member(user_list) => {
                                    if user_list.contains(&account) {
                                        page_result.insert(
                                            each_action.clone(),
                                            (true, None, action_permission.comment.clone()),
                                        );
                                        continue 'action;
                                    }
                                }
                                Role::Group(_) => {}
                            }
                        }
                    }
                }

                let mut group_require = Vec::<(String, String)>::default();
                let mut group_check = false;
                for each_role in action_permission.role.iter() {
                    if let Some(role_users) = self.role.get(each_role) {
                        match role_users {
                            Role::Member(_) => {}
                            Role::Group(user_map) => {
                                for (require, user_list) in user_map.iter() {
                                    if user_list.contains(&account) {
                                        group_require.push((require.clone(), each_role.clone()));
                                        group_check = true;
                                    }
                                }
                            }
                        }
                    }
                }

                if group_check {
                    page_result.insert(
                        each_action.clone(),
                        (true, Some(group_require), action_permission.comment.clone()),
                    );
                } else {
                    if get_all_actions {
                        page_result.insert(
                            each_action.clone(),
                            (false, None, action_permission.comment.clone()),
                        );
                    }
                }
            }
            result.insert(each_page.clone(), page_result);
        }
        result
    }

    pub fn check_user_action(
        &self,
        account: String,
        page: String,
        action: String,
    ) -> (bool, Option<Vec<(String, String)>>) {
        if let Some(page_group) = self.permission.get(&page) {
            if let Some(action_to) = page_group.get(&action) {
                // Member role list already have , don't need check Group role map
                for each_role in action_to.role.iter() {
                    if each_role.eq("default") {
                        return (true, None);
                    } else {
                        if let Some(role_users) = self.role.get(each_role) {
                            match role_users {
                                Role::Member(user_list) => {
                                    if user_list.contains(&account) {
                                        return (true, None);
                                    }
                                }
                                Role::Group(_) => {}
                            }
                        }
                    }
                }

                // not in Member role list, then check Group role map and if match, return Require condition
                let mut group_require = Vec::<(String, String)>::default();
                let mut group_check = false;
                for each_role in action_to.role.iter() {
                    if let Some(role_users) = self.role.get(each_role) {
                        match role_users {
                            Role::Member(_) => {}
                            Role::Group(user_map) => {
                                for (require, user_list) in user_map.iter() {
                                    if user_list.contains(&account) {
                                        group_require.push((require.clone(), each_role.clone()));
                                        group_check = true;
                                    }
                                }
                            }
                        }
                    }
                }
                if group_check {
                    return (true, Some(group_require));
                }
            }
        }
        (false, None)
    }
}