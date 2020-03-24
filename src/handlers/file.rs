// async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
//     while let Some(item) = payload.next().await {
//         let mut field = item?;
//         let content_type = field
//             .content_disposition()
//             .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
//         let filename = content_type
//             .get_filename()
//             .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
//         let filepath = format!("./tmp/{}", filename);
//         let mut f = async_std::fs::File::create(filepath).await?;

//         // Field in turn is stream of *Bytes* object
//         while let Some(chunk) = field.next().await {
//             let data = chunk.unwrap();
//             f.write_all(&data).await?;
//         }
//     }
//     Ok(HttpResponse::Ok().into())
// }
