use crate::common::*;

use crate::repository::mysql_repository::*;

#[async_trait]
pub trait QueryService {

    // async fn bulk_insert<T, A, E>(
    //     &self,
    //     data: Vec<T>
    // ) -> Result<(), anyhow::Error>
    // where
    //     T: IntoActiveModel<A>,
    //     A: ActiveModelTrait<Entity = E>,
    //     E: EntityTrait<Model = T, ActiveModel = A>;   


    async fn bulk_insert_ignore_errors<T, A, E>(
        &self,
        data: Vec<T>
    ) -> Result<(), anyhow::Error>
    where
        T: IntoActiveModel<A> + Send + Sync +'static,
        A: ActiveModelTrait<Entity = E>  + sea_orm::ActiveModelBehavior + Send + Sync +'static,
        E: EntityTrait<Model = T, ActiveModel = A> + Send + Sync +'static;      
}


#[derive(Debug, new)]
pub struct QueryServicePub;

#[async_trait]
impl QueryService for QueryServicePub {
    
    // #[doc = "여러개의 데이터를 특정 엔티티에 insert 해주는 함수"]
    // async fn bulk_insert<T, A, E>(
    //     &self,
    //     data: Vec<T>
    // ) -> Result<(), anyhow::Error>
    // where
    //     T: IntoActiveModel<A>,
    //     A: ActiveModelTrait<Entity = E>,
    //     E: EntityTrait<Model = T, ActiveModel = A> 
    // {
        
    //     let conn: &'static DatabaseConnection = establish_connection().await;

    //     let active_models: Vec<A> = data.into_iter().map(|d| d.into_active_model()).collect();
    //     E::insert_many(active_models).exec(conn).await?;
        
    //     Ok(())
    // }


    #[doc = "여러개의 데이터를 특정 엔티티에 하나하나씩 insert 해주는 함수"]
    async fn bulk_insert_ignore_errors<T, A, E>(
        &self,
        data: Vec<T>
    ) -> Result<(), anyhow::Error>
    where
        T: IntoActiveModel<A> + Send + Sync + 'static,
        A: ActiveModelTrait<Entity = E>  + sea_orm::ActiveModelBehavior + Send + Sync +'static,
        E: EntityTrait<Model = T, ActiveModel = A> + Send + Sync + 'static
    {
        let conn: &DatabaseConnection = establish_connection().await;
        
        for model in data {
            let active_model = model.into_active_model();

            let result = active_model.insert(conn).await;

            if let Err(ref e) = result {
                info!("[Insert Skipped] {:?}", e); // 로그로 남기기
            }
        }


        Ok(())

    }



}