use actix_web::{ HttpResponse };

pub async fn welcome_banner() -> Result<HttpResponse, HttpResponse> {
    let art = r###"


    `7MMF'  `7MMF' .g8""8q.         .g8"""bgd   .g8""8q. `7MM"""Mq.  `7MM"""YMM
    MM      MM .dP'    `YM.     .dP'     `M .dP'    `YM. MM   `MM.   MM    `7
    MM      MM dM'      `MM     dM'       ` dM'      `MM MM   ,M9    MM   d
    MMmmmmmmMM MM        MM     MM          MM        MM MMmmdM9     MMmmMM
    MM      MM MM.      ,MP     MM.         MM.      ,MP MM  YM.     MM   Y  ,
    MM      MM `Mb.    ,dP'     `Mb.     ,' `Mb.    ,dP' MM   `Mb.   MM     ,M
  .JMML.  .JMML. `"bmmd"'         `"bmmmd'    `"bmmd"' .JMML. .JMM..JMMmmmmMMM
                     MMb
                      `bood'
                                                            Welcome to HQ Core
"###;
    Ok(HttpResponse::Ok().content_type("text/plain").body(format!("{}", art)))
}
