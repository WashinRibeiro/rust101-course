use tide::Request;
use calc_nearx_module8::calc_with_1;

#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("Iniciando Server...");

    let mut app = tide::new();

    app.at("/").get(|_| async move { Ok("Hello Tide!") });
    app.at("/sum/:x/:y").get(sum);
    app.at("/sub/:x/:y").get(sub);

    println!("Servidor inicializado!");
    println!("Acesse em: http://0.0.0.0:3003");

    app.listen("0.0.0.0:3003").await?;

    Ok(())
}

async fn sum(req: Request<()>) -> tide::Result {
    let x = req.param("x").unwrap_or("0").parse().unwrap_or(0);
    let y = req.param("y").unwrap_or("0").parse().unwrap_or(0);
    let z = calc_with_1::sum_plus_one(x, y);
    Ok((format!("Resultado: {}", z)).into())
}

async fn sub(req: Request<()>) -> tide::Result {
    let x = req.param("x").unwrap_or("0").parse().unwrap_or(0);
    let y = req.param("y").unwrap_or("0").parse().unwrap_or(0);
    let z = calc_with_1::sub_less_one(x, y);
    Ok((format!("Resultado: {}", z)).into())
}
