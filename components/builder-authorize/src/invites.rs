// Copyright 2018 The Rio Advancement Inc

use db::data_store::DataStoreConn;
use db::error::Error::RecordsNotFound;
use error::Error;
use models::{invitations, team_members};
use super::{InvitationsOutputList, TeamMembersOutput};
use protocol::api::invitations::InvitationsList;
use protocol::api::invitations::Invitations;
use protocol::api::audit::AuditEvent;
use protocol::api::schema::type_meta_url;
use protocol::api::base::{MetaFields, ObjectMeta};
use std::collections::BTreeMap;
use protocol::api::base::ChildTypeMeta;
use protocol::api::authorize::{Teams, TeamMembers};
use protocol::api::origin::OriginMembers;
use protocol::api::session;
use session::models::{session as sessions, origin_members};
use protocol::api::base::IdGet;

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
    pub fn mk_invites(&self, invitations_list: &InvitationsList) -> InvitationsOutputList {
    	let invitation_list = invitations_list.clone();
    	let mk_invites = invitation_list.invites
                        .into_iter()
                        .map(|invite| {
                            invitations::DataStore::create(&self.conn, &invite)
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

    pub fn mk_member(&self, invitations: &Invitations, ini_get: &IdGet) -> TeamMembersOutput {
        let mut account_get = session::AccountGet::new();
        account_get.set_email(invitations.get_invite_to());
        match sessions::DataStore::get_account(&self.conn, &account_get) {
            Ok(Some(opt_account)) => {
                    let team_member = self.build_team_member(opt_account.get_id(), invitations);
                    let origin_member = self.build_origin_member(opt_account.get_id(), invitations);
                    //update accept status into invitations table
                    invitations::DataStore::update_status(&self.conn, &ini_get)?;
                    //insert new origin member entry into origin member table
                    //now the user merged to invited origin
                    let _origin_member = origin_members::DataStore::new(&self.conn).create(&origin_member); //we must track this error..
                    //insert new team member entry into team member table
                    //now the user merged to invited team
                    team_members::DataStore::new(&self.conn).create(&team_member)
            }
            Err(_err) => Err(Error::Db(RecordsNotFound)), //In future  we track this errors
            Ok(None) => Err(Error::AccountNotFound(
                    "Invited Account not found, Please sign up your account.".to_string(),
                )),
        }
    }

    // build audit data for send email invitations
    pub fn build_event(&self, originated_url: String, ini: &Invitations, team: &Teams, account_name: String) -> AuditEvent {
    	let mut audits = AuditEvent::new();
        let mut labels = BTreeMap::new();
        let url = format!("{}/invitations/{}/accept", originated_url, ini.get_id());
        let mut m = audits.mut_meta(
                ObjectMeta::new(),
                "AUDITS".to_string(),
                ini.get_invite_from(),
        );
        labels.insert("email".to_string(), ini.get_invite_to());
        labels.insert("invite_from".to_string(), account_name);
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

    // build team member data for create new entry 
    pub fn build_team_member(&self, account_id: String, ini: &Invitations) -> TeamMembers {
        let mut members = TeamMembers::new();
        let m = members.mut_meta(
                ObjectMeta::new(),
                "teammembers".to_string(),
                account_id,
        );        
        //it is temporary fix 
        let jackie = "POST:teammembers".to_string();
        members.set_meta(type_meta_url(jackie), m);
        let mut metadata = BTreeMap::new();
        metadata.insert("team".to_string(), ini.get_team_id());
        metadata.insert("origin".to_string(), ini.get_origin_id());
        members.set_metadata(metadata);
        members        
    }

    // build origin member data for create new entry 
    pub fn build_origin_member(&self, account_id: String, ini: &Invitations) -> OriginMembers {
        let mut members = OriginMembers::new();
        let m = members.mut_meta(
                ObjectMeta::new(),
                ini.get_origin_id(),
                account_id,
        );        
        //it is temporary fix 
        let jackie = "POST:originmembers".to_string();
        members.set_meta(type_meta_url(jackie), m);
        let mut metadata = BTreeMap::new();
        metadata.insert("origin".to_string(), ini.get_origin_id());
        members.set_metadata(metadata);
        members        
    }
  

}