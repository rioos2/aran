// Copyright 2018 The Rio Advancement Inc

use db::data_store::DataStoreConn;
use models::invitations::DataStore;
use super::InvitationsOutputList;
use protocol::api::invitations::InvitationsList;
use protocol::api::invitations::Invitations;
use protocol::api::audit::AuditEvent;
use protocol::api::schema::type_meta_url;
use protocol::api::base::{MetaFields, ObjectMeta};
use std::collections::BTreeMap;
use protocol::api::base::ChildTypeMeta;
use protocol::api::authorize::Teams;

/// This struct used for insert new entry into invitations table
/// And also sends email/slack invitations to users
pub struct Invites<'a> {
    conn: &'a DataStoreConn,
}

impl<'a> Invites<'a> { 

	pub fn new(conn: &'a DataStoreConn) -> Invites<'a> {
        Invites {
            conn: conn,
        }
    }
   
    /// To store invitation details into postgres
    /// And create EVENT for sends email/slack invitations
    pub fn mk_invites(&self, invitationsList: &InvitationsList) -> InvitationsOutputList {
    	let invitation_list = invitationsList.clone();
    	let mk_invites = invitation_list.invites
                        .into_iter()
                        .map(|invite| {
                            DataStore::create(&self.conn, &invite)
                        })
                        .fold(vec![], |mut acc, x| match x {
                            Ok(one_invite) => {                            	
                            	match one_invite {
                            		Some(res) => {
                            			acc.push(res);
                               			acc
                            		}
                            		None => acc,
                            	}
                                
                            }
                            Err(_) => acc,
                        });
        Ok(Some(mk_invites))
    }

    pub fn build_event(&self, ini: &Invitations, team: &Teams) -> AuditEvent {
    	let mut audits = AuditEvent::new();
        let mut labels = BTreeMap::new();
        let url = format!("{}/{}", "https://console.rioos.xyz/api/v1", team.get_id());
        let mut m = audits.mut_meta(
                ObjectMeta::new(),
                "AUDITS".to_string(),
                ini.get_invite_from(),
        );
        labels.insert("email".to_string(), ini.get_invite_to());
        labels.insert("invite_from".to_string(), ini.get_invite_from());
        labels.insert("origin".to_string(), ini.get_origin_id());
        labels.insert("team".to_string(), team.get_name());
        labels.insert("url".to_string(), url);
        audits.set_labels(&mut m, labels);
        let jackie = ini.children();
        audits.set_meta(type_meta_url(jackie), m);
        audits.set_reason("Invite".to_string());
        audits.set_message("hai".to_string());
        audits        
    }

}