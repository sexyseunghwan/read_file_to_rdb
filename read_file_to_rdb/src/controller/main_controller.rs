use crate::common::*;

use crate::service::query_service::*;

use crate::utils_module::io_utils::*;
use crate::utils_module::time_utils::*;

use crate::model::auto_search_keyword_tbl::*;

use crate::entity::auto_search_keyword_tbl;

#[derive(Debug, new)]
pub struct MainController<Q: QueryService + Sync + Send + 'static> {
    query_service: Arc<Q>
}

impl<Q: QueryService + Sync + Send + 'static> MainController<Q> {

    #[doc = "메인함수"]
    pub async fn main_task(&self) -> Result<(), anyhow::Error> {

        /* 현재시간 - utc 시간 */
        let cur_time: NaiveDateTime = get_current_utc_naive_datetime();

        /* 파일 읽기 */
        let models: Vec<String> = read_line_to_vec("./textfiles/indexing2.txt")?;
        
        let auto_keywords: Vec<auto_search_keyword_tbl::Model> = models
            .into_iter()
            .map(|keyword| 
                auto_search_keyword_tbl::Model {
                    keyword: keyword,
                    keyword_weight: 0,
                    reg_dt: cur_time,
                    chg_dt: Some(cur_time),
                    reg_id: String::from("system"),
                    chg_id: Some(String::from("system"))
                }
            )
            .collect();
        
        // self.query_service
        //     .bulk_insert_ignore_errors::<
        //         auto_search_keyword_tbl::Model, 
        //         auto_search_keyword_tbl::ActiveModel, 
        //         auto_search_keyword_tbl::Entity>
        //     (auto_keywords).await?;

        
        //let arc_query_service: Arc<Q> = Arc::new(self.query_service);
        //let query_service_clone = Arc::clone(&self)

        let chunked: Vec<Vec<_>> = auto_keywords
            .chunks((auto_keywords.len() + 3) / 4) // 최대 5등분
            .map(|chunk| chunk.to_vec())
            .collect();
        
        let mut tasks = Vec::new();
        
        for chunk in chunked {
            let service = self.query_service.clone(); // Arc 필요
            let handle = tokio::task::spawn(async move {
                service
                    .bulk_insert_ignore_errors::<
                        auto_search_keyword_tbl::Model,
                        auto_search_keyword_tbl::ActiveModel,
                        auto_search_keyword_tbl::Entity,
                    >(chunk)
                    .await
            });
            tasks.push(handle);
        }

        futures::future::try_join_all(tasks).await?;


        Ok(())
    }

}