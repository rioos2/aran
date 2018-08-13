// Copyright 2018 The Rio Advancement Inc

use db::data_store::DataStoreConn;
use models::invitations::DataStore;
use super::InvitationsOutputList;
use protocol::api::invitations::InvitationsList;

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

    /*pub fn build_event(&self, ini: &Invitations) -> InvitationsOutput {
    	let mut audits = AuditEvent::new();
        let m = audits.mut_meta(
                ObjectMeta::new(),
                "AUDITS".to_string(),
                ini.get_account_id(),
        );
        let jackie = ini.children();
        audits.set_meta(type_meta_url(jackie), m);
        audits.set_message(INVITATION);
        audits.set_message("hai");
        
        push_notification!(req, audits);
    }*/

}