use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "People")]
    pub people: People,
    #[serde(rename = "Headline")]
    pub headline: Option<String>,
    #[serde(rename = "Source")]
    pub source: Vec<Source>,
    #[serde(rename = "Original Site")]
    pub original_site: Option<String>,
    #[serde(rename = "Time")]
    pub time: Option<String>,
    #[serde(rename = "Section")]
    pub section: Option<String>,
    #[serde(rename = "Keywords")]
    pub keywords: Option<String>,
    #[serde(rename = "Abstract")]
    pub abstract_field: Option<String>,
    #[serde(rename = "Topic")]
    pub topic: Option<String>,
    #[serde(rename = "Body")]
    pub body: Option<String>,
    #[serde(rename = "Notes")]
    pub notes: Option<String>,
    #[serde(rename = "Further Information")]
    pub further_information: Option<String>,
    #[serde(rename = "Label")]
    pub label: Option<String>,
    #[serde(rename = "Peo_State")]
    pub peo_state: Option<String>,
    #[serde(rename = "Org_State")]
    pub org_state: Option<String>,
    #[serde(rename = "Ori_State")]
    pub ori_state: Option<String>,
    #[serde(rename = "By_State")]
    pub by_state: Option<String>,
    #[serde(rename = "Peo_Expression_State")]
    pub peo_expression_state: Option<String>,
    #[serde(rename = "Headline_Classification_State")]
    pub headline_classification_state: Option<String>,
    #[serde(rename = "Keywords_State")]
    pub keywords_state: Option<String>,
    #[serde(rename = "Abstract_State")]
    pub abstract_state: Option<String>,
    #[serde(rename = "Topic_Sentence_State")]
    pub topic_sentence_state: Option<String>,
    #[serde(rename = "Topic_Sentence_Classification_State")]
    pub topic_sentence_classification_state: Option<String>,
    #[serde(rename = "Expression_Classification_State")]
    pub expression_classification_state: Option<String>,
    #[serde(rename = "Update_Time")]
    pub update_time: Option<String>,
    #[serde(rename = "Media_Id")]
    pub media_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct People {
    #[serde(rename = "Geography")]
    pub geography: Option<String>,
    #[serde(rename = "From_Tibet")]
    pub from_tibet: Option<String>,
    #[serde(rename = "Identity_Entertainment")]
    pub identity_entertainment: Option<String>,
    #[serde(rename = "Identity_Refugee")]
    pub identity_refugee: Option<String>,
    #[serde(rename = "From_Uyghur")]
    pub from_uyghur: Option<String>,
    #[serde(rename = "Twitter")]
    pub twitter: Option<String>,
    #[serde(rename = "Youtube_Url")]
    pub youtube_url: Option<String>,
    #[serde(rename = "Information")]
    pub information: Option<String>,
    #[serde(rename = "Identity_Crime")]
    pub identity_crime: Option<String>,
    #[serde(rename = "Fb_Shot")]
    pub fb_shot: Option<String>,
    #[serde(rename = "From_Congressman_State")]
    pub from_congressman_state: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "From_Media")]
    pub from_media: Option<String>,
    #[serde(rename = "Twitter_Acc")]
    pub twitter_acc: Option<String>,
    #[serde(rename = "From_Congressman_District")]
    pub from_congressman_district: Option<String>,
    #[serde(rename = "Name_Clean")]
    pub name_clean: Option<String>,
    #[serde(rename = "From_Expert")]
    pub from_expert: Option<String>,
    #[serde(rename = "From_Congressman")]
    pub from_congressman: Option<String>,
    #[serde(rename = "From_Congressman_Period")]
    pub from_congressman_period: Option<String>,
    #[serde(rename = "Identity")]
    pub identity: Option<String>,
    #[serde(rename = "Identity_Military")]
    pub identity_military: Option<String>,
    #[serde(rename = "Fb")]
    pub fb: Option<String>,
    #[serde(rename = "Orob_Region")]
    pub orob_region: Option<String>,
    #[serde(rename = "Identity_Business")]
    pub identity_business: Option<String>,
    #[serde(rename = "Identity_Expert")]
    pub identity_expert: Option<String>,
    #[serde(rename = "Geopolitics")]
    pub geopolitics: Option<String>,
    #[serde(rename = "Identity_Media")]
    pub identity_media: Option<String>,
    #[serde(rename = "Identity_Religion")]
    pub identity_religion: Option<String>,
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "From_Congressman_Party")]
    pub from_congressman_party: Option<String>,
    #[serde(rename = "Identity_Activist")]
    pub identity_activist: Option<String>,
    #[serde(rename = "From_Topic")]
    pub from_topic: Option<String>,
    #[serde(rename = "Orob")]
    pub orob: Option<String>,
    #[serde(rename = "Identity_Politician")]
    pub identity_politician: Option<String>,
    #[serde(rename = "Expression")]
    pub expression: Option<String>,
    #[serde(rename = "Identity_Judge")]
    pub identity_judge: Option<String>,
    #[serde(rename = "Identity_Student")]
    pub identity_student: Option<String>,
    #[serde(rename = "Country")]
    pub country: Option<String>,
    #[serde(rename = "Identity_Terrorist")]
    pub identity_terrorist: Option<String>,
    #[serde(rename = "Identity_Sports")]
    pub identity_sports: Option<String>,
    #[serde(rename = "Identity_Lawyer")]
    pub identity_lawyer: Option<String>,
    #[serde(rename = "Opinion")]
    pub opinion: Vec<Opinion>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Opinion {
    pub score: f64,
    pub start: i64,
    pub end: i64,
    pub text: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    #[serde(rename = "Geography")]
    pub geography: Option<String>,
    #[serde(rename = "From_Organization")]
    pub from_organization: Option<String>,
    #[serde(rename = "From_Department_Country")]
    pub from_department_country: Option<String>,
    #[serde(rename = "From_Facebook")]
    pub from_facebook: Option<String>,
    #[serde(rename = "From_Twitter")]
    pub from_twitter: Option<String>,
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "From_Web")]
    pub from_web: Option<String>,
    #[serde(rename = "From_Representatives")]
    pub from_representatives: Option<String>,
    #[serde(rename = "From_Embassy_Located")]
    pub from_embassy_located: Option<String>,
    #[serde(rename = "From_Bank")]
    pub from_bank: Option<String>,
    #[serde(rename = "From_University_Region")]
    pub from_university_region: Option<String>,
    #[serde(rename = "Orob_Region")]
    pub orob_region: Option<String>,
    #[serde(rename = "From_Senator_Region")]
    pub from_senator_region: Option<String>,
    #[serde(rename = "Focus_Department")]
    pub focus_department: Option<String>,
    #[serde(rename = "From_Representatives_Region")]
    pub from_representatives_region: Option<String>,
    #[serde(rename = "Geopolitics")]
    pub geopolitics: Option<String>,
    #[serde(rename = "From_Embassy_Country")]
    pub from_embassy_country: Option<String>,
    #[serde(rename = "From_Institution")]
    pub from_institution: Option<String>,
    #[serde(rename = "Official_Site")]
    pub official_site: Option<String>,
    #[serde(rename = "From_University")]
    pub from_university: Option<String>,
    #[serde(rename = "From_Senator")]
    pub from_senator: Option<String>,
    #[serde(rename = "From_News_Agency")]
    pub from_news_agency: Option<String>,
    #[serde(rename = "From_Journal")]
    pub from_journal: Option<String>,
    #[serde(rename = "Orob")]
    pub orob: Option<String>,
    #[serde(rename = "From_Department")]
    pub from_department: Option<String>,
    #[serde(rename = "Country")]
    pub country: Option<String>,
    #[serde(rename = "From_Blog")]
    pub from_blog: Option<String>,
    #[serde(rename = "From_University_News")]
    pub from_university_news: Option<String>,
}

