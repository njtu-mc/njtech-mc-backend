use lettre::{Message, SmtpTransport, Transport};
use rand::Rng;
use redis::Commands;
use crate::db::REDIS_CONN;
use crate::error::Error;

pub fn send_authorize_code_mail(user_id: i32, email_addr: &str) -> Result<(), Error> {
    const CHARSET: &[u8] = b"0123456789";
    const LEN: usize = 6;
    if REDIS_CONN.lock()?.exists(format!("is_send:{}", user_id.to_string()))? {
        return Err(Error::BadRequest(json!("邮件已经发送，请稍后在试")));
    }
    let code: String = (0..LEN)
        .map(|_| {
            let idx = rand::thread_rng().gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    let email = Message::builder()
        .from("no_reply <no_reply@njtumc.moe>".parse().unwrap())
        .to(email_addr.parse()?)
        .subject("njtumc 验证码")
        .body(format!("您的验证码为：{}", code))
        .unwrap();

    let _: () = REDIS_CONN.lock()?.set_ex(format!("is_send:{}", user_id.to_string()), "1", 60)?;
    let _: () = REDIS_CONN.lock()?.set_ex(format!("email:{}:{}", email_addr, user_id.to_string()), &code, 60 * 60 * 90)?;

    let mailer = SmtpTransport::unencrypted_localhost();

    let res = mailer.send(&email);
    println!("{:?}", res);
    Ok(())
}

pub fn check_mail_addr(user_id: i32, email_addr: &str, code: &str) -> Result<(), Error> {
    let c: String = match REDIS_CONN.lock()?.get(format!("email:{}:{}", email_addr, user_id.to_string())) {
        Ok(c) => c,
        Err(_) => return Err(Error::BadRequest(json!("验证码错误，请重新发送")))
    };
    if c != code {
        return Err(Error::BadRequest(json!("验证码错误，请重新发送")));
    }

    Ok(())
}