impl People {
    pub fn get_identity(&self) -> Option<String> {
        let mut valid: Vec<&str> = vec![];
        if let Some(s) = &self.identity_entertainment {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.identity_refugee {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.identity_crime {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.identity_military {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.identity_business {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.identity_expert {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.identity_media {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.identity_religion {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.identity_activist {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.identity_politician {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.identity_judge {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.identity_student {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.identity_terrorist {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.identity_sports {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.identity_lawyer {
            valid.push(s.as_str())
        };
        if valid.is_empty() {
            None
        } else {
            Some(valid.join(","))
        }
    }

    pub fn get_from(&self) -> Option<String> {
        let mut valid = vec![];
        if let Some(s) = &self.from_tibet {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_uyghur {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_congressman_state {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_media {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_congressman_district {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_expert {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_congressman {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_congressman_period {
            if s != "None" {
                valid.push(s.as_str())
            }
        };
        if let Some(s) = &self.from_congressman_party {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_topic {
            valid.push(s.as_str())
        };
        if valid.is_empty() {
            None
        } else {
            Some(valid.join(","))
        }
    }
}

impl Source {
    pub(crate) fn get_from(&self) -> Option<String> {
        let mut valid: Vec<&str> = vec![];
        if let Some(s) = &self.from_blog {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_bank {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_department {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_journal {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_institution {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_senator {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_department_country {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_embassy_country {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_embassy_located {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_facebook {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_news_agency {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_organization {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_representatives {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_representatives_region {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_senator_region {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_twitter {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_university {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_university_region {
            valid.push(s.as_str())
        };
        if let Some(s) = &self.from_web {
            valid.push(s.as_str())
        };
        if valid.is_empty() {
            None
        } else {
            Some(valid.join(","))
        }
    }
}
